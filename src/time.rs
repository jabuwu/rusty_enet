use web_time::{SystemTime, UNIX_EPOCH};

const ENET_TIME_OVERFLOW: u32 = 86400000;

pub(crate) fn enet_time_get() -> u32 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        % u32::MAX as u128) as u32
}

pub(crate) fn enet_time_less(a: u32, b: u32) -> bool {
    a.wrapping_sub(b) >= ENET_TIME_OVERFLOW
}

pub(crate) fn enet_time_greater_equal(a: u32, b: u32) -> bool {
    !enet_time_less(a, b)
}

pub(crate) fn enet_time_difference(a: u32, b: u32) -> u32 {
    if a - b >= ENET_TIME_OVERFLOW {
        b.wrapping_sub(a)
    } else {
        a.wrapping_sub(b)
    }
}
