use nannou::geom::{Rect, Tri};
use nannou::glam::{Vec2, Vec3};

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

pub fn area(tri: Tri<Vec2>) -> f32 {
    let [a_x, a_y] = tri[0].as_ref();
    let [b_x, b_y] = tri[1].as_ref();
    let [c_x, c_y] = tri[2].as_ref(); 

    ((a_x * (b_y - c_y) + b_x * (c_y - a_y) + c_x * (a_y - b_y)) / 2.0).abs()
}

pub fn perimeter(tri: Tri<Vec2>) -> f32 {
    let (ab, bc, ca) = line_segments(tri);

    ab + bc + ca
}
