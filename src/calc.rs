use nannou::geom::{Rect, Tri};
use nannou::glam::{Vec2, Vec3};

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
