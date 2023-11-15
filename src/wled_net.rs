use wled_json_api_library;
use ddp_rs;
use wled_json_api_library::structures::cfg::cfg_hw::cfg_hw_led::Led as CfgLed;
use wled_json_api_library::errors::WledJsonApiError;

struct PhysicalWled {
    json_con: wled_json_api_library::wled::Wled,
    ddp_con: ddp_rs::connection::DDPConnection,
    segments: Vec<SegmentBounds>
}

pub struct SegmentBounds{
    pub start: u16,
    pub length: u16
}

pub fn get_segment_bounds(cfg_led: CfgLed) -> Result<Vec<SegmentBounds>, WledJsonApiError> {
    let server_info = cfg_led.ins.ok_or(WledJsonApiError::MissingKey)?;
    let mut bound_vec: Vec<SegmentBounds> = Vec::with_capacity(server_info.len());
    for bus in server_info{
        bound_vec.push(SegmentBounds
        {
            start: bus.start.ok_or(WledJsonApiError::MissingKey)?,
            length: bus.len.ok_or(WledJsonApiError::MissingKey)?
        })
    }
    Ok(bound_vec)
}