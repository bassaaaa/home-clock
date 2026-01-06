use chrono::prelude::*;
use eframe::egui;

struct MyApp {
    localtime: DateTime<Local>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            localtime: Local::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // localtimeを更新
        self.localtime = Local::now();

        // 次の分の開始時刻まで待つ
        let seconds_until_next_minute = 60 - self.localtime.second();
        let wait_time = if seconds_until_next_minute == 0 {
            60 // ちょうど分の切り替わりの場合は60秒待つ
        } else {
            seconds_until_next_minute
        };
        
        // 再描画
        ctx.request_repaint_after(std::time::Duration::from_secs(wait_time as u64));

        let date_str = self.localtime.format("%Y.%m.%d %a").to_string();
        let time_str = self.localtime.format("%H:%M").to_string();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.heading(egui::RichText::new(date_str).size(50.0));
                ui.heading(egui::RichText::new(time_str).size(250.0));
            })
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Home Clock")
            .with_inner_size([800.0, 480.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Clock",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}
