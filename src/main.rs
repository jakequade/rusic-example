extern crate gio;
extern crate gtk;

mod components;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use components::container::Container;

struct App {
    container: Container,
    window: ApplicationWindow,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Rusic")
            .build();

        let container = Container::new();
        window.add(&container.container);

        window.show_all();

        App { container, window }
    }
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.Rusic")
        .build();

    application.connect_activate(|application| {
        let app = App::new(application);
    });

    application.run();
}
