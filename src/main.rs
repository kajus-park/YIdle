use eframe::egui;
use eframe::App;
use eframe::NativeOptions;
use egui::CentralPanel;
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
    clicks: u128,
    open_screen: Screen,
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
                        if ui.button("Click").clicked() {
                            self.clicks += 1;
                        }
                    });
                });
            }
        }
    }
}
