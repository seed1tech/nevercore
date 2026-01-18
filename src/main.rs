slint::include_modules!();
use slint::{Timer, TimerMode, Color};
use std::fs;
use rfd::FileDialog;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    // Обновление LN/COL (базовая логика по тексту)
    ui.on_update_cursor_info({
        let ui_handle = ui_weak.clone();
        move |text, _| {
            if let Some(ui) = ui_handle.upgrade() {
                // В данной реализации Slint TextInput сложно достать индекс курсора без багов
                // Поэтому считаем строки по всему тексту для статуса
                let lines = text.lines().count();
                let last_line_len = text.lines().last().unwrap_or("").len();
                
                ui.set_cur_line(lines as i32);
                ui.set_cur_col(last_line_len as i32 + 1);
            }
        }
    });

    ui.on_open_file({
        let ui_handle = ui_weak.clone();
        move || {
            if let Some(path) = FileDialog::new().add_filter("Seed", &["seed", "txt"]).pick_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    ui_handle.upgrade().unwrap().set_content(content.into());
                }
            }
        }
    });

    ui.on_save_to_file(|content| {
        if let Some(mut path) = FileDialog::new().add_filter("Seed", &["seed"]).save_file() {
            if path.extension().and_then(|s| s.to_str()) != Some("seed") {
                path.set_extension("seed");
            }
            let _ = fs::write(path, content.as_bytes());
        }
    });

    let timer = Timer::default();
    let mut time: f32 = 0.0;
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(16), move || {
        if let Some(ui) = ui_weak.upgrade() {
            time += 0.016;
            let h = (time * 0.2) % 1.0;
            let (r, g, b) = hsv_to_rgb(h, 0.8, 1.0);
            ui.set_logo_color(Color::from_rgb_u8(r, g, b));
        }
    });

    ui.run()
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let i = (h * 6.0) as i32; let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s); let q = v * (1.0 - f * s); let t = v * (1.0 - (1.0 - f) * s);
    match i % 6 {
        0 => ((v*255.0) as u8, (t*255.0) as u8, (p*255.0) as u8),
        1 => ((q*255.0) as u8, (v*255.0) as u8, (p*255.0) as u8),
        2 => ((p*255.0) as u8, (v*255.0) as u8, (t*255.0) as u8),
        3 => ((p*255.0) as u8, (q*255.0) as u8, (v*255.0) as u8),
        4 => ((t*255.0) as u8, (p*255.0) as u8, (v*255.0) as u8),
        _ => ((v*255.0) as u8, (p*255.0) as u8, (q*255.0) as u8),
    }
}