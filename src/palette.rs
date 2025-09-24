use std::sync::LazyLock;

use ggez::graphics::Color;

pub struct Palette {
    pub piece_white: Color,
    pub piece_black: Color,
    pub board_square_black: Color,
    pub board_square_black_hover: Color,
    pub board_square_black_active: Color,
    pub board_square_white: Color,
    pub board_square_white_hover: Color,
    pub board_square_white_active: Color,
    pub button: Color,
    pub button_hover: Color,
    pub button_active: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            piece_white: Color::from_rgb_u32(0xFFE7C4),
            piece_black: Color::from_rgb_u32(0x636363),
            board_square_black: Color::from_rgb_u32(0xC490D1),
            board_square_black_hover: Color::from_rgb_u32(0xa97cb4),
            board_square_black_active: Color::from_rgb_u32(0xC490D1),
            board_square_white: Color::from_rgb_u32(0xEDE1D1),
            board_square_white_hover: Color::from_rgb_u32(0xd5ccbe),
            board_square_white_active: Color::from_rgb_u32(0xEDE1D1),
            button: Color::from_rgb_u32(0x22211E),
            button_hover: Color::from_rgb_u32(0x393734),
            button_active: Color::from_rgb_u32(0x1b1a18),
        }
    }
}

pub static PALETTE: LazyLock<Palette> = LazyLock::new(|| Palette::default());
