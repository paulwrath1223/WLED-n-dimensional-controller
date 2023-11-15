use wled_json_api_library;
use ddp_rs;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::Led as CfgLed;
use wled_json_api_library::structures::cfg::cfg_id::Id as CfgId;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::LightCapability;
use wled_json_api_library::errors::WledJsonApiError;
use ddp_rs::connection::DDPConnection;

/// A single WLED server.
pub struct PhysicalWled<'a> {
    /// the connection to the WLED JSON API. Library by me
    json_con: wled_json_api_library::wled::Wled,
    /// the DDP connection. Library by @coral and some minor contributions by myself
    ddp_con: DDPConnection,
    /// information about pre-defined segments
    segments: Vec<SegmentInfo<'a>>,
    /// the name of the server (found in the WLED UI settings)
    friendly_name: String,
    /// the url to reach the WLED on in case the IP is not static. always append ".local"
    local_url: String,
    /// Local network IP, ipv4 only currently. (same as WLED)
    local_ip: [u8; 4],
}

pub struct SegmentInfo<'a> {
    pub start: u16,
    pub length: u16,
    pub name: String,
    pub capabilities: LightCapability,
    pub ddp_ref: &'a DDPConnection
}

pub fn get_segment_bounds<'a>(cfg) -> Result<Vec<SegmentInfo<'a>>, WledJsonApiError> {
    let server_info = cfg_led.ins.ok_or(WledJsonApiError::MissingKey)?;
    let mut bound_vec: Vec<SegmentInfo> = Vec::with_capacity(server_info.len());
    for bus in server_info{
        bound_vec.push(SegmentInfo
        {

        })
    }
    Ok(bound_vec)
}