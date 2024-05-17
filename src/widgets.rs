use std::{ops::RangeInclusive, rc::Rc, sync::Arc};

use cosmic::{
    iced::{
        gradient::{ColorStop, Linear},
        Border, Color, Length, Radians, Size,
    },
    iced_core::{layout, renderer, Shadow},
    iced_widget::row,
    theme,
    widget::{
        self,
        slider::{self, HandleShape, RailBackground},
    },
    Element, Theme,
};

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

pub struct ColorBlock {
    color: Color,
    width: f32,
    height: f32,
}

impl ColorBlock {
    pub fn new(color: Color, width: f32, height: f32) -> Self {
        Self {
            color,
            width,
            height,
        }
    }
}

impl<Message, Theme, Renderer> widget::Widget<Message, Theme, Renderer> for ColorBlock
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut cosmic::iced_core::widget::Tree,
        _renderer: &Renderer,
        _limits: &cosmic::iced_core::layout::Limits,
    ) -> cosmic::iced_core::layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        _tree: &cosmic::iced_core::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &cosmic::iced_core::renderer::Style,
        layout: cosmic::iced_core::Layout<'_>,
        _cursor: cosmic::iced_core::mouse::Cursor,
        _viewport: &cosmic::iced::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::default(),
                shadow: Shadow::default(),
            },
            self.color,
        );
    }
}

impl<'a, Message> From<ColorBlock> for Element<'a, Message> {
    fn from(value: ColorBlock) -> Self {
        Self::new(value)
    }
}
