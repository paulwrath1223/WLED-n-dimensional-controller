use wled_json_api_library;
use ddp_rs;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::Led as CfgLed;
use wled_json_api_library::structures::cfg::cfg_id::Id as CfgId;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::LightCapability;
use wled_json_api_library::errors::WledJsonApiError;
use wled_json_api_library::wled::Wled as JsonApi;
use ddp_rs::connection::DDPConnection;
use wled_json_api_library::structures::cfg::Cfg as WledJsonCfg;
use wled_json_api_library::structures::info::Info as WledJsonInfo;
use wled_json_api_library::structures::state::State as WledJsonState;

/// A single WLED server.
pub struct PhysicalWled<'a> {
    /// the connection to the WLED JSON API. Library by me
    json_con: JsonApi,
    /// the DDP connection. Library by @coral and some minor contributions by myself
    ddp_con: DDPConnection,
    /// information about pre-defined segments
    segments: Option<Vec<&'a SegmentInfo<'a>>>,
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
    pub ddp_ref: &'a DDPConnection,
    pub buffer: [u8]
}



impl<'a> PhysicalWled<'a> {
    pub fn get_segment_bounds(& mut self) -> Result<(), WledJsonApiError> {
        // update info from server
        &self.json_con.get_cfg_from_wled()?;
        let config_option: &Option<WledJsonCfg> = &self.json_con.cfg;

        let cfg: WledJsonCfg = config_option.ok_or(WledJsonApiError::MissingKey)?;

        &self.json_con.get_info_from_wled()?;
        let info_option: &Option<WledJsonInfo> = &self.json_con.info;

        let info: WledJsonInfo = info_option.ok_or(WledJsonApiError::MissingKey)?;

        &self.json_con.get_info_from_wled()?;
        let state_option: &Option<WledJsonState> = &self.json_con.state;

        let state: WledJsonState = state_option.ok_or(WledJsonApiError::MissingKey)?;

        let server_info = sel.ins.ok_or(WledJsonApiError::MissingKey)?;
        let temp_vec: Vec<&SegmentInfo> = Vec::with_capacity(server_info.len());
        for bus in server_info{
            temp_vec.push(&SegmentInfo
            {
                start: 0,
                length: 0,
                name: String::from("wled"),
                capabilities: LightCapability::TYPE_NONE,
                ddp_ref: &(),
                buffer: [],
            })
        }
        self.segments = Some(temp_vec);

        Ok(())
    }
}