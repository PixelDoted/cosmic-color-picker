// SPDX-License-Identifier: GPL-3.0-only

use crate::colorspace::{ColorSpace, ColorSpaceCombo, ColorSpaceMessage};
use crate::fl;
use crate::widgets::color_block;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::{clipboard, Length};
use cosmic::iced::{event, keyboard::Event as KeyEvent, Color, Event, Subscription};
use cosmic::iced_widget::scrollable::{Direction, Properties};
use cosmic::{theme, widget, Application, Element};
use log::info;

pub struct ColorPicker {
    pub spaces: Vec<ColorSpace>,
    last_edited: usize,

    colorspace_combo: widget::combo_box::State<ColorSpaceCombo>,
    core: Core,
}

#[derive(Debug, Clone)]
pub enum Message {
    ColorSpace {
        index: usize,
        message: ColorSpaceMessage,
    },
    ChangeColorSpace {
        index: usize,
        selected: ColorSpaceCombo,
    },
    AddSpace,
    RemoveSpace(usize),

    CopyToClipboard(usize),
    Key(Key, Modifiers),
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

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::heading(fl!("app-title")).into()]
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let example = ColorPicker {
            spaces: vec![ColorSpace::default()],
            last_edited: 0,

            colorspace_combo: widget::combo_box::State::new(vec![
                ColorSpaceCombo::RGB,
                ColorSpaceCombo::HSV,
                ColorSpaceCombo::OKLAB,
                ColorSpaceCombo::OKLCH,
            ]),
            core,
        };

        (example, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ColorSpace { index: i, message } => match message {
                ColorSpaceMessage::ChangeValue { index, value } => match &mut self.spaces[i] {
                    ColorSpace::RGB(rgb) => rgb.change_value(index, value),
                    ColorSpace::HSV(hsv) => hsv.change_value(index, value),
                    ColorSpace::OKLAB(oklab) => oklab.change_value(index, value),
                    ColorSpace::OKLCH(oklch) => oklch.change_value(index, value),
                },
                ColorSpaceMessage::ChangeString { index, string } => match &mut self.spaces[i] {
                    ColorSpace::RGB(rgb) => rgb.change_string(index, string),
                    ColorSpace::HSV(hsv) => hsv.change_string(index, string),
                    ColorSpace::OKLAB(oklab) => oklab.change_string(index, string),
                    ColorSpace::OKLCH(oklch) => oklch.change_string(index, string),
                },
            },
            Message::ChangeColorSpace { index, selected } => match selected {
                ColorSpaceCombo::RGB => self.spaces[index].to_rgb(),
                ColorSpaceCombo::HSV => self.spaces[index].to_hsv(),
                ColorSpaceCombo::OKLAB => self.spaces[index].to_oklab(),
                ColorSpaceCombo::OKLCH => self.spaces[index].to_oklch(),
            },
            Message::AddSpace => {
                self.spaces.push(ColorSpace::default());
            }
            Message::RemoveSpace(index) => {
                self.spaces.remove(index);
            }

            Message::CopyToClipboard(index) => {
                return self.copy_to_clipboard(index);
            }
            Message::Key(key, modifiers) => {
                if modifiers.control() && key == Key::Character("c".into()) {
                    return self.copy_to_clipboard(self.last_edited);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut contents = widget::row::with_capacity(self.spaces.len());

        for (colorspace, index) in self.spaces.iter().zip(0..) {
            let (rgb, content, combo_selection) = match colorspace {
                ColorSpace::RGB(rgb) => (rgb.to_rgb(), rgb.view(), ColorSpaceCombo::RGB),
                ColorSpace::HSV(hsv) => (hsv.to_rgb(), hsv.view(), ColorSpaceCombo::HSV),
                ColorSpace::OKLAB(oklab) => (oklab.to_rgb(), oklab.view(), ColorSpaceCombo::OKLAB),
                ColorSpace::OKLCH(oklch) => (oklch.to_rgb(), oklch.view(), ColorSpaceCombo::OKLCH),
            };

            let min_rgb = rgb[0].min(rgb[1]).min(rgb[2]).min(0.0);
            let max_rgb = rgb[0].max(rgb[1]).max(rgb[2]).max(1.0) - min_rgb;
            let norm_rgb = [
                (rgb[0] - min_rgb) / max_rgb,
                (rgb[1] - min_rgb) / max_rgb,
                (rgb[2] - min_rgb) / max_rgb,
            ];

            let sidebar = widget::Container::new(
                widget::column::with_capacity(3)
                    .push(
                        widget::row::with_capacity(2)
                            .push(color_block(
                                Color::from_rgb(rgb[0], rgb[1], rgb[2]),
                                [true, false, false, true],
                            ))
                            .push(color_block(
                                Color::from_rgb(norm_rgb[0], norm_rgb[1], norm_rgb[2]),
                                [false, true, true, false],
                            )),
                    )
                    .push(
                        widget::row::with_capacity(3)
                            .push(
                                widget::button::icon(widget::icon::from_name("edit-copy-symbolic"))
                                    .on_press(Message::CopyToClipboard(index)),
                            )
                            .push(widget::Space::with_width(Length::Fill))
                            .push(
                                widget::button::icon(widget::icon::from_name(
                                    "user-trash-full-symbolic",
                                ))
                                .on_press(Message::RemoveSpace(index))
                                .style(theme::Button::Destructive),
                            ),
                    )
                    .push(widget::combo_box(
                        &self.colorspace_combo,
                        "",
                        Some(&combo_selection),
                        move |t| Message::ChangeColorSpace { index, selected: t },
                    ))
                    .spacing(10.0),
            )
            .style(theme::Container::Card)
            .padding(10.0);

            contents = contents.push(widget::container(
                widget::column::with_capacity(2)
                    .push(sidebar)
                    .push(content.map(move |message| Message::ColorSpace { index, message }))
                    .spacing(10.0)
                    .padding(10.0)
                    .width(300.0),
            ));
        }

        {
            contents = contents.push(
                widget::container(
                    widget::button::icon(widget::icon::from_name("list-add-symbolic"))
                        .icon_size(32)
                        .on_press(Message::AddSpace),
                )
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(50.0)
                .height(200.0),
            );
        }

        widget::scrollable(contents)
            .direction(Direction::Horizontal(Properties::new()))
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![event::listen_with(|event, status| match event {
            Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => match status {
                event::Status::Ignored => Some(Message::Key(key, modifiers)),
                event::Status::Captured => None,
            },
            _ => None,
        })])
    }
}

impl ColorPicker {
    fn copy_to_clipboard(&self, index: usize) -> Command<Message> {
        let contents = match &self.spaces[index] {
            ColorSpace::RGB(rgb) => rgb.copy_to_clipboard(),
            ColorSpace::HSV(hsv) => hsv.copy_to_clipboard(),
            ColorSpace::OKLAB(oklab) => oklab.copy_to_clipboard(),
            ColorSpace::OKLCH(oklch) => oklch.copy_to_clipboard(),
        };

        info!("Copying \"{}\" to clipboard", contents);
        clipboard::write(contents)
    }
}
