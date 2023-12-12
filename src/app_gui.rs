use async_std::stream::StreamExt;
use async_std::task::JoinHandle;
use glow::HasContext;
use glutin::event;
use glutin::event_loop::{self, EventLoop, EventLoopProxy, EventLoopWindowTarget};
use imgui::internal::RawCast;
use imgui_winit_support::WinitPlatform;
use log::*;
use std::fs::{self, DirEntry};
use std::io::Error;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::core::App;
use crate::imgui_main_loop::Window;

pub(crate) struct AppGui {
    coreApp: App,
    list_dir_join_handle: Option<std::thread::JoinHandle<Vec<std::fs::DirEntry>>>,
    dir_listing: Option<Vec<std::fs::DirEntry>>,
}

impl AppGui {
    pub(crate) fn new() -> Self {
        Self {
            coreApp: App::new(),
            list_dir_join_handle: None,
            dir_listing: None,
        }
    }
    pub(crate) fn render(
        &mut self,
        ig_renderer: &mut imgui_glow_renderer::AutoRenderer,
        imgui_context: &mut imgui::Context,
        window: &Window,
        winit_platform: &mut WinitPlatform,
        proxy: Arc<Mutex<EventLoopProxy<()>>>,
    ) {
        // The renderer assumes you'll be clearing the buffer yourself
        unsafe { ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

        let ui = imgui_context.frame();
        let frame_size = ui.io().display_size;
        let token_item_padding_zero = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
        let token_item_spacing_zero = ui.push_style_var(imgui::StyleVar::ItemSpacing([0.0, 0.0]));
        ui.window("hello")
            .title_bar(false)
            .resizable(false)
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(frame_size, imgui::Condition::Always)
            .build(|| {
                let size = ui.content_region_avail();
                let panel_size = [size[0] / 3.0, size[1]];
                let cursor = ui.cursor_pos();
                let panel_pos = [0.0, cursor[1]];

                ui.window("#panel_left")
                    .title_bar(false)
                    .resizable(false)
                    .position(panel_pos, imgui::Condition::Always)
                    .size(panel_size, imgui::Condition::Always)
                    .build(|| {
                        ui.button("hi");
                    });

                ui.window("#panel_mid")
                    .title_bar(false)
                    .resizable(false)
                    .position([size[0] / 3.0, cursor[1]], imgui::Condition::Always)
                    .size(panel_size, imgui::Condition::Always)
                    .build(|| {
                        if self.list_dir_join_handle.is_none() && self.dir_listing.is_none() {
                            let mut paths = self.coreApp.list_dir();
                            let proxy_clone = proxy.clone();
                            let handle = std::thread::spawn(move || {
                                let mut paths = paths;
                                std::thread::sleep(std::time::Duration::from_secs(1));
                                match paths {
                                    Ok(paths) => {
                                        let mut vec = vec![];
                                        for p in paths {
                                            match p {
                                                Ok(p) => {
                                                    vec.push(p);
                                                }
                                                Err(e) => {
                                                    continue;
                                                }
                                            }
                                        }
                                        proxy_clone.lock().unwrap().send_event(()).unwrap();
                                        vec
                                    }
                                    Err(e) => {
                                        warn!("Error reading directory: {}", e);
                                        proxy_clone.lock().unwrap().send_event(()).unwrap();
                                        return Vec::new();
                                    }
                                }
                            });
                            self.list_dir_join_handle = Some(handle);
                            ui.text("Loading...");
                        } else if !self.list_dir_join_handle.is_none() {
                            if self.list_dir_join_handle.as_ref().unwrap().is_finished() {
                                let handle = self.list_dir_join_handle.take();
                                self.list_dir_join_handle = None;
                                let paths = handle.unwrap().join().unwrap();
                                self.dir_listing = Some(paths);
                            } else {
                                ui.text("Loading...");
                            }
                        }

                        if !self.dir_listing.is_none() {
                            if ui.button("../") {
                                if (self.coreApp.parent_path() != self.coreApp.current_path) {
                                    self.coreApp.change_path(self.coreApp.parent_path());
                                    self.dir_listing = None;
                                    self.list_dir_join_handle = None;
                                    proxy.lock().unwrap().send_event(()).unwrap();
                                }
                            }
                        }

                        let mut change_dir = false;
                        if !self.dir_listing.as_ref().is_none() {
                            for p in self.dir_listing.as_ref().unwrap() {
                                if ui.button(p.file_name().to_str().unwrap()) {
                                    if p.path().is_dir() {
                                        self.coreApp.change_path(p.path());
                                        change_dir = true;
                                        self.list_dir_join_handle = None;
                                    }
                                    proxy.lock().unwrap().send_event(()).unwrap();
                                };
                            }
                        }
                        if (change_dir == true) {
                            self.dir_listing = None;
                        }
                    });

                ui.window("#panel_right")
                    .title_bar(false)
                    .resizable(false)
                    .position([size[0] * 2.0 / 3.0, cursor[1]], imgui::Condition::Always)
                    .size(panel_size, imgui::Condition::Always)
                    .build(|| {
                        ui.button("hi");
                    });
            });
        token_item_spacing_zero.pop();
        token_item_padding_zero.pop();
        winit_platform.prepare_render(ui, window.window());
        let draw_data = imgui_context.render();

        // This is the only extra render step to add
        ig_renderer
            .render(draw_data)
            .expect("error rendering imgui");

        window.swap_buffers().unwrap();
    }
}
