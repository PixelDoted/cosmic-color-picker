mod hsv;
mod oklab;
mod oklch;
mod rgb;

use std::fmt::Display;

pub use hsv::HSV;
pub use oklab::OKLAB;
pub use oklch::OKLCH;
pub use rgb::RGB;

#[derive(Clone, Debug)]
pub enum ColorSpaceMessage {
    ChangeValue { index: usize, value: f32 },
    ChangeString { index: usize, string: String },
}

#[derive(Clone, Default, Debug)]
pub enum ColorSpaceCombo {
    #[default]
    RGB,
    HSV,
    OKLAB,
    OKLCH,
}

impl Display for ColorSpaceCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSpaceCombo::RGB => f.write_str("RGB"),
            ColorSpaceCombo::HSV => f.write_str("HSV"),
            ColorSpaceCombo::OKLAB => f.write_str("OKLAB"),
            ColorSpaceCombo::OKLCH => f.write_str("OKLCH"),
        }
    }
}

pub enum ColorSpace {
    RGB(RGB),
    HSV(HSV),
    OKLAB(OKLAB),
    OKLCH(OKLCH),
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
