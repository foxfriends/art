use super::color::Color;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::TRANSPARENT; width * height],
        }
    }
}
