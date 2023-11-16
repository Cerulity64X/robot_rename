use std::{error::Error, str::FromStr};

use build_cfg::FORCE_PASS_CFG;
use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::{run_native, NativeOptions, App};
use egui::{CentralPanel, RichText, Color32};
use url::Url;

mod build_cfg;

fn format_ip_addr(robot_number: i128) -> String {
    let ip_str = format!("{robot_number:04}");
    format!("10.{}.{}.2", &ip_str[0..2], &ip_str[2..4])
}

pub enum StringState {
    Valid,
    HasWs,
    Empty,
}
impl StringState {
    /// `has_whitespace` is ignored if `is_empty` is `true`
    fn get(string: &str) -> Self {
        match (string.is_empty(), string.contains(char::is_whitespace)) {
            (true, _) => StringState::Empty,
            (false, true) => StringState::HasWs,
            (false, false) => StringState::Valid,
        }
    }
}

pub fn can_customize_password() -> bool {
    FORCE_PASS_CFG || std::env::var("ROBOT_RENAME_CFG_PASSWORD").is_ok()
}

pub struct RobotRename {
    pub name: String,
    pub number_string: String,
    pub number: Result<i128, String>,
    pub password: String,
    pub github_link: Url,
    //pub show_command: bool,
    pub clip: ClipboardContext,
}
impl Default for RobotRename {
    fn default() -> Self {
        Self {
            name: String::new(),
            number_string: String::new(),
            number: parse_robot_number(""),
            password: if can_customize_password() { String::new() } else { format!("password") },
            github_link: Url::from_str("https://github.com/Cerulity64X/robot_rename")
                .expect("GitHub link was invalid!"),
            //show_command: false,
            clip: ClipboardProvider::new().unwrap(),
        }
    }
}
fn parse_robot_number(st: &str) -> Result<i128, String> {
    match st.parse::<i128>() {
        Err(e) => Err(format!("Could not parse robot number ({:?}).", e.kind())),
        Ok(n) if n > 9999 => Err(format!("Robot number too large (>9999).")),
        Ok(n) if n < 0 => Err(format!("Robot number must be above zero.")),
        Ok(n) => Ok(n)
    }
}
impl App for RobotRename {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Robot Rename Command Generator").color(Color32::WHITE));

            ui.label("");
            ui.separator();

            ui.label(RichText::new("Fields").size(15.0).color(Color32::WHITE));

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                ui.label("Number");
                if ui.text_edit_singleline(&mut self.number_string).changed() {
                    self.number = parse_robot_number(&self.number_string);
                }

            });

            if can_customize_password() {
                ui.horizontal(|ui| {
                    ui.label("Password");
                    ui.text_edit_singleline(&mut self.password);
                });
            } else {
                self.password = format!("password");
            }

            let name_state = StringState::get(&self.name);
            let name_error = match name_state {
                StringState::Valid => if self.name.starts_with(char::is_numeric) {
                    Some("Name cannot start with a number.".to_string())
                } else { None },
                StringState::HasWs => Some("Name cannot contain whitespace.".to_string()),
                StringState::Empty => Some("Name cannot be empty.".to_string())
            };
            
            let password_state = StringState::get(&self.password);
            let password_error = if can_customize_password() { match password_state {
                StringState::Valid => None,
                StringState::HasWs => Some("Password cannot contain whitespace."),
                StringState::Empty => Some("Password cannot be empty.")
            } } else { None };
            if let Ok(n) = self.number {
                ui.label(format!("New IP address: {}", format_ip_addr(n)));
            }
            
            let can_show =
                password_error.is_none() &&
                name_error.is_none();

            ui.label("");
            ui.separator();
            if can_show {
                if let Ok(num) = self.number {
                    ui.label(RichText::new("Generated Command").size(15.0).color(Color32::WHITE));
                    let cmd = format!("setupWifiAP.sh {} {num:04} {}", self.name, self.password);
                    ui.label(&cmd);
                    if ui.selectable_label(true, "Copy").clicked() {
                        let _ = self.clip.set_contents(cmd);
                    }
                } else {
                    ui.label(RichText::new("Invalid fields").color(Color32::from_rgb(255, 128, 128)).size(15.0));
                }
            } else {
                ui.label(RichText::new("Invalid fields").color(Color32::from_rgb(255, 128, 128)).size(15.0));
            }

            if let Some(nerr) = name_error {
                ui.label(nerr);
            }
            if let Some(perr) = password_error {
                ui.label(perr);
            }
            if let Err(e) = &self.number {
                ui.label(e);
            }

            ui.label("");
            ui.separator();

            ui.label("Tool created by Cerulity32K");
            ui.horizontal(|ui| {
                ui.label("GitHub repository: ");
                if ui.link(self.github_link.as_str()).clicked() {
                    url_open::open(&self.github_link);
                }
            })
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(run_native("Robot Rename", NativeOptions::default(), Box::new(|_|
        Box::new(RobotRename::default())
    ))?)
}
