use eframe::egui;
use eframe::App;
use eframe::NativeOptions;
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
}

#[test]
fn test_ex(){
    let mut i = BigNum::new(5,0);
    i = i.pow(2.0);
    assert_eq!(i, BigNum::new(25, 0));
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
                            self.clicks.increment();
                        }
                        if ui.button("Double").clicked() {
                            self.clicks=self.clicks*2u128.into();
                        }
                        if ui.button("Square").clicked() {
                            self.clicks=self.clicks.pow(2.0);
                        }
                    });
                });
            }
        }
    }
}
