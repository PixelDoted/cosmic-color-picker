// SPDX-License-Identifier: GPL-3.0-only

use app::ColorPicker;
mod app;
mod colorspace;
mod core;
mod shaders;
mod widgets;

fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<ColorPicker>(settings, ())
}
