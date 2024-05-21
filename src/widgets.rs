use std::{ops::RangeInclusive, rc::Rc};

use cosmic::{
    iced::{
        gradient::{ColorStop, Linear},
        Border, Color, Length, Radians,
    },
    theme,
    widget::{
        self,
        slider::{self, HandleShape, RailBackground},
    },
};

pub struct ColorBlock {
    color: Color,
    border: [bool; 4],
    width: Length,
    height: Length,
}

impl ColorBlock {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            border: [true; 4],
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    pub fn border(mut self, border: [bool; 4]) -> Self {
        self.border = border;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message: 'a> From<ColorBlock> for cosmic::Element<'a, Message> {
    fn from(value: ColorBlock) -> Self {
        let ColorBlock {
            color,
            border,
            width,
            height,
        } = value;

        widget::container(widget::vertical_space(height))
            .style(theme::Container::custom(move |theme| {
                let cosmic = theme.cosmic();
                let radius = cosmic.corner_radii.radius_xs;

                cosmic::iced_style::container::Appearance {
                    background: Some(value.color.into()),
                    border: Border {
                        radius: [
                            radius[0] * (border[0] as i8) as f32,
                            radius[1] * (border[1] as i8) as f32,
                            radius[2] * (border[2] as i8) as f32,
                            radius[3] * (border[3] as i8) as f32,
                        ]
                        .into(),
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }))
            .width(width)
            .into()
    }
}

pub fn color_block(color: Color) -> ColorBlock {
    ColorBlock::new(color)
}

pub fn color_slider<'a, Message>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
    color_stops: &'static [ColorStop],
) -> cosmic::Element<'a, Message>
where
    Message: Clone + 'a,
{
    widget::slider(range, value, on_change)
        .step(0.001)
        .style(theme::iced::Slider::Custom {
            active: Rc::new(|t| {
                let cosmic = t.cosmic();
                let mut a = slider::StyleSheet::active(t, &theme::iced::Slider::default());
                a.rail.colors = RailBackground::Gradient {
                    gradient: Linear::new(Radians(0.0)).add_stops(color_stops.iter().cloned()),
                    auto_angle: true,
                };
                a.rail.width = 8.0;
                a.handle.color = Color::TRANSPARENT;
                a.handle.shape = HandleShape::Circle { radius: 8.0 };
                a.handle.border_color = cosmic.palette.neutral_10.into();
                a.handle.border_width = 4.0;
                a
            }),
            hovered: Rc::new(|t| {
                let cosmic = t.cosmic();
                let mut a = slider::StyleSheet::active(t, &theme::iced::Slider::default());
                a.rail.colors = RailBackground::Gradient {
                    gradient: Linear::new(Radians(0.0)).add_stops(color_stops.iter().cloned()),
                    auto_angle: true,
                };
                a.rail.width = 8.0;
                a.handle.color = Color::TRANSPARENT;
                a.handle.shape = HandleShape::Circle { radius: 8.0 };
                a.handle.border_color = cosmic.palette.neutral_10.into();
                a.handle.border_width = 4.0;
                a
            }),
            dragging: Rc::new(|t| {
                let cosmic = t.cosmic();
                let mut a = slider::StyleSheet::active(t, &theme::iced::Slider::default());
                a.rail.colors = RailBackground::Gradient {
                    gradient: Linear::new(Radians(0.0)).add_stops(color_stops.iter().cloned()),
                    auto_angle: true,
                };
                a.rail.width = 8.0;
                a.handle.color = Color::TRANSPARENT;
                a.handle.shape = HandleShape::Circle { radius: 8.0 };
                a.handle.border_color = cosmic.palette.neutral_10.into();
                a.handle.border_width = 4.0;
                a
            }),
        })
        .into()
}
