use n64_math::{vec3, Vec3};

pub fn to_fixpoint_10_2_as_integer(val: f32) -> u64 {
    (((val as i16) * (1 << 2)) & 0xffc) as u64
}

pub fn to_fixpoint_s_11_2(val: f32) -> u64 {
    let val2 = val * (1 << 2) as f32;

    #[allow(clippy::unnecessary_cast)]
    if val2 < -0x8000 as f32 {
        (-0x8000 as i16 & 0x3fff) as u64
    } else if val2 > 0x7fff as f32 {
        (0x7fff as i16 & 0x3fff) as u64
    } else {
        (val2 as i16 & 0x3fff) as u64
    }
}

pub fn to_fixpoint_s_10_5(val: f32) -> u64 {
    ((val * (1 << 5) as f32) as i16) as u64
}

pub fn fixed_16_16_to_f32(fixed_point: i32) -> f32 {
    let sign = fixed_point.signum();
    let abs = fixed_point.abs();

    let int = (abs as u32) >> 16;
    let frac = (abs as u32) & 0xffff;

    if sign >= 0 {
        int as f32 + frac as f32 / (1 << 16) as f32
    } else {
        -(int as f32 + frac as f32 / (1 << 16) as f32)
    }
}

pub fn float_to_unsigned_int_frac(val: f32) -> (u16, u16) {
    if 0.0 >= val {
        return (u16::MAX, u16::MAX);
    }

    let integer_part = libm::floorf(val);

    if (u16::MAX as f32) < integer_part {
        return (u16::MAX, u16::MAX);
    }

    let fractal_part = val - integer_part;

    (
        integer_part as u16,
        libm::floorf(fractal_part * ((1 << 16) as f32)) as u16,
    )
}

pub fn f32_to_fixed_16_16(val: f32) -> i32 {
    if (i16::MAX as f32) < val {
        return i32::MAX;
    } else if (i16::MIN as f32) > val {
        return i32::MIN;
    }

    (val * (1 << 16) as f32) as i32
}

// Dx/Dy of edge from p0 to p1.
// Dx/Dy (kx + m = y)
// x = (y-m)/k
// dx : 1/k
pub fn edge_slope(p0: Vec3, p1: Vec3) -> i32 {
    // TODO: ZERO DIVISION  (old epsilon 0.01)
    if 1.0 > libm::fabsf(p1.y - p0.y) {
        return f32_to_fixed_16_16(p1.x - p0.x);
    }
    f32_to_fixed_16_16((p1.x - p0.x) / (p1.y - p0.y))
}

// kx + m = y
// kx0 + m = y0
// kx1 + m = y1
// k(x1 - x0) = y1 - y0
// k = (y1 - y0)/(x1-x0)
// x0 * (y1 - y0)/(x1-x0) + m = y0
// m = y0 - x0*k
pub fn slope_x_from_y(p0: Vec3, p1: Vec3, y: f32) -> (u16, u16) {
    // kx + m = y
    // k = (p1y-p0y)/(p1x-p0x)
    // m = y0 - x0*k
    // x = (y-m)/k = (y- (y0 - x0*k))/k = y/k - y0/k + x0
    // x =  x0 + (y - y0)/k
    // x = p0x + (y - p0.y)*(p1x-p0x) / (p1y-p0y)

    // ZERO DIVISION check
    if 1.0 > libm::fabsf(p1.y - p0.y) {
        return float_to_unsigned_int_frac(p0.x);
    }

    let x = p0.x + (y - p0.y) * (p1.x - p0.x) / (p1.y - p0.y);

    float_to_unsigned_int_frac(x)
}

// X coordinate of the intersection of the edge from p0 to p1 and the sub-scanline at (or higher than) p0.y
pub fn slope_y_next_subpixel_intersection(p0: Vec3, p1: Vec3) -> (u16, u16) {
    let y = libm::ceilf(p0.y * 4.0) / 4.0;

    slope_x_from_y(p0, p1, y)
}

pub fn slope_y_prev_scanline_intersection(p0: Vec3, p1: Vec3) -> (u16, u16) {
    let y = libm::floorf(p0.y);

    slope_x_from_y(p0, p1, y)
}

// p0  y postive down
// p1
// p2
// p2 - p0 slope vs p1-p0 slope.
// 2_0 slope > 1_0 slope => left major
// 2_0 slope = (p2x-p0x)/(p2_y-p0_y)
// 1_0 slope = (p1x-p0x)/(p1_y-p0_y)
//   p2_y-p0_y > 0 && p1_y-p0_y > 0
// (p2x-p0x)/(p2_y-p0_y) > (p1x-p0x)/(p1_y-p0_y)
// if and only if (since denominators are positive)
//   (p2x-p0x)*(p1_y-p0_y) > (p1x-p0x)*(p2_y-p0_y)
pub fn is_triangle_right_major(p0: Vec3, p1: Vec3, p2: Vec3) -> bool {
    // Counter clockwise order?
    // (p0 - p1)x(p2 - p1) > 0 (area)
    // (p0x - p1x)   (p2x - p1x)    0
    // (p0y - p1y) x (p2y - p1y)  = 0
    //      0             0         Z

    // Z = (p0x - p1x)*(p2y - p1y) - (p2x - p1x)*(p0y - p1y);
    // Z > 0 => (p0x - p1x)*(p2y - p1y) > (p2x - p1x)*(p0y - p1y)

    (p0.x - p1.x) * (p2.y - p1.y) < (p2.x - p1.x) * (p0.y - p1.y)
}

// Sort so that v0.y <= v1.y <= v2.y
pub fn sorted_triangle(v0: Vec3, v1: Vec3, v2: Vec3) -> (Vec3, Vec3, Vec3) {
    if v0.y > v1.y {
        sorted_triangle(v1, v0, v2)
    } else if v0.y > v2.y {
        sorted_triangle(v2, v0, v1)
    } else if v1.y > v2.y {
        sorted_triangle(v0, v2, v1)
    } else {
        (v0, v1, v2)
    }
}

// Sort so that v0.y <= v1.y <= v2.y
pub fn sorted_triangle_indices(v0: Vec3, v1: Vec3, v2: Vec3) -> (u8, u8, u8) {
    if v0.y > v1.y {
        if v1.y > v2.y {
            // V0 > v1, V1 > V2
            (2, 1, 0)
        } else if v0.y > v2.y {
            // V0 > V1, V2 > V1, V0 > V2
            (1, 2, 0)
        } else {
            // V0 > V1, V2 > V1, V2 > V0
            (1, 0, 2)
        }
    } else if v0.y > v2.y {
        // V1 > V0, V0 > V2
        (2, 0, 1)
    } else if v1.y > v2.y {
        // V1 > v0, V2 > v0, V1 > V2
        (0, 2, 1)
    } else {
        //
        (0, 1, 2)
    }
}

pub fn triangle_is_too_small(v0: Vec3, v1: Vec3, v2: Vec3) -> bool {
    // Check area == 0
    (v0.x - v1.x) * (v2.y - v1.y) == (v0.y - v1.y) * (v2.x - v1.x)
}

// TODO: Take nz and va-vb & vc-vb instead
pub fn shaded_triangle_coeff(
    vb: Vec3,
    va: Vec3,
    vc: Vec3,
    bi: f32,
    ai: f32,
    ci: f32,
) -> (i32, i32, i32, i32) {
    // Already checked for nz = 0
    let nx = (va.y - vb.y) * (ci - bi) - (ai - bi) * (vc.y - vb.y);
    let ny = (ai - bi) * (vc.x - vb.x) - (va.x - vb.x) * (ci - bi);
    let nz = (va.x - vb.x) * (vc.y - vb.y) - (va.y - vb.y) * (vc.x - vb.x);
    let ne = ny + nx * (vc.x - vb.x) / (libm::fmaxf(1.0, vc.y - vb.y));

    let norm = -((1 << 16) as f32) / nz;

    let dcdx = safe_cast_i32(nx * norm);
    let dcdy = safe_cast_i32(ny * norm);
    let dcde = safe_cast_i32(ne * norm);

    let color = safe_cast_i32(bi * 65536.0);

    (dcdx, dcdy, dcde, color)
}

pub fn color_to_i32(color: u32) -> [i32; 3] {
    [
        ((color >> 24) & 0xff) as i32,
        ((color >> 16) & 0xff) as i32,
        ((color >> 8) & 0xff) as i32,
    ]
}

pub fn z_buff_val_transform(z: f32) -> f32 {
    let scale = 0x3ff as f32;
    32.0 * z * scale
}

pub fn z_triangle_coeff(vh: Vec3, vm: Vec3, vl: Vec3) -> (i32, i32, i32, i32) {
    let (dx, dy, de, val) = shaded_triangle_coeff(
        vh,
        vm,
        vl,
        z_buff_val_transform(vh.z),
        z_buff_val_transform(vm.z),
        z_buff_val_transform(vl.z),
    );
    (val, dx, de, dy)
}

pub fn truncate_to_pixel(val: Vec3) -> Vec3 {
    vec3(libm::floorf(val.x), libm::floorf(val.y), val.z)
}

pub fn safe_cast_i32(val: f32) -> i32 {
    if (i32::MAX as f32) < val {
        i32::MAX
    } else if (i32::MIN as f32) > val {
        i32::MIN
    } else {
        val as i32
    }
}
