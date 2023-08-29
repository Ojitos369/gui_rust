use std::cell::Cell;
use std::rc::Rc;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Orientation };
use glib::clone;
// use gtk::{glib, Application};

const APP_ID: &str = "com.ojitos369.pkcal";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let number = Rc::new(Cell::new(0));
    let text = format!("More: {}", number.get());
    let text2 = format!("Less: {}", number.get());

    let button = Button::builder()
        .label(text)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let d_button = Button::builder()
        .label(text2)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    button.connect_clicked(clone! (@weak number, @weak d_button => move |button| {
        number.set(number.get() + 1);
        let text = format!("More: {}", number.get());
        button.set_label(&text);
        let otext = format!("Less: {}", number.get());
        d_button.set_label(&otext);
    }));

    d_button.connect_clicked(clone!(@weak button => move |d_button| {
        number.set(number.get() - 1);
        let text = format!("Less: {}", number.get());
        d_button.set_label(&text);
        let otext = format!("More: {}", number.get());
        button.set_label(&otext);
    }));


    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&d_button);


    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("pkCal")
        .child(&gtk_box)
        .build();

    // Present window
    window.present();
}

