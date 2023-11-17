use std::net::{IpAddr, SocketAddr, SocketAddrV4};
use std::num::{ParseIntError};
use wled_json_api_library;
use ddp_rs;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::LightCapability;
use wled_json_api_library::wled::{Wled as JsonApi};
use ddp_rs::connection::DDPConnection;
use reqwest::Url;
use wled_json_api_library::structures::info::{Info as WledJsonInfo};
use wled_json_api_library::structures::state::State as WledJsonState;

use wled_json_api_library::errors::WledJsonApiError;
use crate::error::WledControllerError;


/// A single WLED server.
#[derive(Debug)]
pub struct PhysicalWled<'a> {
    /// the connection to the WLED JSON API. Library by me
    pub json_con: JsonApi,
    /// the DDP connection. Library by @coral and some minor contributions by myself
    pub ddp_con: DDPConnection,
    /// information about pre-defined segments
    pub segments: Vec<SegmentInfo<'a>>,
    /// the name of the server (found in the WLED UI settings)
    pub friendly_name: String,
    /// the url to reach the WLED on in case the IP is not static. ends in ".local"
    pub local_url: String,
    /// Local network IP, ipv4 only currently. (same as WLED)
    pub local_ip: [u8; 4],
}

#[derive(Debug)]
pub struct SegmentInfo<'s> {
    pub start: u16,
    pub stop: u16,
    pub name: String,
    pub capabilities: LightCapability,
    pub ddp_ref: &'s DDPConnection,
}



impl<'b> PhysicalWled<'b> {

    pub fn try_from_ip(ip: IpAddr) -> Result<PhysicalWled<'b>, WledControllerError> {
        let mut physical_wled = PhysicalWled::private_try_from_ip(ip)?;
        physical_wled.get_self_segment_bounds()?;
        Ok(physical_wled)
    }


    fn private_try_from_ip(ip: IpAddr) -> Result<PhysicalWled<'b>, WledControllerError> {

        let socketted_ip = SocketAddr::new(ip, 4048);

        let mut url = Url::parse("http://1.1.1.1").map_err(|_| WledControllerError::TempError)?;
        // I know how bad this is

        url.set_ip_host(ip).map_err(|_| WledControllerError::TempError)?;

        let mut json_con = JsonApi::try_from_url(&url)?;

        // time for a dedicated url struct

        let ddp_con: DDPConnection = DDPConnection::try_new
        (
            socketted_ip, // The IP address of the device followed by :4048
            ddp_rs::protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
            // TODO: set pixel config according to information from WLED.
            // This information is present in the JSON API
            ddp_rs::protocol::ID::Default,
            std::net::UdpSocket::bind("0.0.0.0:4048")?
         // can be any unused port on 0.0.0.0, but protocol recommends 4048
        )?;

        json_con.get_info_from_wled()?;

        json_con.get_cfg_from_wled()?;

        let wled_name: String = match &json_con.info{
            None => {return Err(WledControllerError::MissingKey)}
            Some(a) => {
                match &a.name {
                    None => {
                        match &a.mac{
                            None => {return Err(WledControllerError::MissingKey)}
                            Some(c) => {
                                let mut temp = String::from("Unamed WLED with MAC: ");
                                temp.push_str(c);
                                temp
                            }
                        }
                    }
                    Some(b) => {
                        b.clone()
                    }
                }
            }
        };

        let wled_ip_string: [u8; 4] = try_string_to_ipv4
            (
                match &json_con.info
                {
                    None => { return Err(WledControllerError::MissingKey) }
                    Some(a) => {
                        match &a.ip {
                            None => { return Err(WledControllerError::MissingKey) }
                            Some(b) => {
                                b.clone()
                            }
                        }
                    }
                }
            )?;

        let mut wled_url: String = match &json_con.cfg
        {
            None => { return Err(WledControllerError::MissingKey) }
            Some(a) => {
                match &a.id {
                    None => { return Err(WledControllerError::MissingKey) }
                    Some(b) => {
                        match &b.mdns {
                            None => { return Err(WledControllerError::MissingKey) }
                            Some(c) => {
                                c.clone()
                            }
                        }
                    }
                }
            }
        };
        wled_url.push_str(".local");


        let physical_wled = PhysicalWled{
            json_con,
            ddp_con,
            segments: Vec::new(),
            friendly_name: wled_name,
            local_url: wled_url,
            local_ip: wled_ip_string,
        };

        Ok(physical_wled)
    }


    /// Gets all segments from the WLED server and saves them in a format
    fn get_self_segment_bounds(&mut self) -> Result<(), WledControllerError>
    {
        // update info from server
        self.json_con.get_info_from_wled()?;
        self.json_con.get_state_from_wled()?;


        let info_option: &Option<WledJsonInfo> = &self.json_con.info;
        let info: &WledJsonInfo = info_option.as_ref().ok_or(WledControllerError::MissingKey)?;

        let state_option: &Option<WledJsonState> = &self.json_con.state;
        let state: &WledJsonState = state_option.as_ref().ok_or(WledControllerError::MissingKey)?;


        let info_leds = info.leds.as_ref().ok_or(WledControllerError::MissingKey)?;
        let mut segment_capability_vec = info_leds.seglc.as_ref().ok_or(WledControllerError::MissingKey)?.iter();

        let segment_vec = state.seg.as_ref().ok_or(WledControllerError::MissingKey)?;
        self.segments.reserve(segment_vec.len());
        for segment in segment_vec.iter(){
            println!("segment: {:?}", segment);
            let name: String = match &segment.name {
                Some(a) => a.clone(),
                None => {
                    let mut b = String::from("Segment");
                    b.push_str(
                        &segment.id
                            .ok_or(WledControllerError::MissingKey)?
                            .to_string()
                    );
                    b.clone()
                }
            };
            let start = match &segment.start{
                None => { return Err(WledControllerError::MissingKey) }
                Some(a) => {}
            }
            self.segments.push(SegmentInfo
                {
                    start: segment.start.as_ref().ok_or(WledControllerError::MissingKey)?.clone(),
                    stop: segment.stop.as_ref().ok_or(WledControllerError::MissingKey)?.clone(),
                    name,
                    capabilities: segment_capability_vec.next().ok_or(WledControllerError::MissingKey)?.clone(),
                    ddp_ref: &self.ddp_con,
                }
            )
        }
        Ok(())
    }
}

fn try_string_to_ipv4(string_in: String) -> Result<[u8; 4], WledControllerError>{
    let split_str = string_in.split('.');
    let temp = split_str.map(|s| {s.parse::<u8>()}).collect::<Result<Vec<u8>, ParseIntError>>();
    let temp2: [u8; 4] = temp?.try_into().map_err(|_|{WledControllerError::BadIp})?;
    Ok(temp2)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use crate::error::WledControllerError;
    use crate::wled_net::{PhysicalWled, try_string_to_ipv4};

    #[test]
    fn test_try_string_to_ipv4() {

        let ip = try_string_to_ipv4(String::from("192.168.1.40")).unwrap();
        assert_eq!(ip, [192,168,1,40]);

        let ip2 = try_string_to_ipv4(String::from("5.40")).unwrap_err().to_string();
        assert_eq!(ip2, WledControllerError::BadIp.to_string());

        let ip2 = try_string_to_ipv4(String::from("0")).unwrap_err().to_string();
        assert_eq!(ip2, WledControllerError::BadIp.to_string());

    }

    #[test]
    fn test_new_wled() {

        let wled = PhysicalWled::try_from_ip(
            IpAddr::V4(Ipv4Addr::new(192,168,1,40))
        ).unwrap();
        let segs = wled.segments.unwrap();
        println!("segments: {:?}", segs)

    }
}