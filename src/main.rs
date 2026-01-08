use chrono::prelude::*;
use eframe::egui;
use std::time::Duration;
use tokio::sync::mpsc;
use weather::*;

mod weather;

struct MyApp {
    localtime: DateTime<Local>,
    weather: Option<Weather>,
    weather_receiver: mpsc::Receiver<Weather>,
}

impl MyApp {
    fn new(egui_context: &egui::Context) -> Self {
        // 天気データを送受信するためのチャネルを作成
        // 1 はバッファサイズ（1つだけ保持できる）
        let (weather_sender, weather_receiver) = mpsc::channel(1);
        let ctx = egui_context.clone();

        // バックグラウンドタスクで定期的に天気を取得
        tokio::spawn(async move {
            loop {
                // 天気を取得
                match get_weather().await {
                    Ok(weather) => {
                        // メインスレッドに送信
                        let _ = weather_sender.send(weather).await;
                        // 画面を再描画するよう通知
                        ctx.request_repaint();
                    }
                    Err(error) => {
                        eprintln!("天気の取得に失敗しました: {}", error);
                    }
                }
                // 10分間待機
                tokio::time::sleep(Duration::from_secs(600)).await;
            }
        });

        Self {
            localtime: Local::now(),
            weather: None,
            weather_receiver,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // バックグラウンドタスクから天気データを受け取る
        if let Ok(weather) = self.weather_receiver.try_recv() {
            self.weather = Some(weather);
        }

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

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Home Clock")
            .with_inner_size([800.0, 480.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Clock",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(&cc.egui_ctx)))),
    );
}
