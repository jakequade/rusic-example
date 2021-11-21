extern crate gio;
extern crate gtk;

use std::env;

use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
};

fn main() {
    let application = Application::builder().application_id("com.example.Rusic").build();

    application.connect_activate(|application| {
        let window = ApplicationWindow::builder().application(application).title("Rusic").build();

        window.show_all();
    });

    application.run();
}
