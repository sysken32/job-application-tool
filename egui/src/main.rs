#![windows_subsystem = "windows"]

use eframe;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::process::Command;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 240.0]),
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
    // v: Vec<String>,
    label_text: String,
    url: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label_text: String::from(""),
            url: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical_centered(|ui| {
                ui.heading("Job Applier App");
                //working on section here
                let url_label = ui.label("Paste your linkedin URL below -");
                ui.text_edit_singleline(&mut self.url)
                    .labelled_by(url_label.id);
                if ui.add(egui::Button::new("Linkedin")).clicked(){
                    linked_in(&mut self.url);
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

pub fn linked_in(url_ref: &mut String) {
    let mut ctx = ClipboardContext::new().unwrap();
    match ctx.set_contents(url_ref.to_string()) {
        Ok(_) => {}
        Err(e) => {println!("{} returned", e)}
    }
    println!("{} has been copied.", url_ref);
}

pub fn chat_gpt(){
    let url = String::from("https://chat.openai.com/");
    let _open_url = Command::new("cmd")
    .args(&["/C", "start", &url])
    .status()
    .expect("failed to open browser");
}