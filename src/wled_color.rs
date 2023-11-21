use crate::error::WledControllerError;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum WledColorType {
    RGB = 0u8,
    RGBW = 1u8,
    W = 2u8,
}

impl WledColorType {
    /// This takes the JSON value returned by the server and returns a useful value
    ///
    /// json_type: A bitmap with masks defined in ```SegmentLightCapability```
    pub fn try_new_from_json_type(json_type: u8) -> Result<WledColorType, WledControllerError>{
        match json_type % 4{
            0 => {Err(WledControllerError::InvalidCapability(0))}
            1 => {Ok(WledColorType::RGB)}
            2 => {Ok(WledColorType::W)}
            3 => {Ok(WledColorType::RGBW)}
            e => {Err(WledControllerError::InvalidCapability(e))} // this should never be
                                                    // reached but compiler gets what compiler wants
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum WledColor {
    RGB([u8; 3]),
    RGBW([u8; 4]),
    W(u8),
}

impl Default for WledColor{
    fn default() -> WledColor {
        WledColor::RGB([0u8, 0u8, 0u8])
    }
}

impl WledColor{
    pub fn default_of_type(color_type: WledColorType) -> WledColor{
        match color_type{
            WledColorType::RGB => {WledColor::RGB([0u8, 0u8, 0u8])}
            WledColorType::RGBW => {WledColor::RGBW([0u8, 0u8, 0u8, 0u8])}
            WledColorType::W => {WledColor::W(0u8)}
        }
    }
}