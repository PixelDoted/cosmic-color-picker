mod hsv;
mod oklab;
mod oklch;
mod rgb;

#[derive(Clone)]
pub enum ColorSpace {
    RGB(rgb::RGB),
    HSV(hsv::HSV),
    OKLAB(oklab::OKLAB),
    OKLCH(oklch::OKLCH),
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::RGB(rgb::RGB {
            values: [1.0; 3],
            strings: ["1".into(), "1".into(), "1".into()],
        })
    }
}

impl ColorSpace {
    pub fn to_rgb(&mut self) {
        let rgb = self.get_rgb();
        *self = Self::RGB(rgb::RGB::from_rgb(rgb));
    }

    pub fn to_hsv(&mut self) {
        let rgb = self.get_rgb();
        *self = Self::HSV(hsv::HSV::from_rgb(rgb));
    }

    pub fn to_oklab(&mut self) {
        let rgb = self.get_rgb();
        *self = Self::OKLAB(oklab::OKLAB::from_rgb(rgb));
    }

    pub fn to_oklch(&mut self) {
        let rgb = self.get_rgb();
        *self = Self::OKLCH(oklch::OKLCH::from_rgb(rgb));
    }

    fn get_rgb(&self) -> [f32; 3] {
        match self {
            ColorSpace::RGB(rgb) => rgb.to_rgb(),
            ColorSpace::HSV(hsv) => hsv.to_rgb(),
            ColorSpace::OKLAB(oklab) => oklab.to_rgb(),
            ColorSpace::OKLCH(oklch) => oklch.to_rgb(),
        }
    }
}
