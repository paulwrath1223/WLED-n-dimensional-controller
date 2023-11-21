use std::collections::HashMap;
use crate::wled_net::{RawMacAddress, PhysicalWled};

pub struct Dispatcher{
    constituents: HashMap<PhysicalWled, RawMacAddress>
}

