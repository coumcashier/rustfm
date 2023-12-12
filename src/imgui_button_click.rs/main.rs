fn main() {
    // let mut app = AppGui::new();
    let mut app = app_gui::AppGui::new();
    let app_arc = Box::new(app);

    imgui_main_loop::start_gui(app_arc);
}

mod app_gui;
mod imgui_main_loop;
mod core;
