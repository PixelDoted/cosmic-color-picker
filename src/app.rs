// SPDX-License-Identifier: GPL-3.0-only

use crate::colorspace::{self, ColorSpace};
use crate::fl;
use crate::widgets::ColorBlock;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Color, Length};
use cosmic::widget::nav_bar;
use cosmic::{theme, widget, Application, Element};

#[derive(Default)]
pub struct ColorPicker {
    pub colorspace: ColorSpace,

    nav_model: nav_bar::Model,
    core: Core,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeValue { index: usize, value: f32 },
    ChangeString { index: usize, string: String },
}

impl Application for ColorPicker {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "me.pixeldoted.CosmicColorPicker";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the header of your application, it can be used to display the title of your application.
    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::heading(fl!("app-title")).into()]
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut nav_model = nav_bar::Model::default();
        nav_model.insert().text(fl!("rgb")).data(0);
        nav_model.insert().text(fl!("hsv")).data(1);
        nav_model.insert().text(fl!("oklab")).data(2);
        nav_model.insert().text(fl!("oklch")).data(3);
        nav_model.activate_position(0);

        let example = ColorPicker {
            colorspace: ColorSpace::default(),

            nav_model,
            core,
        };

        (example, Command::none())
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Self::Message> {
        self.nav_model.activate(id);
        match self.nav_model.active_data() {
            Some(0) => self.colorspace.to_rgb(),
            Some(1) => self.colorspace.to_hsv(),
            Some(2) => self.colorspace.to_oklab(),
            Some(3) => self.colorspace.to_oklch(),
            _ => (),
        }

        Command::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangeValue { index, value } => match &mut self.colorspace {
                ColorSpace::RGB(rgb) => rgb.change_value(index, value),
                ColorSpace::HSV(hsv) => hsv.change_value(index, value),
                ColorSpace::OKLAB(oklab) => oklab.change_value(index, value),
                ColorSpace::OKLCH(oklch) => oklch.change_value(index, value),
            },
            Message::ChangeString { index, string } => match &mut self.colorspace {
                ColorSpace::RGB(rgb) => rgb.change_string(index, string),
                ColorSpace::HSV(hsv) => hsv.change_string(index, string),
                ColorSpace::OKLAB(oklab) => oklab.change_string(index, string),
                ColorSpace::OKLCH(oklch) => oklch.change_string(index, string),
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let (rgb, content) = match &self.colorspace {
            ColorSpace::RGB(rgb) => (rgb.to_rgb(), rgb.view()),
            ColorSpace::HSV(hsv) => (hsv.to_rgb(), hsv.view()),
            ColorSpace::OKLAB(oklab) => (oklab.to_rgb(), oklab.view()),
            ColorSpace::OKLCH(oklch) => (oklch.to_rgb(), oklch.view()),
        };

        let color_block = widget::Container::new(ColorBlock::new(
            Color::from_rgb(rgb[0], rgb[1], rgb[2]),
            100.0,
            100.0,
        ))
        .style(theme::Container::Card)
        .padding(10.0);

        widget::container(
            widget::column::with_capacity(2)
                .push(color_block)
                .push(content)
                .spacing(10.0)
                .padding(10.0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
