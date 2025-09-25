//! Additional methods for the ggez::graphics::Rect struct.

use ggez::{glam, graphics};

pub trait RectUtils {
    fn top_left(&self) -> glam::Vec2;
    #[allow(unused)]
    fn top_right(&self) -> glam::Vec2;
    fn bottom_left(&self) -> glam::Vec2;
    #[allow(unused)]
    fn bottom_right(&self) -> glam::Vec2;
    #[allow(unused)]
    fn outset(&self, offset: f32) -> Self;
}

impl RectUtils for graphics::Rect {
    fn top_left(&self) -> glam::Vec2 {
        glam::vec2(self.left(), self.top())
    }
    fn top_right(&self) -> glam::Vec2 {
        glam::vec2(self.right(), self.top())
    }
    fn bottom_left(&self) -> glam::Vec2 {
        glam::vec2(self.left(), self.bottom())
    }
    fn bottom_right(&self) -> glam::Vec2 {
        glam::vec2(self.right(), self.bottom())
    }
    fn outset(&self, offset: f32) -> Self {
        Self {
            x: self.x - offset,
            y: self.y - offset,
            w: self.w + offset,
            h: self.h + offset,
        }
    }
}
