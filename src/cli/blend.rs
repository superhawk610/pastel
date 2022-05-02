use pastel::blend::*;
use pastel::Color;

pub fn get_blending_function(blend_mode: &str) -> Box<dyn Fn(&Color, &Color) -> Color> {
    match blend_mode.to_lowercase().as_ref() {
        "multiply" => Box::new(|b: &Color, s: &Color| b.blend::<Multiply>(s)),
        "screen" => Box::new(|b: &Color, s: &Color| b.blend::<Screen>(s)),
        "overlay" => Box::new(|b: &Color, s: &Color| b.blend::<Overlay>(s)),
        _ => unreachable!("Unknown blend mode"),
    }
}
