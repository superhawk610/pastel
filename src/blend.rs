use crate::{Color, RGBA};

/// Blend modes that determine how partially transparent source and backdrop colors
/// interact when they overlap. For more information on color blending, see
/// [here](https://www.w3.org/TR/compositing/#blending) and
/// [here](https://en.wikipedia.org/wiki/Blend_modes).
pub trait BlendMode {
    fn blend_channel(backdrop: u8, source: u8) -> u8;

    fn blend(backdrop: &RGBA<u8>, source: &RGBA<u8>) -> Color {
        let r = Self::blend_channel(backdrop.r, source.r);
        let g = Self::blend_channel(backdrop.g, source.g);
        let b = Self::blend_channel(backdrop.b, source.b);
        let a = (backdrop.alpha + source.alpha) / 2.0;

        Color::from_rgba(r, g, b, a)
    }
}

pub struct Multiply;
pub struct Screen;
pub struct Overlay;

impl BlendMode for Multiply {
    fn blend_channel(backdrop: u8, source: u8) -> u8 {
        ((backdrop as f64 * source as f64) / 255.0).floor() as u8
    }
}

impl BlendMode for Screen {
    fn blend_channel(backdrop: u8, source: u8) -> u8 {
        let backdrop = backdrop as f64 / 255.0;
        let source = source as f64 / 255.0;
        ((1.0 - (1.0 - backdrop) * (1.0 - source)) * 255.0) as u8
    }
}

impl BlendMode for Overlay {
    fn blend_channel(backdrop: u8, source: u8) -> u8 {
        let backdrop = backdrop as f64 / 255.0;
        let source = source as f64 / 255.0;
        if backdrop < 0.5 {
            ((2.0 * backdrop * source) * 255.0) as u8
        } else {
            ((1.0 - 2.0 * (1.0 - backdrop) * (1.0 - source)) * 255.0) as u8
        }
    }
}
