// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::colorspace::{ColorSpace, ColorSpaceCombo, ColorSpaceMessage};
use crate::fl;
use crate::widgets::color_block;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::{clipboard, Length};
use cosmic::iced::{event, keyboard::Event as KeyEvent, Color, Event, Subscription};
use cosmic::iced_widget::scrollable::{Direction, Properties};
use cosmic::widget::menu::{self, action::MenuAction, MenuBar};
use cosmic::{theme, widget, Application, ApplicationExt, Apply, Element};
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

    ToggleAboutPage,
    LaunchUrl(String),

    CopyToClipboard(usize),
    Key(Key, Modifiers),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
}

impl MenuAction for Action {
    type Message = Message;

    fn message(&self) -> Message {
        match self {
            Action::About => Message::ToggleAboutPage,
        }
    }
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

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![MenuBar::new(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &HashMap::new(),
                vec![menu::Item::Button(fl!("menu-about"), Action::About)],
            ),
        )])
        .into()]
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::heading(fl!("app-title")).into()]
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = ColorPicker {
            spaces: vec![ColorSpace::default()],
            last_edited: 0,

            colorspace_combo: widget::combo_box::State::new(vec![
                ColorSpaceCombo::Rgb,
                ColorSpaceCombo::Hsv,
                ColorSpaceCombo::Oklab,
                ColorSpaceCombo::Oklch,
                ColorSpaceCombo::Cmyk,
            ]),
            core,
        };

        let command = app.set_window_title(fl!("app-title"));
        (app, command)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ColorSpace { index: i, message } => match message {
                ColorSpaceMessage::ChangeValue { index, value } => {
                    self.spaces[i].change_value(index, value);
                }
                ColorSpaceMessage::ChangeString { index, string } => {
                    self.spaces[i].change_string(index, string);
                }
            },
            Message::ChangeColorSpace { index, selected } => {
                self.spaces[index] = match selected {
                    ColorSpaceCombo::Rgb => self.spaces[index].to_rgb(),
                    ColorSpaceCombo::Hsv => self.spaces[index].to_hsv(),
                    ColorSpaceCombo::Oklab => self.spaces[index].to_oklab(),
                    ColorSpaceCombo::Oklch => self.spaces[index].to_oklch(),
                    ColorSpaceCombo::Cmyk => self.spaces[index].to_cmyk(),
                };
            }
            Message::AddSpace => {
                self.spaces.push(ColorSpace::default());
            }
            Message::RemoveSpace(index) => {
                self.spaces.remove(index);
            }

            Message::ToggleAboutPage => {
                self.core.window.show_context = !self.core.window.show_context;
            }
            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(e) => {
                    log::warn!("Failed to open {:?}: {}", url, e);
                }
            },

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
                ColorSpace::Rgb(rgb) => (rgb.to_rgb(), rgb.view(), ColorSpaceCombo::Rgb),
                ColorSpace::Hsv(hsv) => (hsv.to_rgb(), hsv.view(), ColorSpaceCombo::Hsv),
                ColorSpace::Oklab(oklab) => (oklab.to_rgb(), oklab.view(), ColorSpaceCombo::Oklab),
                ColorSpace::Oklch(oklch) => (oklch.to_rgb(), oklch.view(), ColorSpaceCombo::Oklch),
                ColorSpace::Cmyk(cmyk) => (cmyk.to_rgb(), cmyk.view(), ColorSpaceCombo::Cmyk),
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
                            .push(
                                color_block(Color::from_rgb(rgb[0], rgb[1], rgb[2]))
                                    .border([true, false, false, true])
                                    .height(100.0),
                            )
                            .push(
                                color_block(Color::from_rgb(norm_rgb[0], norm_rgb[1], norm_rgb[2]))
                                    .border([false, true, true, false])
                                    .height(100.0),
                            ),
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
                    .push(
                        content
                            .map(move |message| Message::ColorSpace { index, message })
                            .apply(widget::scrollable),
                    )
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

    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(self.about())
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
            ColorSpace::Rgb(rgb) => rgb.copy_to_clipboard(),
            ColorSpace::Hsv(hsv) => hsv.copy_to_clipboard(),
            ColorSpace::Oklab(oklab) => oklab.copy_to_clipboard(),
            ColorSpace::Oklch(oklch) => oklch.copy_to_clipboard(),
            ColorSpace::Cmyk(cmyk) => cmyk.copy_to_clipboard(),
        };

        info!("Copying \"{}\" to clipboard", contents);
        clipboard::write(contents)
    }

    fn about(&self) -> cosmic::Element<Message> {
        let repository = "https://github.com/PixelDoted/cosmic-color-picker";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash = &hash[0..7];
        let date = env!("VERGEN_GIT_COMMIT_DATE");
        widget::column::with_capacity(4)
            .push(widget::svg(widget::svg::Handle::from_memory(
                &include_bytes!(
                    "../res/icons/hicolor/128x128/apps/me.pixeldoted.CosmicColorPicker.svg"
                )[..],
            )))
            .push(widget::text::title3(fl!("app-title")))
            .push(
                widget::button::link(repository)
                    .on_press(Message::LaunchUrl(repository.to_string()))
                    .padding(0),
            )
            .push(
                widget::button::link(fl!("git-description", hash = short_hash, date = date))
                    .on_press(Message::LaunchUrl(format!(
                        "{}/commits/{}",
                        repository, hash
                    )))
                    .padding(0),
            )
            .into()
    }
}
