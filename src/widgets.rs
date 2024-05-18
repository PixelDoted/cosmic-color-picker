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

pub fn color_block<'a, Message>(color: Color, border: [bool; 4]) -> cosmic::Element<'a, Message>
where
    Message: Clone + 'a,
{
    widget::container(widget::vertical_space(100.0))
        .style(theme::Container::custom(move |theme| {
            let cosmic = theme.cosmic();
            let radius = cosmic.corner_radii.radius_xs;

            cosmic::iced_style::container::Appearance {
                background: Some(color.into()),
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
        .width(Length::Fill)
        .into()
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
