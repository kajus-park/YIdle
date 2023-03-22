#![windows_subsystem = "windows"]
use eframe::{egui, App, NativeOptions};
use egui::CentralPanel;
use jk_ban::BigNum;


fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Clicker",
        options,
        Box::new(|_cc| Box::<IdleGame>::default()),
    )
}
#[derive(Default)]
enum Screen {
    #[default]
    Click,
}

#[derive(Default)]
struct IdleGame {
    clicks: BigNum,
    open_screen: Screen,
    clicker1: BigNum,  
    update_thread: Option<std::thread::JoinHandle<()>>,
}

impl App for IdleGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.open_screen {
            Screen::Click => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Click to your hearts content!");
                        ui.separator();
                        ui.label(format!("Clicks: {}", self.clicks));
                        ui.label(format!("Clickers: {}", self.clicker1));
                        ui.separator();
                        if ui.button("Click").clicked() {
                            self.clicks.increment();
                        }
                        if ui.button("+1 Clicker").clicked() {
                            self.clicker1.increment();
                        }
                        if ui.button("Square").clicked() {
                            self.clicks = self.clicks.pow(2.0);
                        }
                        if ui.button("Square Clicker").clicked() {
                            self.clicker1 = self.clicker1.pow(2.0);
                        }
                    });
                });
            }
        }

        self.clicks = self.clicks + self.clicker1;
        let ctx_clone = ctx.clone();
        if let None = self.update_thread {
            self.update_thread = Some(std::thread::spawn(move || loop {
                ctx_clone.request_repaint();
            }))
        }
    }
}
