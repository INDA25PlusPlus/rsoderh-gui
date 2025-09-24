use ggez::graphics;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{self, Transform},
};

use crate::chess_game::PieceKind;
use crate::chess_game::{self};

macro_rules! load_png_unwrap {
    ($path: expr, $ctx: expr $(,)?) => {
        graphics::Image::from_bytes($ctx, include_bytes!($path)).expect(concat!(
            "expect path '",
            stringify!(path),
            "' to contain a valid PNG"
        ))
    };
}

#[allow(unused_macros)]
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
    #[allow(dead_code)]
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

#[allow(dead_code)]
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
        // let width = 512;
        // let options = usvg::Options::default();

        println!("Loading assets..");

        let assets = Self {
            white: ChessPieces {
                pawn: load_png_unwrap!("../assets/pieces/pw.png", ctx),
                knight: load_png_unwrap!("../assets/pieces/nw.png", ctx),
                bishop: load_png_unwrap!("../assets/pieces/bw.png", ctx),
                rook: load_png_unwrap!("../assets/pieces/rw.png", ctx),
                queen: load_png_unwrap!("../assets/pieces/qw.png", ctx),
                king: load_png_unwrap!("../assets/pieces/kw.png", ctx),
            },
            black: ChessPieces {
                pawn: load_png_unwrap!("../assets/pieces/pb.png", ctx),
                knight: load_png_unwrap!("../assets/pieces/nb.png", ctx),
                bishop: load_png_unwrap!("../assets/pieces/bb.png", ctx),
                rook: load_png_unwrap!("../assets/pieces/rb.png", ctx),
                queen: load_png_unwrap!("../assets/pieces/qb.png", ctx),
                king: load_png_unwrap!("../assets/pieces/kb.png", ctx),
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
