use super::common;
use super::common::js_round;
use js_sys::Float32Array;
use js_sys::JsString;
use rand;
use wasm_bindgen::prelude::*;

#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub struct vec3 {}

#[allow(non_camel_case_types)]
#[wasm_bindgen]
impl vec3 {
    pub fn create() -> Float32Array {
        return Float32Array::new_with_length(3);
    }

    pub fn clone(a: &Float32Array) -> Float32Array {
        let out = vec3::create();
        out.set_index(0, a.get_index(0));
        out.set_index(1, a.get_index(1));
        out.set_index(2, a.get_index(2));
        return out;
    }

    pub fn from_values(x: f32, y: f32, z: f32) -> Float32Array {
        let out = vec3::create();
        out.set_index(0, x);
        out.set_index(1, y);
        out.set_index(2, z);
        return out;
    }

    pub fn copy(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0));
        out.set_index(1, a.get_index(1));
        out.set_index(2, a.get_index(2));
        return out;
    }

    pub fn set(out: Float32Array, x: f32, y: f32, z: f32) -> Float32Array {
        out.set_index(0, x);
        out.set_index(1, y);
        out.set_index(2, z);
        return out;
    }

    pub fn add(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) + b.get_index(0));
        out.set_index(1, a.get_index(1) + b.get_index(1));
        out.set_index(2, a.get_index(2) + b.get_index(2));
        return out;
    }

    pub fn subtract(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) - b.get_index(0));
        out.set_index(1, a.get_index(1) - b.get_index(1));
        out.set_index(2, a.get_index(2) - b.get_index(2));
        return out;
    }

    pub fn multiply(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) * b.get_index(0));
        out.set_index(1, a.get_index(1) * b.get_index(1));
        out.set_index(2, a.get_index(2) * b.get_index(2));
        return out;
    }

    pub fn divide(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0) / b.get_index(0));
        out.set_index(1, a.get_index(1) / b.get_index(1));
        out.set_index(2, a.get_index(2) / b.get_index(2));
        return out;
    }

    pub fn ceil(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).ceil());
        out.set_index(1, a.get_index(1).ceil());
        out.set_index(2, a.get_index(2).ceil());
        return out;
    }

    pub fn floor(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).floor());
        out.set_index(1, a.get_index(1).floor());
        out.set_index(2, a.get_index(2).floor());
        return out;
    }

    pub fn min(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).min(b.get_index(0)));
        out.set_index(1, a.get_index(1).min(b.get_index(1)));
        out.set_index(2, a.get_index(2).min(b.get_index(2)));
        return out;
    }

    pub fn max(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(0, a.get_index(0).max(b.get_index(0)));
        out.set_index(1, a.get_index(1).max(b.get_index(1)));
        out.set_index(2, a.get_index(2).max(b.get_index(2)));
        return out;
    }

    pub fn js_round(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, js_round!(a.get_index(0)));
        out.set_index(1, js_round!(a.get_index(1)));
        out.set_index(2, js_round!(a.get_index(2)));
        return out;
    }

    pub fn scale(out: Float32Array, a: &Float32Array, b: f32) -> Float32Array {
        out.set_index(0, a.get_index(0) * b);
        out.set_index(1, a.get_index(1) * b);
        out.set_index(2, a.get_index(1) * b);
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
        return out;
    }

    pub fn distance(a: &Float32Array, b: &Float32Array) -> f32 {
        let x: f32 = b.get_index(0) - a.get_index(0);
        let y: f32 = b.get_index(1) - a.get_index(1);
        let z: f32 = b.get_index(2) - a.get_index(2);
        return (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    }

    pub fn squared_distance(a: &Float32Array, b: &Float32Array) -> f32 {
        let x: f32 = b.get_index(0) - a.get_index(0);
        let y: f32 = b.get_index(1) - a.get_index(1);
        let z: f32 = b.get_index(2) - a.get_index(2);
        return x.powi(2) + y.powi(2) + z.powi(2);
    }

    pub fn length(a: &Float32Array) -> f32 {
        return (a.get_index(0).powi(2) + a.get_index(1).powi(2) + a.get_index(2).powi(2)).sqrt();
    }

    pub fn squared_length(a: &Float32Array) -> f32 {
        return a.get_index(0).powi(2) + a.get_index(1).powi(2) + a.get_index(2).powi(2);
    }

    pub fn negate(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, -a.get_index(0));
        out.set_index(1, -a.get_index(1));
        out.set_index(2, -a.get_index(2));
        return out;
    }

    pub fn inverse(out: Float32Array, a: &Float32Array) -> Float32Array {
        out.set_index(0, 1_f32 / a.get_index(0));
        out.set_index(1, 1_f32 / a.get_index(1));
        out.set_index(2, 1_f32 / a.get_index(2));
        return out;
    }

    pub fn normalize(out: Float32Array, a: &Float32Array) -> Float32Array {
        let len: f32 = vec3::length(&a);
        if len <= f32::EPSILON {
            out.set_index(0, a.get_index(0) / len);
            out.set_index(1, a.get_index(1) / len);
            out.set_index(2, a.get_index(2) / len);
        } else {
            out.set_index(0, 0_f32);
            out.set_index(1, 0_f32);
            out.set_index(2, 0_f32);
        }
        return out;
    }

    pub fn dot(a: &Float32Array, b: &Float32Array) -> f32 {
        return a.get_index(0) * b.get_index(0)
            + a.get_index(1) * b.get_index(1)
            + a.get_index(2) * b.get_index(2);
    }

    pub fn cross(out: Float32Array, a: &Float32Array, b: &Float32Array) -> Float32Array {
        out.set_index(
            0,
            a.get_index(1) * b.get_index(2) - a.get_index(2) * b.get_index(1),
        );
        out.set_index(
            1,
            a.get_index(2) * b.get_index(0) - a.get_index(0) * b.get_index(2),
        );
        out.set_index(
            2,
            a.get_index(0) * b.get_index(1) - a.get_index(1) * b.get_index(0),
        );
        return out;
    }

    pub fn lerp(out: Float32Array, a: &Float32Array, b: &Float32Array, t: f32) -> Float32Array {
        out.set_index(0, a.get_index(0) + t * (b.get_index(0) - a.get_index(0)));
        out.set_index(1, a.get_index(1) + t * (b.get_index(1) - a.get_index(1)));
        out.set_index(2, a.get_index(2) + t * (b.get_index(2) - a.get_index(2)));
        return out;
    }

    pub fn slerp(out: Float32Array, a: &Float32Array, b: &Float32Array, t: f32) -> Float32Array {
        let angle: f32 = vec3::dot(a, b).max(-1_f32).min(1_f32).acos();
        let sin_total: f32 = angle.sin();

        let ratio_a = ((1_f32 - t) * angle).sin() / sin_total;
        let ratio_b = (t * angle).sin() / sin_total;
        out.set_index(0, ratio_a * a.get_index(0) + ratio_b * b.get_index(0));
        out.set_index(1, ratio_a * a.get_index(1) + ratio_b * b.get_index(1));
        out.set_index(2, ratio_a * a.get_index(2) + ratio_b * b.get_index(2));
        return out;
    }

    pub fn hermit(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        c: &Float32Array,
        d: &Float32Array,
        t: f32,
    ) -> Float32Array {
        let factor1: f32 = t.powi(2) * (2_f32 * t - 3_f32) + 1_f32;
        let factor2: f32 = t.powi(2) * (t - 2_f32) + t;
        let factor3: f32 = t.powi(2) * (t - 1_f32);
        let factor4: f32 = t.powi(2) * (-2_f32 * t + 3_f32);

        out.set_index(
            0,
            factor1 * a.get_index(0)
                + factor2 * b.get_index(0)
                + factor3 * c.get_index(0)
                + factor4 * d.get_index(0),
        );
        out.set_index(
            1,
            factor1 * a.get_index(1)
                + factor2 * b.get_index(1)
                + factor3 * c.get_index(1)
                + factor4 * d.get_index(1),
        );
        out.set_index(
            2,
            factor1 * a.get_index(2)
                + factor2 * b.get_index(2)
                + factor3 * c.get_index(2)
                + factor4 * d.get_index(2),
        );
        return out;
    }

    pub fn bezier(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        c: &Float32Array,
        d: &Float32Array,
        t: f32,
    ) -> Float32Array {
        let factor1: f32 = (1_f32 - t).powi(3);
        let factor2: f32 = 3_f32 * t * (1_f32 - t).powi(2);
        let factor3: f32 = 3_f32 * t.powi(2) * (1_f32 - t);
        let factor4: f32 = t.powi(3);

        out.set_index(
            0,
            factor1 * a.get_index(0)
                + factor2 * b.get_index(0)
                + factor3 * c.get_index(0)
                + factor4 * d.get_index(0),
        );
        out.set_index(
            1,
            factor1 * a.get_index(1)
                + factor2 * b.get_index(1)
                + factor3 * c.get_index(1)
                + factor4 * d.get_index(1),
        );
        out.set_index(
            2,
            factor1 * a.get_index(2)
                + factor2 * b.get_index(2)
                + factor3 * c.get_index(2)
                + factor4 * d.get_index(2),
        );
        return out;
    }

    pub fn js_random(out: Float32Array, scale: f32) -> Float32Array {
        let r: f32 = rand::random::<f32>() * 2.0_f32 * std::f32::consts::PI;
        let z: f32 = rand::random::<f32>() * 2.0_f32 - 1_f32;
        let z_scale: f32 = (1_f32 - z.powi(2)) * scale;

        out.set_index(0, z_scale * r.cos());
        out.set_index(1, z_scale * r.sin());
        out.set_index(2, z * scale);
        return out;
    }

    pub fn transform_mat_4(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
        let mut w: f32 = m.get_index(3) * a.get_index(0)
            + m.get_index(7) * a.get_index(1)
            + m.get_index(11) * a.get_index(1)
            + m.get_index(15);
        if w == 0_f32 {
            w = 1_f32;
        }
        out.set_index(
            0,
            (m.get_index(0) * a.get_index(0)
                + m.get_index(4) * a.get_index(1)
                + m.get_index(8) * a.get_index(2)
                + m.get_index(12))
                / w,
        );
        out.set_index(
            1,
            (m.get_index(1) * a.get_index(0)
                + m.get_index(5) * a.get_index(1)
                + m.get_index(9) * a.get_index(2)
                + m.get_index(13))
                / w,
        );
        out.set_index(
            2,
            (m.get_index(2) * a.get_index(0)
                + m.get_index(6) * a.get_index(1)
                + m.get_index(10) * a.get_index(2)
                + m.get_index(14))
                / w,
        );
        return out;
    }

    pub fn transform_mat_3(out: Float32Array, a: &Float32Array, m: &Float32Array) -> Float32Array {
        out.set_index(
            0,
            m.get_index(0) * a.get_index(0)
                + m.get_index(3) * a.get_index(1)
                + m.get_index(6) * a.get_index(2),
        );
        out.set_index(
            1,
            m.get_index(1) * a.get_index(0)
                + m.get_index(4) * a.get_index(1)
                + m.get_index(7) * a.get_index(2),
        );
        out.set_index(
            2,
            m.get_index(2) * a.get_index(0)
                + m.get_index(5) * a.get_index(1)
                + m.get_index(8) * a.get_index(2),
        );
        return out;
    }

    pub fn transform_quat(out: Float32Array, a: &Float32Array, q: &Float32Array) -> Float32Array {
        let uvx: f32 = q.get_index(1) * a.get_index(2) - q.get_index(2) * a.get_index(1);
        let uvy: f32 = q.get_index(3) * a.get_index(0) - q.get_index(0) * a.get_index(2);
        let uvz: f32 = q.get_index(0) * a.get_index(1) - q.get_index(1) * a.get_index(0);

        let uuvx: f32 = q.get_index(1) * uvz - q.get_index(2) * uvy;
        let uuvy: f32 = q.get_index(3) * uvx - q.get_index(0) * uvz;
        let uuvz: f32 = q.get_index(0) * uvy - q.get_index(1) * uvx;

        out.set_index(0, a.get_index(0) * q.get_index(4) * uvx + 2_f32 * uuvx);
        out.set_index(1, a.get_index(1) * q.get_index(4) * uvy + 2_f32 * uuvy);
        out.set_index(2, a.get_index(2) * q.get_index(4) * uvz + 2_f32 * uuvz);
        return out;
    }

    pub fn rotate_x(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        rad: f32,
    ) -> Float32Array {
        let p = vec3::subtract(Float32Array::new_with_length(3), a, b);
        let r = vec3::from_values(
            p.get_index(0),
            p.get_index(1) * rad.cos() - p.get_index(2) * rad.sin(),
            p.get_index(2) * rad.sin() - p.get_index(2) * rad.cos(),
        );

        out.set_index(0, r.get_index(0) + b.get_index(0));
        out.set_index(1, r.get_index(1) + b.get_index(1));
        out.set_index(2, r.get_index(2) + b.get_index(2));
        return out;
    }

    pub fn rotate_y(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        rad: f32,
    ) -> Float32Array {
        let p = vec3::subtract(Float32Array::new_with_length(3), a, b);
        let r = vec3::from_values(
            p.get_index(2) * rad.sin() + p.get_index(0) * rad.cos(),
            p.get_index(1),
            p.get_index(2) * rad.cos() - p.get_index(0) * rad.sin(),
        );

        out.set_index(0, r.get_index(0) + b.get_index(0));
        out.set_index(1, r.get_index(1) + b.get_index(1));
        out.set_index(2, r.get_index(2) + b.get_index(2));
        return out;
    }

    pub fn rotate_z(
        out: Float32Array,
        a: &Float32Array,
        b: &Float32Array,
        rad: f32,
    ) -> Float32Array {
        let p = vec3::subtract(Float32Array::new_with_length(3), a, b);
        let r = vec3::from_values(
            p.get_index(0) * rad.cos() - p.get_index(1) * rad.sin(),
            p.get_index(0) * rad.sin() + p.get_index(1) * rad.cos(),
            p.get_index(2),
        );

        out.set_index(0, r.get_index(0) + b.get_index(0));
        out.set_index(1, r.get_index(1) + b.get_index(1));
        out.set_index(2, r.get_index(2) + b.get_index(2));
        return out;
    }

    pub fn zero(out: Float32Array) -> Float32Array {
        out.set_index(0, 0_f32);
        out.set_index(1, 0_f32);
        out.set_index(2, 0_f32);
        return out;
    }

    pub fn str(out: Float32Array) -> JsString {
        return JsString::from(format!(
            "vec3({}, {}, {})",
            out.get_index(0),
            out.get_index(1),
            out.get_index(2)
        ));
    }

    pub fn exact_equals(a: &Float32Array, b: &Float32Array) -> bool {
        return (a.get_index(0) == b.get_index(0))
            && (a.get_index(1) == b.get_index(1))
            && (a.get_index(2) == b.get_index(2));
    }

    pub fn equals(a: &Float32Array, b: &Float32Array) -> bool {
        return common::equals(a.get_index(0), b.get_index(0))
            && common::equals(a.get_index(1), b.get_index(1))
            && common::equals(a.get_index(2), b.get_index(2));
    }
}
