#![feature(iter_advance_by)]
#![feature(box_patterns)]
#![feature(core_intrinsics)]
#![feature(type_alias_impl_trait)]
use std::{sync::Mutex, io::Write};
use chrono;
use env_logger::{self, fmt::Color};
use log::Level;

fn main() {
    env_logger::Builder::from_default_env()
    .format(|buf, record| {
        let level = record.level();
        let mut style = buf.style();
        match record.level() {
            Level::Error => style.set_color(Color::Red),
            Level::Warn => style.set_color(Color::Yellow),
            Level::Info => style.set_color(Color::Green),
            Level::Debug => style.set_color(Color::Blue),
            Level::Trace => style.set_color(Color::Cyan),
        };


        writeln!(buf, 
            "{}:{} {} [{}] - {}",
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            style.value(level),
            record.args()
        )
            

    })
    .init();

    // let mut app = AppGui::new();
    let mut app = app_gui::AppGui::new();
    let app_arc = Box::new(app);

    imgui_main_loop::start_gui(app_arc);
}

mod app_gui;
mod imgui_main_loop;
mod core;
