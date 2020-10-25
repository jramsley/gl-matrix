use super::common;
use super::common::js_round;
use js_sys::Array;
use js_sys::Float32Array;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub struct mat2 {}

#[allow(non_camel_case_types)]
#[wasm_bindgen]
impl mat2 {
    pub fn create() -> Float32Array {
        let out: Float32Array = Float32Array::new_with_length(4);
        out.set_index(0, 1_f32);
        out.set_index(3, 1_f32);
        return out;
    }

    pub fn identity(out: Float32Array) -> Float32Array {
        out.set_index(0, 1_f32);
        out.set_index(1, 0_f32);
        out.set_index(2, 0_f32);
        out.set_index(3, 1_f32);
        return out;
    }

    pub fn clone(a: &Float32Array) -> Float32Array {
        let out = Float32Array::new_with_length(4);
        out.set_index(0, a.get_index(0));
        out.set_index(1, a.get_index(1));
        out.set_index(2, a.get_index(2));
        out.set_index(3, a.get_index(3));
        return out;
    }

    pub fn from_values(m00: f32, m01: f32, m10: f32, m11: f32) -> Float32Array {
        let out = Float32Array::new_with_length(4);
        out.set_index(0, m00);
        out.set_index(1, m01);
        out.set_index(2, m10);
        out.set_index(3, m11);
        return out;
    }

    pub fn copy(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0));
        out.set_index(1, a.get_index(1));
        out.set_index(2, a.get_index(2));
        out.set_index(3, a.get_index(3));
        return out;
    }

    pub fn set(out: Float32Array, m00: f32, m01: f32, m10: f32, m11: f32) -> Float32Array {
        out.set_index(0, m00);
        out.set_index(1, m01);
        out.set_index(2, m10);
        out.set_index(3, m11);
        return out;
    }

    pub fn transpose(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0));
        out.set_index(1, -a.get_index(2));
        out.set_index(2, -a.get_index(1));
        out.set_index(3, a.get_index(3));
        return out;
    }

    pub fn adjoint(a: &Float32Array) -> f32 {
        return a.get_index(0) * a.get_index(3) - a.get_index(2) * a.get_index(1);
    }

    pub fn determinant(a: &Float32Array) -> f32 {
        return a.get_index(0) * a.get_index(3) - a.get_index(2) * a.get_index(1);
    }

    pub fn invert(out: Float32Array, a: &Float32Array) -> Option<Float32Array> {
        let det = mat2::determinant(a);
        if det == 0_f32 {
            return None;
        } else {
            out.set_index(0, a.get_index(3) / det);
            out.set_index(1, -a.get_index(1) / det);
            out.set_index(2, -a.get_index(2) / det);
            out.set_index(3, a.get_index(0) / det);
            return Some(out);
        }
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
        out.set_index(
            0,
            a.get_index(0) * b.get_index(0) + a.get_index(2) * b.get_index(1),
        );
        out.set_index(
            1,
            a.get_index(1) * b.get_index(0) + a.get_index(3) * b.get_index(1),
        );
        out.set_index(
            2,
            a.get_index(0) * b.get_index(2) + a.get_index(2) * b.get_index(3),
        );
        out.set_index(
            3,
            a.get_index(1) * b.get_index(2) + a.get_index(3) * b.get_index(3),
        );
        return out;
    }

    pub fn rotate(out: Float32Array, a: &Float32Array, rad: f32) -> Float32Array {
        out.set_index(0, a.get_index(0) * rad.cos() + a.get_index(2) * rad.sin());
        out.set_index(1, a.get_index(1) * rad.cos() + a.get_index(3) * rad.sin());
        out.set_index(2, a.get_index(0) * -rad.sin() + a.get_index(2) * rad.cos());
        out.set_index(3, a.get_index(1) * -rad.sin() + a.get_index(3) * rad.cos());
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

    pub fn scale(out: Float32Array, a: &Float32Array, v: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) * v.get_index(0));
        out.set_index(1, a.get_index(1) * v.get_index(0));
        out.set_index(2, a.get_index(2) * v.get_index(1));
        out.set_index(3, a.get_index(3) * v.get_index(1));
        return out;
    }

    pub fn from_rotation(out: Float32Array, rad: f32) -> Float32Array {
        out.set_index(0, rad.cos());
        out.set_index(1, rad.sin());
        out.set_index(2, -rad.sin());
        out.set_index(3, rad.cos());
        return out;
    }

    pub fn from_scaling(out: Float32Array, v: &Float32Array) -> Float32Array {
        out.set_index(0, v.get_index(0));
        out.set_index(1, 0_f32);
        out.set_index(2, 0_f32);
        out.set_index(3, v.get_index(1));
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
            "mat2({}, {}, {}, {})",
            out.get_index(0),
            out.get_index(1),
            out.get_index(2),
            out.get_index(3)
        ));
    }

    pub fn frob(a: &Float32Array) -> f32 {
        return (a.get_index(0).powi(2)
            + a.get_index(1).powi(2)
            + a.get_index(2).powi(2)
            + a.get_index(3).powi(2))
        .sqrt();
    }

    pub fn l_d_u(l: &Float32Array, d: &Float32Array, u: &Float32Array, a: &Float32Array) -> Array {
        l.set_index(2, a.get_index(2) / a.get_index(0));
        u.set_index(0, a.get_index(0));
        u.set_index(1, a.get_index(1));
        u.set_index(3, a.get_index(3) - l.get_index(2) * u.get_index(1));
        return Array::of3(l, d, u);
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

    pub fn multiply_scalar(out: Float32Array, a: &Float32Array, b: f32) -> Float32Array {
        out.set_index(0, a.get_index(0) * b);
        out.set_index(1, a.get_index(1) * b);
        out.set_index(2, a.get_index(2) * b);
        out.set_index(3, a.get_index(3) * b);
        return out;
    }

    pub fn multiply_scalar_and_add(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        scale: f32,
    ) -> Float32Array {
        out.set_index(0, a.get_index(0) + b.get_index(0) * scale);
        out.set_index(1, a.get_index(1) + b.get_index(1) * scale);
        out.set_index(2, a.get_index(2) + b.get_index(2) * scale);
        out.set_index(3, a.get_index(3) + b.get_index(3) * scale);
        return out;
    }
}
