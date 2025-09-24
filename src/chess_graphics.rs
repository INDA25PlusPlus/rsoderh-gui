//! Collection of types which implement `ggez::graphics::Drawable`.

use ggez::{GameResult, context, glam, graphics};

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
        canvas.draw(self.image, param.scale(self.dimensions / src_dimensions));
    }
}

/// The radius of each corner of a rounded rectangle.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct BorderRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl BorderRadii {
    pub fn zero() -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_left: 0.0,
            bottom_right: 0.0,
        }
    }
}

pub struct RoundedRectangle {
    bounds: graphics::Rect,
    meshes: [graphics::Mesh; 5],
}

impl RoundedRectangle {
    pub fn new(
        gfx: &impl context::Has<graphics::GraphicsContext>,
        mode: graphics::DrawMode,
        bounds: graphics::Rect,
        corners: BorderRadii,
        color: graphics::Color,
    ) -> GameResult<Self> {
        let top_left = glam::vec2(bounds.left(), bounds.top());
        let top_right = glam::vec2(bounds.right(), bounds.top());
        let bottom_left = glam::vec2(bounds.left(), bounds.bottom());
        let bottom_right = glam::vec2(bounds.right(), bounds.bottom());

        // TODO: Figure out tolerance
        let tolerance = 0.001;

        let top_left_corner = graphics::Mesh::new_circle(
            gfx,
            mode,
            top_left + glam::vec2(corners.top_left, corners.top_left),
            corners.top_left,
            tolerance,
            color,
        )?;
        let top_right_corner = graphics::Mesh::new_circle(
            gfx,
            mode,
            top_right + glam::vec2(-corners.top_right, corners.top_right),
            corners.top_right,
            tolerance,
            color,
        )?;
        let bottom_left_corner = graphics::Mesh::new_circle(
            gfx,
            mode,
            bottom_left + glam::vec2(corners.bottom_left, -corners.bottom_left),
            corners.bottom_left,
            tolerance,
            color,
        )?;
        let bottom_right_corner = graphics::Mesh::new_circle(
            gfx,
            mode,
            bottom_right + glam::vec2(-corners.bottom_right, -corners.bottom_right),
            corners.bottom_right,
            tolerance,
            color,
        )?;

        let center = graphics::Mesh::new_polygon(
            gfx,
            mode,
            &[
                top_left + glam::vec2(corners.top_left, 0.0),
                top_right + glam::vec2(-corners.top_right, 0.0),
                top_right + glam::vec2(-corners.top_right, corners.top_right),
                top_right + glam::vec2(0.0, corners.top_right),
                bottom_right + glam::vec2(0.0, -corners.bottom_right),
                bottom_right + glam::vec2(-corners.bottom_right, -corners.bottom_right),
                bottom_right + glam::vec2(-corners.bottom_right, 0.0),
                bottom_left + glam::vec2(corners.bottom_left, 0.0),
                bottom_left + glam::vec2(corners.bottom_left, -corners.bottom_left),
                bottom_left + glam::vec2(0.0, -corners.bottom_left),
                top_left + glam::vec2(0.0, corners.top_left),
                top_left + glam::vec2(corners.top_left, corners.top_left),
            ],
            color,
        )?;

        Ok(Self {
            bounds,
            meshes: [
                top_left_corner,
                top_right_corner,
                bottom_left_corner,
                bottom_right_corner,
                center,
            ],
        })
    }
}

impl graphics::Drawable for RoundedRectangle {
    fn dimensions(
        &self,
        _gfx: &impl context::Has<graphics::GraphicsContext>,
    ) -> Option<graphics::Rect> {
        Some(self.bounds)
    }
    fn draw(&self, canvas: &mut graphics::Canvas, param: impl Into<graphics::DrawParam>) {
        let param = param.into();
        for mesh in &self.meshes {
            canvas.draw(mesh, param);
        }
    }
}
