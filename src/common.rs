use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const ANGLE_ORDER: &str = "xyz";

#[wasm_bindgen(js_name = toRadian)]
pub fn to_radian(a: f32) -> f32 {
    return a.to_radians();
}

#[wasm_bindgen(js_name = commonequals)]
pub fn equals(a: f32, b: f32) -> bool {
    let vals = [a, b];
    return (a - b).abs() <= f32::EPSILON * vals.iter().fold(1_f32, |a, &b| a.abs().max(b.abs()));
}

macro_rules! js_round {
    ($x: expr) => {
        if ($x.rem_euclid(1_f32) - 0.5_f32) <= f32::EPSILON {
            $x.floor()
        } else {
            $x.ceil()
        }
    };
}
pub(crate) use js_round;