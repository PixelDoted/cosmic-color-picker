use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};

use crate::{app::Message, fl, widgets::color_slider};

const COLOR_STOPS_HUE: [ColorStop; 7] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 0.166,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.333,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.5,
        color: Color::from_rgb(0.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 0.666,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 0.833,
        color: Color::from_rgb(1.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_SATURATION: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_VALUE: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
];

#[derive(Clone)]
pub struct HSV {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl HSV {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let x_max = rgb[0].max(rgb[1]).max(rgb[2]);
        let x_min = rgb[0].min(rgb[1]).min(rgb[2]);
        let c = x_max - x_min;
        let h = if c == 0.0 {
            0.0
        } else if x_max == rgb[0] {
            60.0 * ((rgb[1] - rgb[2]) / c % 6.0)
        } else if x_max == rgb[1] {
            60.0 * ((rgb[2] - rgb[0]) / c + 2.0)
        } else if x_max == rgb[2] {
            60.0 * ((rgb[0] - rgb[1]) / c + 4.0)
        } else {
            // Default to (c = 0)
            0.0
        };

        let s = if x_max == 0.0 { 0.0 } else { c / x_max };

        Self {
            strings: [h.to_string(), s.to_string(), x_max.to_string()],
            values: [h, s, x_max],
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        let c = self.values[2] * self.values[1];
        let h_ = self.values[0] / 60.0;
        let x = c * (1.0 - (h_ % 2.0 - 1.0).abs());

        let (r1, g1, b1) = if 0.0 <= h_ && h_ < 1.0 {
            (c, x, 0.0)
        } else if 1.0 <= h_ && h_ < 2.0 {
            (x, c, 0.0)
        } else if 2.0 <= h_ && h_ < 3.0 {
            (0.0, c, x)
        } else if 3.0 <= h_ && h_ < 4.0 {
            (0.0, x, c)
        } else if 4.0 <= h_ && h_ < 5.0 {
            (x, 0.0, c)
        } else {
            // otherwise (5.0 <= h' < 6.0)
            (c, 0.0, x)
        };

        let m = self.values[2] - c;
        [r1 + m, g1 + m, b1 + m]
    }
}

impl HSV {
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
                    .push(widget::text(fl!("hue")).size(20.0))
                    .push(
                        widget::text_input("", strings[0].clone())
                            .on_input(|string| Message::ChangeString { index: 0, string }),
                    )
                    .align_items(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=360f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_HUE,
            ))
            .spacing(10.0)
            .padding(10.0);
        let green = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("saturation")).size(20.0))
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
                &COLOR_STOPS_SATURATION,
            ))
            .spacing(10.0)
            .padding(10.0);
        let blue = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("value")).size(20.0))
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
                &COLOR_STOPS_VALUE,
            ))
            .spacing(10.0)
            .padding(10.0);

        let content = widget::column::with_capacity(3)
            .push(
                widget::container(red)
                    .style(cosmic::style::Container::Card)
                    .max_width(300.0)
                    .max_height(300.0),
            )
            .push(
                widget::container(green)
                    .style(cosmic::style::Container::Card)
                    .max_width(300.0)
                    .max_height(300.0),
            )
            .push(
                widget::container(blue)
                    .style(cosmic::style::Container::Card)
                    .max_width(300.0)
                    .max_height(300.0),
            )
            .spacing(10.0)
            .padding(10.0);

        content.into()
    }
}
