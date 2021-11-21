extern crate gio;
extern crate gtk;

mod toolbar;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use toolbar::MusicToolbar;

struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Rusic")
            .build();

        let toolbar = MusicToolbar::new();
        window.add(toolbar.toolbar());

        window.show_all();

        let app = App { toolbar, window };

        app.connect_events();

        app
    }

    fn connect_events(&self) {}

    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| unsafe {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            // if play_button.fmt() == Some(PLAY_STOCK.to_string()) {
            //     play_button.set_stock_id(PAUSE_STOCK);
            // } else {
            //     play_button.set_stock_id(PLAY_STOCK);
            // }
            println!("{:?}", play_button);
        });
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
