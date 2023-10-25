use std::cmp::{max, min};

pub fn composite_zindex(z: i128, z0: i128, z1: i128, z2: i128) -> i128 {
    let scale = 1 << 10; // 1024
    let normalize = |mut v: i128| -> i128 {
        // v = v.abs();
        v = v + scale / 2;
        v = max(0, min(v, scale - 1));
        return v;
    };
    return normalize(z) * scale * scale * scale
        + normalize(z0) * scale * scale
        + normalize(z1) * scale
        + normalize(z2)
        - 1024 * 1024 * 1024 * 512;
}

pub fn cal_ax(ox: f32, width: f32) -> f32 {
    let x = (ox - width / 2.0) / width;
    return x;
}

pub fn cal_ay(oy: f32, height: f32) -> f32 {
    let y = (oy - height / 2.0) / height;
    return y;
}
