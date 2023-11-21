use std::net::{IpAddr, SocketAddr};
use std::num::{ParseIntError};
use wled_json_api_library;
use ddp_rs;
use wled_json_api_library::wled::{Wled as JsonApi};
use ddp_rs::connection::DDPConnection;
use reqwest::Url;
use wled_json_api_library::structures::info::{Info as WledJsonInfo};
use wled_json_api_library::structures::cfg::Cfg as WledJsonConfig;
use wled_json_api_library::structures::state::State as WledJsonState;

use crate::error::WledControllerError;
use crate::wled_color::{WledColor, WledColorType};

/// Type alias for clarity
pub type RawMacAddress = [u8; 6];


/// Type alias for clarity. WLED only supports IpV4 and therefore same for me
pub type RawIpAddress = [u8; 4];


/// A single WLED server.
#[derive(Debug)]
pub struct PhysicalWled {
    /// the connection to the WLED JSON API. Library by me
    pub json_con: JsonApi,
    /// the DDP connection. Library by @coral and some minor contributions by myself
    pub ddp_con: DDPConnection,
    /// information about pre-defined segments
    pub segments: Vec<CanonSegmentInfo>,
    /// the name of the server (found in the WLED UI settings)
    pub friendly_name: String,
    /// the url to reach the WLED on in case the IP is not static. ends in ".local"
    pub local_url: String,
    /// Local network IP, ipv4 only currently. (same as WLED)
    pub local_ip: RawIpAddress,
    /// mac address of the WLED. this is used as a reference and unique identifier.
    pub mac_address: RawMacAddress,
    /// A buffer for colors before they get sent.
    pub buffer: Vec<WledColor> // this does not need to be a vec.
    // It will have a constant length determined before it's created,
    // but I cant figure out how to combat 'sized' issues.
}

/// A segment in a WLED. These are segments that are defined on the WLED and not by this program.
///
/// Only used to make grouping universes easier if you wish to reuse the same bounds.
///
/// This object should be read-only. You can change it, but the changes won't effect the Wled
#[derive(Debug)]
pub struct CanonSegmentInfo {
    /// The starting index of the segment
    pub start: u16,

    /// The stop index of the segment
    pub stop: u16,

    /// The name of the segment
    pub name: String,

    /// Capabilities of the segment.
    ///
    /// As of now this is not actually segment specific,
    /// but hopefully WLED fixes this and it will become more useful later
    pub capabilities: WledColorType,
}


impl CanonSegmentInfo {
    // /// returns true if the segment supports RGB input
    // ///
    // /// (actually because of WLED it will be true if any busses on the WLED have RGB, but theres nothing I can do about that)
    // pub fn has_rgb(&self) -> bool{
    //     self.capabilities & SegmentLightCapability::SEG_CAPABILITY_RGB as u8 != 0
    // }
    //
    // /// returns true if the segment supports W (white channel) input
    // ///
    // /// (actually because of WLED it will be true if any busses on the WLED have a white channel, but theres nothing I can do about that)
    // pub fn has_w(&self) -> bool{
    //     self.capabilities & SegmentLightCapability::SEG_CAPABILITY_W as u8 != 0
    // }
    //
    // /// returns true if the segment supports CCT
    // ///
    // /// (actually because of WLED it will be true if any busses on the WLED have CCT, but theres nothing I can do about that)
    // pub fn has_cct(&self) -> bool{
    //     self.capabilities & SegmentLightCapability::SEG_CAPABILITY_CCT as u8 != 0
    // }
}



impl PhysicalWled {
    /// Just a wrapper for two private implementations: ```private_try_from_ip``` and ```get_self_segment_bounds```
    pub fn try_from_ip(ip: IpAddr) -> Result<PhysicalWled, WledControllerError> {
        let mut physical_wled = PhysicalWled::private_try_from_ip(ip)?;
        physical_wled.get_self_segment_bounds()?;
        Ok(physical_wled)
    }


    fn private_try_from_ip(ip: IpAddr) -> Result<PhysicalWled, WledControllerError> {

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

        let info_ref: &WledJsonInfo = json_con.info.as_ref().ok_or(WledControllerError::MissingKey)?;

        let wled_mac_string: String = info_ref.mac.as_ref().ok_or(WledControllerError::MissingKey)?.clone();

        let wled_name: String = match info_ref.name.as_ref(){
            None => {
                let mut temp = String::from("Unamed WLED with MAC: ");
                temp.push_str(&wled_mac_string);
                temp
            }
            Some(b) => {
                b.clone()
            }
        };

        let wled_mac: [u8; 6] = try_string_to_mac(wled_mac_string)?;

        let wled_ip_string: RawIpAddress = try_string_to_ipv4(
            info_ref.ip.as_ref().ok_or(WledControllerError::MissingKey)?.clone()
        )?;

        let cfg_ref: &WledJsonConfig = json_con.cfg.as_ref().ok_or(WledControllerError::MissingKey)?;

        let mut wled_url: String = cfg_ref.id.as_ref().ok_or(WledControllerError::MissingKey)?
            .mdns.as_ref().ok_or(WledControllerError::MissingKey)?.clone();
        wled_url.push_str(".local");

        let total_wled_length = cfg_ref.hw.as_ref().ok_or(WledControllerError::MissingKey)?
            .led.as_ref().ok_or(WledControllerError::MissingKey)?
            .total.as_ref().ok_or(WledControllerError::MissingKey)?.clone();


        let physical_wled = PhysicalWled{
            json_con,
            ddp_con,
            segments: Vec::new(),
            friendly_name: wled_name,
            local_url: wled_url,
            local_ip: wled_ip_string,
            mac_address: wled_mac,
            buffer: Vec::with_capacity(total_wled_length as usize),
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
                    b
                }
            };
            let start = &segment.start.ok_or(WledControllerError::MissingKey)?;
            let stop = &segment.stop.ok_or(WledControllerError::MissingKey)?;
            let capabilities = WledColorType::try_new_from_json_type(
                segment_capability_vec.next().ok_or(WledControllerError::MissingKey)?.clone()
            )?;

            let temp_seg = CanonSegmentInfo {
                start: start.clone(),
                stop: stop.clone(),
                name,
                capabilities,
            };

            self.segments.push(temp_seg)
        }
        Ok(())
    }
}

fn try_string_to_ipv4(string_in: String) -> Result<RawIpAddress, WledControllerError>{
    let split_str = string_in.split('.');
    let temp = split_str.map(|s| {s.parse::<u8>()}).collect::<Result<Vec<u8>, ParseIntError>>();
    let temp2: RawIpAddress = temp?.try_into().map_err(|_|{WledControllerError::BadIp(string_in)})?;
    Ok(temp2)
}

fn try_string_to_mac(string_in: String) -> Result<RawMacAddress, WledControllerError>{

    let str_as_bytes = string_in.as_bytes();
    if str_as_bytes.len() != 12{ return Err(WledControllerError::BadMac(string_in))}
    let split_str = str_as_bytes.chunks(2)
        .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) });
    let temp = split_str.map(|s| { u8::from_str_radix(s, 16) }).collect::<Result<Vec<u8>, ParseIntError>>();
    Ok(temp?.try_into().map_err(|_|{WledControllerError::BadMac(string_in)})?)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use crate::error::WledControllerError;
    use crate::wled_net::{PhysicalWled, try_string_to_ipv4, try_string_to_mac};

    #[test]
    fn test_try_string_to_ipv4() {

        let ip = try_string_to_ipv4(String::from("192.168.1.40")).unwrap();
        assert_eq!(ip, [192,168,1,40]);

        let ip2 = try_string_to_ipv4(String::from("5.40")).unwrap_err().to_string();
        assert_eq!(ip2, WledControllerError::BadIp("5.40".to_string()).to_string());

        let ip2 = try_string_to_ipv4(String::from("0")).unwrap_err().to_string();
        assert_eq!(ip2, WledControllerError::BadIp("0".to_string()).to_string());

    }

    #[test]
    fn test_try_string_to_mac() {

        let mac = try_string_to_mac(String::from("a00ac9225a10")).unwrap();
        assert_eq!(mac, [160,10,201,34,90,16]);

        let mac2 = try_string_to_mac(String::from("a00ac922510")).unwrap_err().to_string();
        assert_eq!(mac2, WledControllerError::BadMac("a00ac922510".to_string()).to_string());

        let mac3 = try_string_to_mac(String::from("a00ac9h22510")).unwrap_err().to_string();
        assert_eq!(mac3, "error parsing integer from a string: invalid digit found in string".to_string());

    }

    #[test]
    fn test_new_wled() {

        let wled = PhysicalWled::try_from_ip(
            IpAddr::V4(Ipv4Addr::new(192,168,1,40))
        ).unwrap();
        let segs = wled.segments;
        println!("segments: {:?}", segs)

    }
}