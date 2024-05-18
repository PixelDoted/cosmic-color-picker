// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};

use crate::{colorspace::ColorSpaceMessage as Message, fl, widgets::color_slider};

const COLOR_STOPS_LIGHTNESS: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
];
const COLOR_STOPS_GREEN_RED: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_BLUE_YELLOW: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
];

#[derive(Clone)]
pub struct OKLAB {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl OKLAB {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);

        Self {
            strings: [lab[0].to_string(), lab[1].to_string(), lab[2].to_string()],
            values: lab,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        oklab_to_rgb(self.values[0], self.values[1], self.values[2])
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}, {}, {}", self.values[0], self.values[1], self.values[2])
    }
}

impl OKLAB {
    pub fn change_value(&mut self, index: usize, value: f32) {
        self.values[index] = value;
        self.strings[index] = value.to_string();
    }

    pub fn change_string(&mut self, index: usize, string: String) {
        if let Ok(value) = string.parse::<f32>() {
            self.values[index] = value;
        }

        self.strings[index] = string;
    }

    pub fn view<'a>(&self) -> cosmic::Element<'a, Message> {
        let values = &self.values;
        let strings = &self.strings;

        let lightness = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("lightness")).size(20.0))
                    .push(
                        widget::text_input("", strings[0].clone())
                            .on_input(|string| Message::ChangeString { index: 0, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_LIGHTNESS,
            ))
            .spacing(10.0)
            .padding(10.0);
        let green_red = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("green-red")).size(20.0))
                    .push(
                        widget::text_input("", strings[1].clone())
                            .on_input(|string| Message::ChangeString { index: 1, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                -0.5..=0.5,
                values[1],
                |value| Message::ChangeValue { index: 1, value },
                &COLOR_STOPS_GREEN_RED,
            ))
            .spacing(10.0)
            .padding(10.0);
        let blue_yellow = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("blue-yellow")).size(20.0))
                    .push(
                        widget::text_input("", strings[2].clone())
                            .on_input(|string| Message::ChangeString { index: 2, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                -0.5..=0.5,
                values[2],
                |value| Message::ChangeValue { index: 2, value },
                &COLOR_STOPS_BLUE_YELLOW,
            ))
            .spacing(10.0)
            .padding(10.0);

        let content = widget::column::with_capacity(3)
            .push(widget::container(lightness).style(cosmic::style::Container::Card))
            .push(widget::container(green_red).style(cosmic::style::Container::Card))
            .push(widget::container(blue_yellow).style(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}

// https://bottosson.github.io/posts/oklab/
pub fn oklab_to_rgb(l: f32, a: f32, b: f32) -> [f32; 3] {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    [
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ]
}

// https://bottosson.github.io/posts/oklab/
pub fn rgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    [
        0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
    ]
}
