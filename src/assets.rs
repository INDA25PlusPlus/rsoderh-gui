use ggez::graphics;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{self, Transform},
};

use crate::chess_game::PieceKind;
use crate::chess_game::{self};

macro_rules! load_svg_unwrap {
    ($path: expr, $ctx: expr, $width: expr, $options: expr $(,)?) => {
        load_svg($ctx, include_bytes!($path), $width, $options).unwrap_or_else(|err| match err {
            SvgLoadError::ZeroWidth => panic!("expect width to not equal 0"),
            SvgLoadError::SvgParse => panic!(concat!(
                "expect path '",
                stringify!(path),
                "' to contain valid SVG"
            )),
            SvgLoadError::Ggez(err) => panic!("ggez loading error: {:?}", err),
        })
    };
}

#[derive(Debug)]
enum SvgLoadError {
    /// The `width` argument was equal to zero.
    ZeroWidth,
    SvgParse,
    Ggez(ggez::GameError),
}

impl From<usvg::Error> for SvgLoadError {
    fn from(_v: usvg::Error) -> Self {
        Self::SvgParse
    }
}

impl From<ggez::GameError> for SvgLoadError {
    fn from(v: ggez::GameError) -> Self {
        Self::Ggez(v)
    }
}

fn load_svg(
    ctx: &mut ggez::Context,
    data: &[u8],
    width: u32,
    options: &usvg::Options,
) -> Result<graphics::Image, SvgLoadError> {
    let tree = usvg::Tree::from_data(data, options)?;
    let scale = width as f32 / tree.size().width();

    let height = (scale * tree.size().height()).floor() as u32;

    let Some(mut pixmap) = Pixmap::new(width, height) else {
        return Err(SvgLoadError::ZeroWidth);
    };

    resvg::render(
        &tree,
        Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    Ok(graphics::Image::from_pixels(
        ctx,
        pixmap.data(),
        graphics::ImageFormat::Rgba8Unorm,
        pixmap.width(),
        pixmap.height(),
    ))
}

pub struct ChessPieces {
    pawn: graphics::Image,
    knight: graphics::Image,
    bishop: graphics::Image,
    rook: graphics::Image,
    queen: graphics::Image,
    king: graphics::Image,
}

impl ChessPieces {
    pub fn get(&self, kind: PieceKind) -> &graphics::Image {
        match kind {
            PieceKind::Pawn => &self.pawn,
            PieceKind::Knight => &self.knight,
            PieceKind::Bishop => &self.bishop,
            PieceKind::Rook => &self.rook,
            PieceKind::Queen => &self.queen,
            PieceKind::King => &self.king,
        }
    }
}

/// Stores collection of the pre-loaded chess assets used by the game.
pub struct Assets {
    white: ChessPieces,
    black: ChessPieces,
}

impl Assets {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let width = 512;
        let options = usvg::Options::default();

        println!("Loading assets..");

        let assets = Self {
            white: ChessPieces {
                pawn: load_svg_unwrap!("../assets/Chess_plt45.svg", ctx, width, &options),
                knight: load_svg_unwrap!("../assets/Chess_nlt45.svg", ctx, width, &options),
                bishop: load_svg_unwrap!("../assets/Chess_blt45.svg", ctx, width, &options),
                rook: load_svg_unwrap!("../assets/Chess_rlt45.svg", ctx, width, &options),
                queen: load_svg_unwrap!("../assets/Chess_qlt45.svg", ctx, width, &options),
                king: load_svg_unwrap!("../assets/Chess_klt45.svg", ctx, width, &options),
            },
            black: ChessPieces {
                pawn: load_svg_unwrap!("../assets/Chess_pdt45.svg", ctx, width, &options),
                knight: load_svg_unwrap!("../assets/Chess_ndt45.svg", ctx, width, &options),
                bishop: load_svg_unwrap!("../assets/Chess_bdt45.svg", ctx, width, &options),
                rook: load_svg_unwrap!("../assets/Chess_rdt45.svg", ctx, width, &options),
                queen: load_svg_unwrap!("../assets/Chess_qdt45.svg", ctx, width, &options),
                king: load_svg_unwrap!("../assets/Chess_kdt45.svg", ctx, width, &options),
            },
        };

        println!("Loading complete");

        assets
    }

    pub fn piece(&self, color: chess_game::Color, kind: chess_game::PieceKind) -> &graphics::Image {
        match color {
            chess_game::Color::White => self.white.get(kind),
            chess_game::Color::Black => self.black.get(kind),
        }
    }
}
