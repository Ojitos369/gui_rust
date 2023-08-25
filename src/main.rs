use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            // notify-send "Hello World!" "The button was clicked!"
            std::process::Command::new("notify-send")
                .arg("Hello World!")
                .arg("The button was clicked!")
                .spawn()
                .expect("Failed to execute process");

        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
}