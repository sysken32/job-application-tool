use eframe;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std:: {process::Command};

fn main() -> Result<(), eframe::Error> {
    
    let _v: Vec<String> = Vec::new();

    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Elkan's Job Applier App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

//app functionality
struct MyApp {
    v: Vec<String>,
    label_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            v: Vec::new(),
            label_text: String::from("Default"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical_centered(|ui| {
                ui.heading("Job Applier App");
                if ui.add(egui::Button::new("Linkedin")).clicked(){
                    linked_in(&mut self.v);
                    self.label_text = String::from("LinkedIn selected");
                }
                if ui.add(egui::Button::new("ChatGPT")).clicked(){
                    chat_gpt();
                    self.label_text = String::from("ChatGPT opened")
                }
                ui.add(egui::Label::new(&self.label_text));
            });
        });
    }
}

//prompts functionality

pub fn linked_in(v:&mut Vec<String>) {
    let mut ctx = ClipboardContext::new().unwrap();

    v.push("https://www.linkedin.com/in/elkan-d-silva-249755217/".to_string());

    ctx.set_contents(v[0].to_owned()).unwrap();

}

pub fn chat_gpt(){
    let url = String::from("https://chat.openai.com/");
    let _open_url = Command::new("cmd")
    .args(&["/C", "start", &url])
    .status()
    .expect("failed to open browser");
}