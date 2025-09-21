//! Collection of types which implement `ggez::graphics::Drawable`.

use ggez::{glam, graphics};

/// Image with a fixed rendered size. So the rendered size does not depend on the source image
/// dimensions.
pub struct SizedImage<'a> {
    image: &'a graphics::Image,
    dimensions: glam::Vec2,
}

impl<'a> SizedImage<'a> {
    pub fn new(image: &'a graphics::Image, dimensions: glam::Vec2) -> Self {
        Self { image, dimensions }
    }
}

impl<'a> graphics::Drawable for SizedImage<'a> {
    fn dimensions(
        &self,
        _gfx: &impl ggez::context::Has<graphics::GraphicsContext>,
    ) -> Option<graphics::Rect> {
        Some(graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: self.dimensions.x,
            h: self.dimensions.y,
        })
    }
    fn draw(&self, canvas: &mut graphics::Canvas, param: impl Into<graphics::DrawParam>) {
        let param: graphics::DrawParam = param.into();
        let src_dimensions = glam::Vec2::new(self.image.width() as f32, self.image.height() as f32);
        dbg!(src_dimensions);
        dbg!(self.dimensions / src_dimensions);
        canvas.draw(self.image, param.scale(self.dimensions / src_dimensions));
    }
}
