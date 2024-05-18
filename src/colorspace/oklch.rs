use std::ops::Deref;

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};
use once_cell::sync::Lazy;

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
const COLOR_STOPS_CHROMA: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_HUE: [ColorStop; 7] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.92, 0.0, 0.22),
    },
    ColorStop {
        offset: 0.0812,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 0.304,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.395,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.541,
        color: Color::from_rgb(0.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 0.733,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 0.912,
        color: Color::from_rgb(1.0, 0.0, 1.0),
    },
];

#[derive(Clone)]
pub struct OKLCH {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl OKLCH {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let lab = super::oklab::rgb_to_oklab(rgb[0], rgb[1], rgb[2]);
        let mut lch = [
            lab[0],
            (lab[1] * lab[1] + lab[2] * lab[2]).sqrt(),
            lab[2].atan2(lab[1]).to_degrees(),
        ];

        if lch[2] < 0.0 {
            lch[2] = 360.0 + lch[2];
        }

        Self {
            strings: [lch[0].to_string(), lch[1].to_string(), lch[2].to_string()],
            values: lch,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        let h = self.values[2].to_radians();
        let a = self.values[1] * h.cos();
        let b = self.values[1] * h.sin();

        super::oklab::oklab_to_rgb(self.values[0], a, b)
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}, {}, {}", self.values[0], self.values[1], self.values[2])
    }
}

impl OKLCH {
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
                0f32..=1.0f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_LIGHTNESS,
            ))
            .spacing(10.0)
            .padding(10.0);
        let chroma = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("chroma")).size(20.0))
                    .push(
                        widget::text_input("", strings[1].clone())
                            .on_input(|string| Message::ChangeString { index: 1, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=0.5f32,
                values[1],
                |value| Message::ChangeValue { index: 1, value },
                &COLOR_STOPS_CHROMA,
            ))
            .spacing(10.0)
            .padding(10.0);
        let hue = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("hue")).size(20.0))
                    .push(
                        widget::text_input("", strings[2].clone())
                            .on_input(|string| Message::ChangeString { index: 2, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=360f32,
                values[2],
                |value| Message::ChangeValue { index: 2, value },
                &COLOR_STOPS_HUE,
            ))
            .spacing(10.0)
            .padding(10.0);

        let content = widget::column::with_capacity(3)
            .push(widget::container(lightness).style(cosmic::style::Container::Card))
            .push(widget::container(chroma).style(cosmic::style::Container::Card))
            .push(widget::container(hue).style(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}
