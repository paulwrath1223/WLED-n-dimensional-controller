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
    segments: Vec<SegmentInfo<'a>>,
    /// the name of the server (found in the WLED UI settings)
    friendly_name: String,
    /// the url to reach the WLED on in case the IP is not static. always append ".local"
    local_url: String,
    /// Local network IP, ipv4 only currently. (same as WLED)
    local_ip: [u8; 4],
}

pub struct SegmentInfo<'b> {
    pub start: u16,
    pub stop: u16,
    pub name: String,
    pub capabilities: LightCapability,
    pub ddp_ref: &'b DDPConnection,
}



impl<'b> PhysicalWled<'b> {
    pub fn get_segment_bounds(&'b mut self) -> Result<(), WledJsonApiError> {
        // update info from server
        self.json_con.get_cfg_from_wled()?;
        self.json_con.get_info_from_wled()?;
        self.json_con.get_state_from_wled()?;


        let config_option: &Option<WledJsonCfg> = &self.json_con.cfg;
        let cfg: &WledJsonCfg = config_option.as_ref().ok_or(WledJsonApiError::MissingKey)?;


        let info_option: &Option<WledJsonInfo> = &self.json_con.info;
        let info: &WledJsonInfo = info_option.as_ref().ok_or(WledJsonApiError::MissingKey)?;

        let state_option: &Option<WledJsonState> = &self.json_con.state;
        let state: &WledJsonState = state_option.as_ref().ok_or(WledJsonApiError::MissingKey)?;


        let info_leds = info.leds.as_ref().ok_or(WledJsonApiError::MissingKey)?;
        let mut segment_capability_vec = info_leds.seglc.as_ref().ok_or(WledJsonApiError::MissingKey)?.iter();

        let segment_vec = state.seg.as_ref().ok_or(WledJsonApiError::MissingKey)?;
        self.segments = Vec::with_capacity(segment_vec.len());
        for segment in segment_vec.iter(){
            let name: String = match &segment.name {
                Some(a) => a.clone(),
                None => {
                    let mut b = String::from("Segment");
                    b.push_str(
                        &*segment.id
                            .ok_or(WledJsonApiError::MissingKey)?
                            .to_string()
                    );
                    b.clone()
                }
            };
            let temp_seg = SegmentInfo
            {
                start: segment.start.as_ref().ok_or(WledJsonApiError::MissingKey)?.clone(),
                stop: segment.stop.as_ref().ok_or(WledJsonApiError::MissingKey)?.clone(),
                name,
                capabilities: segment_capability_vec.next().ok_or(WledJsonApiError::MissingKey)?.clone(),
                ddp_ref: &self.ddp_con,
            };
            self.segments.push(temp_seg)
        }
        Ok(())
    }
}