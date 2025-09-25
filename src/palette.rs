use std::sync::LazyLock;

use colors_transform::{AlphaColor, Color, Rgb};
use ggez::graphics;

pub struct Palette {
    pub piece_white: graphics::Color,
    pub piece_black: graphics::Color,
    pub board_square_black: graphics::Color,
    pub board_square_black_hover: graphics::Color,
    pub board_square_black_active: graphics::Color,
    pub board_square_white: graphics::Color,
    pub board_square_white_hover: graphics::Color,
    pub board_square_white_active: graphics::Color,
    pub board_square_selected: graphics::Color,
    pub highlight_subtle_overlay: graphics::Color,
    pub button: graphics::Color,
    pub button_hover: graphics::Color,
    pub button_active: graphics::Color,
    pub text_subtle: graphics::Color,
}

impl Default for Palette {
    fn default() -> Self {
        fn convert_color(color: impl Color + AlphaColor) -> graphics::Color {
            let (r, g, b) = color.to_rgb().as_tuple();

            graphics::Color::from_rgba(r as u8, g as u8, b as u8, (color.get_alpha() * 255.0) as u8)
        }

        let board_square_black = Rgb::from_hex_str("#cfa59b").unwrap();
        let board_square_white = Rgb::from_hex_str("#ede1d1").unwrap();

        Self {
            piece_white: convert_color(Rgb::from_hex_str("#ffe7c4").unwrap()),
            piece_black: convert_color(Rgb::from_hex_str("#636363").unwrap()),
            board_square_black: convert_color(board_square_black),
            board_square_black_hover: convert_color(board_square_black),
            board_square_black_active: convert_color(board_square_black.lighten(-10.0)),
            board_square_white: convert_color(board_square_white),
            board_square_white_hover: convert_color(board_square_white),
            board_square_white_active: convert_color(board_square_white.lighten(-10.0)),
            board_square_selected: convert_color(Rgb::from_hex_str("#e9da57").unwrap()),
            highlight_subtle_overlay: convert_color(
                Rgb::from_hex_str("#000000").unwrap().set_alpha(0.3),
            ),
            button: convert_color(Rgb::from_hex_str("#22211e").unwrap()),
            button_hover: convert_color(Rgb::from_hex_str("#393734").unwrap()),
            button_active: convert_color(Rgb::from_hex_str("#1b1a18").unwrap()),
            text_subtle: convert_color(Rgb::from_hex_str("#aea696").unwrap()),
        }
    }
}

pub static PALETTE: LazyLock<Palette> = LazyLock::new(|| Palette::default());
