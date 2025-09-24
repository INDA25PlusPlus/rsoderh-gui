use ggez::{
    Context, GameResult,
    glam::Vec2,
    graphics::{self, Rect},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressState {
    Released,
    Pressed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseColors {
    released: graphics::Color,
    hover: graphics::Color,
    pressed: graphics::Color,
}

impl MouseColors {
    pub fn new(
        released: graphics::Color,
        hover: graphics::Color,
        pressed: graphics::Color,
    ) -> Self {
        Self {
            released,
            hover,
            pressed,
        }
    }

    pub fn get(&self, hovered: bool, press_state: PressState) -> graphics::Color {
        match (hovered, press_state) {
            (false, PressState::Released) => self.released,
            (true, PressState::Released) => self.hover,
            (_, PressState::Pressed) => self.pressed,
        }
    }
}

pub trait ButtonSpecialization {
    fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        bounds: Rect,
        press_state: PressState,
        hovered: bool,
    ) -> GameResult;
    fn on_press(&mut self);
}

pub struct Button {
    bounds: Rect,
    press_state: PressState,
    hovered: bool,
    button: Box<dyn ButtonSpecialization>,
}

impl Button {
    pub fn new(bounds: Rect, button: impl ButtonSpecialization + 'static) -> Self {
        Self {
            bounds,
            press_state: PressState::Released,
            hovered: false,
            button: Box::new(button),
        }
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }
    pub fn press_state(&self) -> PressState {
        self.press_state
    }
    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        self.button
            .draw(ctx, canvas, self.bounds, self.press_state, self.hovered)
    }

    pub fn on_press(&mut self) {
        self.button.on_press();
    }

    pub fn update_with_press_state(&mut self, position: Vec2, press_state: PressState) -> bool {
        if !self.bounds().contains(position) {
            self.press_state = PressState::Released;
            return false;
        }

        self.press_state = press_state;
        if press_state == PressState::Pressed {
            self.on_press()
        }

        true
    }

    pub fn update_with_mouse_position(&mut self, position: Vec2) {
        self.hovered = self.bounds().contains(position);
    }
}

pub struct RoundedButton {
    radius: f32,
    colors: MouseColors,
    on_press: Box<dyn FnMut()>,
}

impl RoundedButton {
    pub fn new(radius: f32, colors: MouseColors, on_press: impl FnMut() + 'static) -> Self {
        Self {
            radius,
            colors,
            on_press: Box::new(on_press),
        }
    }
}

impl ButtonSpecialization for RoundedButton {
    fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        bounds: Rect,
        press_state: PressState,
        hovered: bool,
    ) -> GameResult {
        let rect = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            self.radius,
            self.colors.get(hovered, press_state),
        )?;

        canvas.draw(&rect, graphics::DrawParam::new());

        Ok(())
    }
    fn on_press(&mut self) {
        (self.on_press)();
    }
}
