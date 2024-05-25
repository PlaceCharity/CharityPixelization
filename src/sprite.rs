use crate::Color;

#[derive(Clone, Default)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Sprite {
    #[must_use]
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        self.data.get(y * self.width + x).copied()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if let Some(col) = self.data.get_mut(y * self.width + x) {
            *col = color;
        }
    }
}
