use wled_json_api_library;
use ddp_rs;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::Led as CfgLed;
use wled_json_api_library::structures::cfg::cfg_id::Id as CfgId;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::LightCapability;
use wled_json_api_library::errors::WledJsonApiError;
use wled_json_api_library::wled::Wled as JsonApi;
use ddp_rs::connection::DDPConnection;
use ddp_rs::error::DDPError;
use wled_json_api_library::structures::cfg::Cfg as WledJsonCfg;
use wled_json_api_library::structures::info::Info as WledJsonInfo;
use wled_json_api_library::structures::state::State as WledJsonState;
use reqwest::Url;
use crate::error::WledControllerError;
use crate::error::WledControllerError::JsonApiError;

/// A single WLED server.
pub struct PhysicalWled<'a> {
    /// the connection to the WLED JSON API. Library by me
    pub json_con: JsonApi,
    /// the DDP connection. Library by @coral and some minor contributions by myself
    pub ddp_con: DDPConnection,
    /// information about pre-defined segments
    pub segments: Vec<SegmentInfo<'a>>,
    /// the name of the server (found in the WLED UI settings)
    pub friendly_name: String,
    /// the url to reach the WLED on in case the IP is not static. always append ".local"
    pub local_url: String,
    /// Local network IP, ipv4 only currently. (same as WLED)
    pub local_ip: [u8; 4],
}

pub struct SegmentInfo<'b> {
    pub start: u16,
    pub stop: u16,
    pub name: String,
    pub capabilities: LightCapability,
    pub ddp_ref: &'b DDPConnection,
}



impl<'b> PhysicalWled<'b> {

    // TODO: merge these functions. or at least make get_segments use just the JSON API and then make a wrapper for reloading.

    pub fn try_from_url(url: Url) -> Result<Self, WledControllerError> {
        let mut url_string = url.to_string();
        url_string.push_str("4048");

        let mut json_con = JsonApi::try_from_url(&url)?;

        let ddp_con = DDPConnection::try_new
        (
            url_string, // The IP address of the device followed by :4048
            ddp_rs::protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
            // TODO: set pixel config according to information from WLED.
            // This information is present in the JSON API
            ddp_rs::protocol::ID::Default,
            std::net::UdpSocket::bind("0.0.0.0:6969")
        .unwrap() // can be any unused port on 0.0.0.0, but protocol recommends 4048
        )?;

        let segments = PhysicalWled::get_segment_bounds(json_con, &ddp_con)?;

        Ok(PhysicalWled{
            json_con,
            ddp_con,
            segments,
            friendly_name: "".to_string(),
            local_url: "".to_string(),
            local_ip: [192,168,1,40], // TODO Obviously not all WLEDs are here, just temporary
        })
    }


    /// Gets all segments from the WLED server and saves them in a format
    fn get_segment_bounds(mut json_con: JsonApi, ddp_connection: &'b DDPConnection)
        -> Result<Vec<SegmentInfo<'b>>, WledControllerError> {
        // update info from server
        json_con.get_info_from_wled()?;
        json_con.get_state_from_wled()?;


        let info_option: &Option<WledJsonInfo> = &json_con.info;
        let info: &WledJsonInfo = info_option.as_ref().ok_or(WledControllerError::MissingKey)?;

        let state_option: &Option<WledJsonState> = &json_con.state;
        let state: &WledJsonState = state_option.as_ref().ok_or(WledControllerError::MissingKey)?;


        let info_leds = info.leds.as_ref().ok_or(WledControllerError::MissingKey)?;
        let mut segment_capability_vec = info_leds.seglc.as_ref().ok_or(WledControllerError::MissingKey)?.iter();

        let segment_vec = state.seg.as_ref().ok_or(WledControllerError::MissingKey)?;
        let mut segments: Vec<SegmentInfo> = Vec::with_capacity(segment_vec.len());
        for segment in segment_vec.iter(){
            let name: String = match &segment.name {
                Some(a) => a.clone(),
                None => {
                    let mut b = String::from("Segment");
                    b.push_str(
                        &*segment.id
                            .ok_or(WledControllerError::MissingKey)?
                            .to_string()
                    );
                    b.clone()
                }
            };
            let temp_seg = SegmentInfo
            {
                start: segment.start.as_ref().ok_or(WledControllerError::MissingKey)?.clone(),
                stop: segment.stop.as_ref().ok_or(WledControllerError::MissingKey)?.clone(),
                name,
                capabilities: segment_capability_vec.next().ok_or(WledControllerError::MissingKey)?.clone(),
                ddp_ref: ddp_connection,
            };
            segments.push(temp_seg)
        }
        Ok(segments)
    }
}