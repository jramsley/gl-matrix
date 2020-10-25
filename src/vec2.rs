use super::common;
use js_sys::Float32Array;
use js_sys::JsString;
use rand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = vec2_create)]
pub fn create() -> Float32Array {
    return Float32Array::new_with_length(2);
}

#[wasm_bindgen(js_name = vec2_clone)]
pub fn clone(a: &Float32Array) -> Float32Array {
    let out = create();
    out.set_index(0, a.get_index(0));
    out.set_index(1, a.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_fromValues)]
pub fn from_values(x: f32, y: f32) -> Float32Array {
    let out = create();
    out.set_index(0, x);
    out.set_index(1, y);
    return out;
}

#[wasm_bindgen(js_name = vec2_copy)]
pub fn copy(out: Float32Array, a: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0));
    out.set_index(1, a.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_set)]
pub fn set(out: Float32Array, x: f32, y: f32) -> Float32Array {
    out.set_index(0, x);
    out.set_index(1, y);
    return out;
}

#[wasm_bindgen(js_name = vec2_add)]
pub fn add(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0) + b.get_index(0));
    out.set_index(1, a.get_index(1) + b.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_subtract)]
pub fn subtract(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0) - b.get_index(0));
    out.set_index(1, a.get_index(1) - b.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_multiply)]
pub fn multiply(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0) * b.get_index(0));
    out.set_index(1, a.get_index(1) * b.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_divide)]
pub fn divide(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0) / b.get_index(0));
    out.set_index(1, a.get_index(1) / b.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_ceil)]
pub fn ceil(out: Float32Array, a: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0).ceil());
    out.set_index(1, a.get_index(1).ceil());
    return out;
}

#[wasm_bindgen(js_name = vec2_floor)]
pub fn floor(out: Float32Array, a: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0).floor());
    out.set_index(1, a.get_index(1).floor());
    return out;
}

#[wasm_bindgen(js_name = vec2_min)]
pub fn min(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0).min(b.get_index(0)));
    out.set_index(1, a.get_index(1).min(b.get_index(1)));
    return out;
}

#[wasm_bindgen(js_name = vec2_max)]
pub fn max(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, a.get_index(0).max(b.get_index(0)));
    out.set_index(1, a.get_index(1).max(b.get_index(1)));
    return out;
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

#[wasm_bindgen(js_name = vec2_round)]
pub fn js_round(out: Float32Array, a: &Float32Array) -> Float32Array {
    out.set_index(0, js_round!(a.get_index(0)));
    out.set_index(1, js_round!(a.get_index(1)));
    return out;
}

#[wasm_bindgen(js_name = vec2_scale)]
pub fn scale(out: Float32Array, a: &Float32Array, b: f32) -> Float32Array {
    out.set_index(0, a.get_index(0) * b);
    out.set_index(1, a.get_index(1) * b);
    return out;
}

#[wasm_bindgen(js_name = vec2_scaleAndAdd)]
pub fn scale_and_add(
    out: Float32Array,
    a: &Float32Array,
    b: &Float32Array,
    scale: f32,
) -> Float32Array {
    out.set_index(0, a.get_index(0) + b.get_index(0) * scale);
    out.set_index(1, a.get_index(1) + b.get_index(1) * scale);
    return out;
}

#[wasm_bindgen(js_name = vec2_distance)]
pub fn distance(a: &Float32Array, b: &Float32Array) -> f32 {
    let x: f32 = b.get_index(0) - a.get_index(0);
    let y: f32 = b.get_index(1) - a.get_index(1);
    return x.hypot(y);
}

#[wasm_bindgen(js_name = vec2_squaredDistance)]
pub fn squared_distance(a: &Float32Array, b: &Float32Array) -> f32 {
    let x: f32 = b.get_index(0) - a.get_index(0);
    let y: f32 = b.get_index(1) - a.get_index(1);
    return x.powi(2) + y.powi(2);
}

#[wasm_bindgen(js_name = vec2_length)]
pub fn length(a: &Float32Array) -> f32 {
    return a.get_index(0).hypot(a.get_index(1));
}

#[wasm_bindgen(js_name = vec2_squaredLength)]
pub fn squared_length(a: &Float32Array) -> f32 {
    return a.get_index(0) * a.get_index(0) + a.get_index(1) * a.get_index(1);
}

#[wasm_bindgen(js_name = vec2_negate)]
pub fn negate(out: Float32Array, a: &Float32Array) -> Float32Array {
    out.set_index(0, -a.get_index(0));
    out.set_index(1, -a.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_inverse)]
pub fn inverse(out: Float32Array, a: &Float32Array) -> Float32Array {
    out.set_index(0, 1_f32 / a.get_index(0));
    out.set_index(1, 1_f32 / a.get_index(1));
    return out;
}

#[wasm_bindgen(js_name = vec2_normalize)]
pub fn normalize(out: Float32Array, a: &Float32Array) -> Float32Array {
    let len: f32 = length(&a);
    if len <= f32::EPSILON {
        out.set_index(0, a.get_index(0) / len);
        out.set_index(1, a.get_index(1) / len);
    } else {
        out.set_index(0, 0_f32);
        out.set_index(1, 0_f32);
    }
    return out;
}

#[wasm_bindgen(js_name = vec2_dot)]
pub fn dot(a: &Float32Array, b: &Float32Array) -> f32 {
    return a.get_index(0) * b.get_index(0) + a.get_index(1) * b.get_index(1);
}

#[wasm_bindgen(js_name = vec2_cross)]
pub fn cross(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
    out.set_index(0, 0_f32);
    out.set_index(1, 0_f32);
    out.set_index(
        2,
        a.get_index(0) * b.get_index(1) - a.get_index(1) * b.get_index(0),
    );
    return out;
}

#[wasm_bindgen(js_name = vec2_lerp)]
pub fn lerp(out: Float32Array, a: &Float32Array, b: &Float32Array, t: f32) -> Float32Array {
    out.set_index(0, a.get_index(0) + t * (b.get_index(0) - a.get_index(0)));
    out.set_index(1, a.get_index(1) + t * (b.get_index(1) - a.get_index(1)));
    return out;
}

#[wasm_bindgen(js_name = vec2_random)]
pub fn js_random(out: Float32Array, scale: f32) -> Float32Array {
    let r: f32 = rand::random::<f32>() * 2.0_f32 * std::f32::consts::PI;
    out.set_index(0, scale * r.cos());
    out.set_index(0, scale * r.sin());
    return out;
}

#[wasm_bindgen(js_name = vec2_transformMat2)]
pub fn transform_mat_2(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
    out.set_index(
        0,
        m.get_index(0) * a.get_index(0) + m.get_index(2) * a.get_index(1),
    );
    out.set_index(
        1,
        m.get_index(1) * a.get_index(0) + m.get_index(3) * a.get_index(1),
    );
    return out;
}

#[wasm_bindgen(js_name = vec2_transformMat2d)]
pub fn transform_mat_2d(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
    out.set_index(
        0,
        m.get_index(0) + m.get_index(2) * a.get_index(0) + m.get_index(4) * a.get_index(1),
    );
    out.set_index(
        1,
        m.get_index(1) + m.get_index(3) * a.get_index(0) + m.get_index(5) * a.get_index(1),
    );
    return out;
}

#[wasm_bindgen(js_name = vec2_transformMat3)]
pub fn transform_mat_3(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
    out.set_index(
        0,
        m.get_index(6) + m.get_index(0) * a.get_index(0) + m.get_index(3) * a.get_index(1),
    );
    out.set_index(
        1,
        m.get_index(7) + m.get_index(1) * a.get_index(0) + m.get_index(4) * a.get_index(1),
    );
    return out;
}

#[wasm_bindgen(js_name = vec2_transformMat4)]
pub fn transform_mat_4(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
    out.set_index(
        0,
        m.get_index(12) + m.get_index(0) * a.get_index(0) + m.get_index(12) * a.get_index(1),
    );
    out.set_index(
        1,
        m.get_index(13) + m.get_index(1) * a.get_index(0) + m.get_index(13) * a.get_index(1),
    );
    return out;
}

#[wasm_bindgen(js_name = vec2_rotate)]
pub fn rotate(out: Float32Array, a: &Float32Array, b: &Float32Array, rad: f32) -> Float32Array {
    let p = subtract(Float32Array::new_with_length(2), a, b);
    out.set_index(
        0,
        p.get_index(0) * rad.cos() - p.get_index(1) * rad.sin() + b.get_index(0),
    );
    out.set_index(
        1,
        p.get_index(0) * rad.sin() + p.get_index(1) * rad.cos() + b.get_index(1),
    );
    return out;
}

#[wasm_bindgen(js_name = vec2_angle)]
pub fn angle(a: &Float32Array, b: &Float32Array) -> f32 {
    let len_a = length(a);
    let len_b = length(b);
    if len_a == 0_f32 || len_b == 0_f32 {
        return 0_f32.acos();
    } else {
        return dot(a, b) / (len_a * len_b);
    }
}

#[wasm_bindgen(js_name = vec2_zero)]
pub fn zero(out: Float32Array) -> Float32Array {
    out.set_index(0, 0_f32);
    out.set_index(1, 0_f32);
    return out;
}

#[wasm_bindgen(js_name = vec2_str)]
pub fn str(out: Float32Array) -> JsString {
    return JsString::from(format!("vec2({}, {})", out.get_index(0), out.get_index(1)));
}

#[wasm_bindgen(js_name = vec2_exactEquals)]
pub fn exact_equals(a: &Float32Array, b: &Float32Array) -> bool {
    return (a.get_index(0) == b.get_index(0)) && (a.get_index(1) == b.get_index(1));
}

#[wasm_bindgen(js_name = vec2_equals)]
pub fn equals(a: &Float32Array, b: &Float32Array) -> bool {
    return common::equals(a.get_index(0), b.get_index(0))
        && common::equals(a.get_index(1), b.get_index(1));
}
