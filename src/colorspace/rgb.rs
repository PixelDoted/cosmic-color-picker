// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};

use crate::{colorspace::ColorSpaceMessage as Message, fl, widgets::color_slider};

const COLOR_STOPS_RED: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_GREEN: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
];
const COLOR_STOPS_BLUE: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
];

#[derive(Clone)]
pub struct RGB {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl Default for RGB {
    fn default() -> Self {
        Self {
            values: [1.0; 3],
            strings: ["1".into(), "1".into(), "1".into()],
        }
    }
}

impl RGB {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        Self {
            strings: [rgb[0].to_string(), rgb[1].to_string(), rgb[2].to_string()],
            values: rgb,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        self.values.clone()
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}, {}, {}", self.values[0], self.values[1], self.values[2])
    }
}

impl RGB {
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

        let red = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("red")).size(20.0))
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
                &COLOR_STOPS_RED,
            ))
            .spacing(10.0)
            .padding(10.0);
        let green = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("green")).size(20.0))
                    .push(
                        widget::text_input("", strings[1].clone())
                            .on_input(|string| Message::ChangeString { index: 1, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[1],
                |value| Message::ChangeValue { index: 1, value },
                &COLOR_STOPS_GREEN,
            ))
            .spacing(10.0)
            .padding(10.0);
        let blue = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("blue")).size(20.0))
                    .push(
                        widget::text_input("", strings[2].clone())
                            .on_input(|string| Message::ChangeString { index: 2, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[2],
                |value| Message::ChangeValue { index: 2, value },
                &COLOR_STOPS_BLUE,
            ))
            .spacing(10.0)
            .padding(10.0);

        let content = widget::column::with_capacity(3)
            .push(widget::container(red).style(cosmic::style::Container::Card))
            .push(widget::container(green).style(cosmic::style::Container::Card))
            .push(widget::container(blue).style(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}
