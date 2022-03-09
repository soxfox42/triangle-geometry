use nannou::geom::{Rect, Tri};
use nannou::glam::{Vec2, Vec3};

const RIGHT: f32 = std::f32::consts::FRAC_PI_2;
// To allow triangles that are very close but not exact matches to be identified
const NEAR_ZERO: f32 = 0.0001;

// Several of the calculations in this module could be performed with Vec2's existing methods, but
// to stick to the rules of the challenge they are reimplemented here, only using Vec2 as a way to
// store 2D coordinates.

pub fn view_transform(tri: Tri<Vec2>, view: Rect) -> (Vec3, f32) {
    let bounding_rect = tri.bounding_rect();

    let w_ratio = bounding_rect.w() / view.w();
    let h_ratio = bounding_rect.h() / view.h();
    let max_ratio = f32::max(w_ratio, h_ratio);
    let scale = 1.0 / max_ratio;

    let x_offset = view.x() - bounding_rect.x() * scale;
    let y_offset = view.y() - bounding_rect.y() * scale;

    (Vec3::new(x_offset, y_offset, 0.0), scale)
}

// Pythagorean theorem to find segment lengths
pub fn line_segments(tri: Tri<Vec2>) -> (f32, f32, f32) {
    let [a_x, a_y] = tri[0].as_ref();
    let [b_x, b_y] = tri[1].as_ref();
    let [c_x, c_y] = tri[2].as_ref();

    (
        ((a_x - b_x).powi(2) + (a_y - b_y).powi(2)).sqrt(),
        ((b_x - c_x).powi(2) + (b_y - c_y).powi(2)).sqrt(),
        ((c_x - a_x).powi(2) + (c_y - a_y).powi(2)).sqrt(),
    )
}

// 3 points to triangle area
pub fn area(tri: Tri<Vec2>) -> f32 {
    let [a_x, a_y] = tri[0].as_ref();
    let [b_x, b_y] = tri[1].as_ref();
    let [c_x, c_y] = tri[2].as_ref();

    ((a_x * (b_y - c_y) + b_x * (c_y - a_y) + c_x * (a_y - b_y)) / 2.0).abs()
}

// Sum line segments to find perimenter
pub fn perimeter(tri: Tri<Vec2>) -> f32 {
    let (ab, bc, ca) = line_segments(tri);

    ab + bc + ca
}

// Use dot products to calculate angles
fn angles(tri: Tri<Vec2>) -> (f32, f32, f32) {
    let (ab, bc, ca) = line_segments(tri);
    let [a_x, a_y] = tri[0].as_ref();
    let [b_x, b_y] = tri[1].as_ref();
    let [c_x, c_y] = tri[2].as_ref();

    (
        (((b_x - a_x) * (c_x - a_x) + (b_y - a_y) * (c_y - a_y)) / ab / ca).acos(),
        (((c_x - b_x) * (a_x - b_x) + (c_y - b_y) * (a_y - b_y)) / bc / ab).acos(),
        (((a_x - c_x) * (b_x - c_x) + (a_y - c_y) * (b_y - c_y)) / ca / bc).acos(),
    )
}

fn is_right(angle: f32) -> bool {
    (angle - RIGHT).abs() < NEAR_ZERO
}

pub fn classify(tri: Tri<Vec2>) -> String {
    let [a_x, a_y] = tri[0].as_ref();
    let [b_x, b_y] = tri[1].as_ref();
    let [c_x, c_y] = tri[2].as_ref();

    let mut result = String::new();

    let m_1 = (a_y - b_y) / (a_x - b_x);
    let m_2 = (a_y - c_y) / (a_x - c_x);

    if (m_1 - m_2).abs() < NEAR_ZERO {
        return String::from("Degenerate");
    }

    let (a_ang, b_ang, c_ang) = angles(tri);
    if is_right(a_ang) || is_right(b_ang) || is_right(c_ang) {
        result.push_str("Right");
    } else if a_ang > RIGHT || b_ang > RIGHT || c_ang > RIGHT {
        result.push_str("Obtuse");
    } else {
        result.push_str("Acute");
    }

    result.push(' ');

    let (ab, bc, ca) = line_segments(tri);

    if (ab - bc).abs() < NEAR_ZERO && (ab - ca).abs() < NEAR_ZERO {
        result.push_str("Equilateral");
    } else if (ab - bc).abs() < NEAR_ZERO
        || (bc - ca).abs() < NEAR_ZERO
        || (ca - ab).abs() < NEAR_ZERO
    {
        result.push_str("Isosceles");
    } else {
        result.push_str("Scalene");
    }

    result
}
