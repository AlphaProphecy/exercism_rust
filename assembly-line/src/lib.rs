// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut multipler = 221.0;
    if speed > 4 && speed < 9 {
        multipler = multipler * 0.9;
    } else if speed >= 9 {
        multipler = multipler * 0.77;
    }
    return (speed as f64) * multipler
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32
}
