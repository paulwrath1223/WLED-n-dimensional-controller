use crate::wled_net::PhysicalWled;



/// A slice of a WLED.
///
/// This can be thought of as a segment, but they are not necessarily controlled by the segments defined on board the WLED.
/// These segments must be part of a single universe
pub struct WledSlice<'slt>{ // lifetime is "Slice Life Time", not slut
    pub wled_ref: &'slt PhysicalWled,
    pub start: u16,
    pub stop: u16,
}


pub struct Universe<'w>{
    constituents: Vec<WledSlice<'w>>
}

