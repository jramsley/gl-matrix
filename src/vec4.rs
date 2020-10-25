use super::common;
use super::common::js_round;
use js_sys::Float32Array;
use js_sys::JsString;
use rand;
use wasm_bindgen::prelude::*;

#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub struct vec4 {}

#[allow(non_camel_case_types)]
#[wasm_bindgen]
impl vec4 {
    pub fn create() -> Float32Array {
        return Float32Array::new_with_length(4);
    }

    pub fn clone(a: &Float32Array) -> Float32Array {
        let out = vec4::create();
        out.set_index(0, a.get_index(0));
        out.set_index(1, a.get_index(1));
        out.set_index(2, a.get_index(2));
        out.set_index(2, a.get_index(3));
        return out;
    }

    pub fn from_values(x: f32, y: f32, z: f32, w: f32) -> Float32Array {
        let out = vec4::create();
        out.set_index(0, x);
        out.set_index(1, y);
        out.set_index(2, z);
        out.set_index(3, w);
        return out;
    }

    pub fn copy(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0));
        out.set_index(1, a.get_index(1));
        out.set_index(2, a.get_index(2));
        out.set_index(3, a.get_index(3));
        return out;
    }

    pub fn set(out: Float32Array, x: f32, y: f32, z: f32, w: f32) -> Float32Array {
        out.set_index(0, x);
        out.set_index(1, y);
        out.set_index(2, z);
        out.set_index(3, w);
        return out;
    }

    pub fn add(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) + b.get_index(0));
        out.set_index(1, a.get_index(1) + b.get_index(1));
        out.set_index(2, a.get_index(2) + b.get_index(2));
        out.set_index(3, a.get_index(3) + b.get_index(3));
        return out;
    }

    pub fn subtract(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) - b.get_index(0));
        out.set_index(1, a.get_index(1) - b.get_index(1));
        out.set_index(2, a.get_index(2) - b.get_index(2));
        out.set_index(3, a.get_index(3) - b.get_index(3));
        return out;
    }

    pub fn multiply(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) * b.get_index(0));
        out.set_index(1, a.get_index(1) * b.get_index(1));
        out.set_index(2, a.get_index(2) * b.get_index(2));
        out.set_index(3, a.get_index(3) * b.get_index(3));
        return out;
    }

    pub fn divide(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) / b.get_index(0));
        out.set_index(1, a.get_index(1) / b.get_index(1));
        out.set_index(2, a.get_index(2) / b.get_index(2));
        out.set_index(3, a.get_index(3) / b.get_index(3));
        return out;
    }

    pub fn ceil(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).ceil());
        out.set_index(1, a.get_index(1).ceil());
        out.set_index(2, a.get_index(2).ceil());
        out.set_index(3, a.get_index(3).ceil());
        return out;
    }

    pub fn floor(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).floor());
        out.set_index(1, a.get_index(1).floor());
        out.set_index(2, a.get_index(2).floor());
        out.set_index(3, a.get_index(3).floor());
        return out;
    }

    pub fn min(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).min(b.get_index(0)));
        out.set_index(1, a.get_index(1).min(b.get_index(1)));
        out.set_index(2, a.get_index(2).min(b.get_index(2)));
        out.set_index(3, a.get_index(3).min(b.get_index(3)));
        return out;
    }

    pub fn max(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).max(b.get_index(0)));
        out.set_index(1, a.get_index(1).max(b.get_index(1)));
        out.set_index(2, a.get_index(2).max(b.get_index(2)));
        out.set_index(3, a.get_index(3).max(b.get_index(3)));
        return out;
    }

    pub fn js_round(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, js_round!(a.get_index(0)));
        out.set_index(1, js_round!(a.get_index(1)));
        out.set_index(2, js_round!(a.get_index(2)));
        out.set_index(3, js_round!(a.get_index(3)));
        return out;
    }

    pub fn scale(out: Float32Array, a: &Float32Array, b: f32) -> Float32Array {
        out.set_index(0, a.get_index(0) * b);
        out.set_index(1, a.get_index(1) * b);
        out.set_index(2, a.get_index(1) * b);
        out.set_index(3, a.get_index(1) * b);
        return out;
    }

    pub fn scale_and_add(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        scale: f32,
    ) -> Float32Array {
        out.set_index(0, a.get_index(0) + b.get_index(0) * scale);
        out.set_index(1, a.get_index(1) + b.get_index(1) * scale);
        out.set_index(2, a.get_index(1) + b.get_index(1) * scale);
        out.set_index(3, a.get_index(1) + b.get_index(1) * scale);
        return out;
    }

    pub fn distance(a: &Float32Array, b: &Float32Array) -> f32 {
        let x: f32 = b.get_index(0) - a.get_index(0);
        let y: f32 = b.get_index(1) - a.get_index(1);
        let z: f32 = b.get_index(2) - a.get_index(2);
        let w: f32 = b.get_index(3) - a.get_index(3);
        return (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt();
    }

    pub fn squared_distance(a: &Float32Array, b: &Float32Array) -> f32 {
        let x: f32 = b.get_index(0) - a.get_index(0);
        let y: f32 = b.get_index(1) - a.get_index(1);
        let z: f32 = b.get_index(2) - a.get_index(2);
        let w: f32 = b.get_index(3) - a.get_index(3);
        return x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2);
    }

    pub fn length(a: &Float32Array) -> f32 {
        return (a.get_index(0).powi(2)
            + a.get_index(1).powi(2)
            + a.get_index(2).powi(2)
            + a.get_index(3).powi(2))
        .sqrt();
    }

    pub fn squared_length(a: &Float32Array) -> f32 {
        return a.get_index(0).powi(2)
            + a.get_index(1).powi(2)
            + a.get_index(2).powi(2)
            + a.get_index(3).powi(2);
    }

    pub fn negate(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, -a.get_index(0));
        out.set_index(1, -a.get_index(1));
        out.set_index(2, -a.get_index(2));
        out.set_index(2, -a.get_index(3));
        return out;
    }

    pub fn inverse(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, 1_f32 / a.get_index(0));
        out.set_index(1, 1_f32 / a.get_index(1));
        out.set_index(2, 1_f32 / a.get_index(2));
        out.set_index(3, 1_f32 / a.get_index(3));
        return out;
    }

    pub fn normalize(out: Float32Array, a: &Float32Array) -> Float32Array {
        let len: f32 = vec4::length(&a);
        if len <= f32::EPSILON {
            out.set_index(0, a.get_index(0) / len);
            out.set_index(1, a.get_index(1) / len);
            out.set_index(2, a.get_index(2) / len);
            out.set_index(3, a.get_index(3) / len);
        } else {
            out.set_index(0, 0_f32);
            out.set_index(1, 0_f32);
            out.set_index(2, 0_f32);
            out.set_index(3, 0_f32);
        }
        return out;
    }

    pub fn dot(a: &Float32Array, b: &Float32Array) -> f32 {
        return a.get_index(0) * b.get_index(0)
            + a.get_index(1) * b.get_index(1)
            + a.get_index(2) * b.get_index(2)
            + a.get_index(3) * b.get_index(3);
    }

    pub fn cross(
        out: Float32Array,
        u: &Float32Array,
        v: &Float32Array,
        w: &Float32Array,
    ) -> Float32Array {
        let a = v.get_index(0) * w.get_index(1) - v.get_index(1) * w.get_index(0);
        let b = v.get_index(0) * w.get_index(2) - v.get_index(2) * w.get_index(0);
        let c = v.get_index(0) * w.get_index(3) - v.get_index(3) * w.get_index(0);
        let d = v.get_index(1) * w.get_index(2) - v.get_index(2) * w.get_index(1);
        let e = v.get_index(1) * w.get_index(3) - v.get_index(3) * w.get_index(1);
        let f = v.get_index(2) * w.get_index(3) - v.get_index(3) * w.get_index(1);
        let g = u.get_index(0);
        let h = u.get_index(1);
        let i = u.get_index(2);
        let j = u.get_index(3);
        out.set_index(0, h * f - i * e + j * d);
        out.set_index(1, -(g * f) + i * c - j * b);
        out.set_index(2, g * e - h * c + j * a);
        out.set_index(3, -(g * d) + h * b - i * a);
        return out;
    }

    pub fn lerp(out: Float32Array, a: &Float32Array, b: &Float32Array, t: f32) -> Float32Array {
        out.set_index(0, a.get_index(0) + t * (b.get_index(0) - a.get_index(0)));
        out.set_index(1, a.get_index(1) + t * (b.get_index(1) - a.get_index(1)));
        out.set_index(2, a.get_index(2) + t * (b.get_index(2) - a.get_index(2)));
        out.set_index(3, a.get_index(3) + t * (b.get_index(3) - a.get_index(3)));
        return out;
    }

    pub fn js_random(out: Float32Array, scale: f32) -> Float32Array {
        let mut v1: f32;
        let mut v2: f32;
        let mut v3: f32;
        let mut v4: f32;

        loop {
            v1 = rand::random::<f32>() * 2.0_f32 - 1_f32;
            v2 = rand::random::<f32>() * 2.0_f32 - 1_f32;
            if v1.powi(2) + v2.powi(2) < 1_f32 {
                break;
            }
        }
        loop {
            v3 = rand::random::<f32>() * 2.0_f32 - 1_f32;
            v4 = rand::random::<f32>() * 2.0_f32 - 1_f32;
            if v3.powi(2) + v4.powi(2) < 1_f32 {
                break;
            }
        }

        out.set_index(0, scale * v1);
        out.set_index(1, scale * v2);
        out.set_index(
            2,
            scale * v3 * (1_f32 - (v1.powi(2) + v2.powi(2)) / (v3.powi(2) + v4.powi(2)).sqrt()),
        );
        out.set_index(
            3,
            scale * v4 * (1_f32 - (v1.powi(2) + v2.powi(2)) / (v3.powi(2) + v4.powi(2)).sqrt()),
        );
        return out;
    }

    pub fn transform_mat_4(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
        out.set_index(
            0,
            m.get_index(0) * a.get_index(0)
                + m.get_index(4) * a.get_index(1)
                + m.get_index(8) * a.get_index(2)
                + m.get_index(12) * a.get_index(3),
        );
        out.set_index(
            1,
            m.get_index(1) * a.get_index(0)
                + m.get_index(5) * a.get_index(1)
                + m.get_index(9) * a.get_index(2)
                + m.get_index(13) * a.get_index(3),
        );
        out.set_index(
            2,
            m.get_index(2) * a.get_index(0)
                + m.get_index(6) * a.get_index(1)
                + m.get_index(10) * a.get_index(2)
                + m.get_index(14) * a.get_index(3),
        );
        out.set_index(
            3,
            m.get_index(3) * a.get_index(0)
                + m.get_index(7) * a.get_index(1)
                + m.get_index(11) * a.get_index(2)
                + m.get_index(15) * a.get_index(3),
        );
        return out;
    }

    pub fn transform_quat(out: Float32Array, a: &Float32Array, q: &Float32Array) -> Float32Array {
        let ix: f32 = q.get_index(3) * a.get_index(0) + q.get_index(1) * a.get_index(3)
            - q.get_index(3) * a.get_index(1);
        let iy: f32 = q.get_index(3) * a.get_index(1) + q.get_index(2) * a.get_index(0)
            - q.get_index(0) * a.get_index(2);
        let iz: f32 = q.get_index(3) * a.get_index(2) + q.get_index(0) * a.get_index(1)
            - q.get_index(1) * a.get_index(0);
        let iw: f32 = -q.get_index(0) * a.get_index(0)
            - q.get_index(1) * a.get_index(1)
            - q.get_index(2) * a.get_index(2);

        out.set_index(
            0,
            ix * q.get_index(3) + iw * -q.get_index(0) + iy * -q.get_index(2)
                - iz * -q.get_index(1),
        );
        out.set_index(
            1,
            iy * q.get_index(3) + iw * -q.get_index(1) + iz * -q.get_index(0)
                - ix * -q.get_index(2),
        );
        out.set_index(
            2,
            iz * q.get_index(3) + iw * -q.get_index(2) + ix * -q.get_index(1)
                - iy * -q.get_index(0),
        );
        out.set_index(3, a.get_index(3));
        return out;
    }

    pub fn zero(out: Float32Array) -> Float32Array {
        out.set_index(0, 0_f32);
        out.set_index(1, 0_f32);
        out.set_index(2, 0_f32);
        out.set_index(3, 0_f32);
        return out;
    }

    pub fn str(out: Float32Array) -> JsString {
        return JsString::from(format!(
            "vec4({}, {}, {}, {})",
            out.get_index(0),
            out.get_index(1),
            out.get_index(2),
            out.get_index(3)
        ));
    }

    pub fn exact_equals(a: &Float32Array, b: &Float32Array) -> bool {
        return (a.get_index(0) == b.get_index(0))
            && (a.get_index(1) == b.get_index(1))
            && (a.get_index(2) == b.get_index(2))
            && (a.get_index(3) == b.get_index(3));
    }

    pub fn equals(a: &Float32Array, b: &Float32Array) -> bool {
        return common::equals(a.get_index(0), b.get_index(0))
            && common::equals(a.get_index(1), b.get_index(1))
            && common::equals(a.get_index(2), b.get_index(2))
            && common::equals(a.get_index(3), b.get_index(3));
    }
}
