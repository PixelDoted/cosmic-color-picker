// SPDX-License-Identifier: GPL-3.0-only

mod hsv;
mod oklab;
mod oklch;
mod rgb;

use std::fmt::Display;

pub use hsv::Hsv;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use rgb::Rgb;

use crate::fl;

#[derive(Clone, Debug)]
pub enum ColorSpaceMessage {
    ChangeValue { index: usize, value: f32 },
    ChangeString { index: usize, string: String },
}

#[derive(Clone, Default, Debug)]
pub enum ColorSpaceCombo {
    #[default]
    Rgb,
    Hsv,
    Oklab,
    Oklch,
}

impl Display for ColorSpaceCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSpaceCombo::Rgb => f.write_str(&fl!("rgb")),
            ColorSpaceCombo::Hsv => f.write_str(&fl!("hsv")),
            ColorSpaceCombo::Oklab => f.write_str(&fl!("oklab")),
            ColorSpaceCombo::Oklch => f.write_str(&fl!("oklch")),
        }
    }
}

pub enum ColorSpace {
    Rgb(Rgb),
    Hsv(Hsv),
    Oklab(Oklab),
    Oklch(Oklch),
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::Rgb(Rgb::from_rgb([1.0; 3]))
    }
}

impl ColorSpace {
    pub fn change_value(&mut self, index: usize, value: f32) {
        match self {
            ColorSpace::Rgb(rgb) => rgb.change_value(index, value),
            ColorSpace::Hsv(hsv) => hsv.change_value(index, value),
            ColorSpace::Oklab(oklab) => oklab.change_value(index, value),
            ColorSpace::Oklch(oklch) => oklch.change_value(index, value),
        }
    }

    pub fn change_string(&mut self, index: usize, string: String) {
        match self {
            ColorSpace::Rgb(rgb) => rgb.change_string(index, string),
            ColorSpace::Hsv(hsv) => hsv.change_string(index, string),
            ColorSpace::Oklab(oklab) => oklab.change_string(index, string),
            ColorSpace::Oklch(oklch) => oklch.change_string(index, string),
        }
    }
}

impl ColorSpace {
    pub fn to_rgb(&self) -> ColorSpace {
        let rgb = self.get_rgb();
        Self::Rgb(rgb::Rgb::from_rgb(rgb))
    }

    pub fn to_hsv(&self) -> ColorSpace {
        let rgb = self.get_rgb();
        Self::Hsv(hsv::Hsv::from_rgb(rgb))
    }

    pub fn to_oklab(&self) -> ColorSpace {
        let rgb = self.get_rgb();
        Self::Oklab(oklab::Oklab::from_rgb(rgb))
    }

    pub fn to_oklch(&self) -> ColorSpace {
        let rgb = self.get_rgb();
        Self::Oklch(oklch::Oklch::from_rgb(rgb))
    }

    pub fn get_rgb(&self) -> [f32; 3] {
        match self {
            ColorSpace::Rgb(rgb) => rgb.to_rgb(),
            ColorSpace::Hsv(hsv) => hsv.to_rgb(),
            ColorSpace::Oklab(oklab) => oklab.to_rgb(),
            ColorSpace::Oklch(oklch) => oklch.to_rgb(),
        }
    }
}
