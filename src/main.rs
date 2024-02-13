use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Button, Box, Orientation};

// Set the Macro to our app ID
const APP_ID: &str = "org.ratcap.TestGUI";

fn main() -> glib::ExitCode {
    // Create new application as "app"
    let app = Application::builder().application_id(APP_ID).build();
    
    // Connect the function build_ui to the "activate" signal of "app"
    app.connect_activate(build_ui);

    // Run the application "app"
    app.run()
}

fn build_ui(app: &Application){
    // Create some buttons
    let profile_button = Button::builder()
        .label("Profile")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let settings_button = Button::builder()
        .label("Settings")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Start)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Horizontal)
        .build();
    gtk_box.append(&profile_button);
    gtk_box.append(&settings_button);

    // Create a window and set its title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ratcap")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
