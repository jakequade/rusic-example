use gtk::prelude::ContainerExt;

use gtk::{RadioToolButton, SeparatorToolItem, Toolbar};

const PLAY_STOCK: &str = "gtk-media-play";

pub struct MusicToolbar {
    pub open_button: RadioToolButton,
    pub next_button: RadioToolButton,
    pub play_button: RadioToolButton,
    pub previous_button: RadioToolButton,
    pub quit_button: RadioToolButton,
    pub remove_button: RadioToolButton,
    pub stop_button: RadioToolButton,
    pub toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> MusicToolbar {
        let toolbar = Toolbar::new();

        let open_button = RadioToolButton::from_stock("gtk-open");
        toolbar.add(&open_button);

        toolbar.add(&SeparatorToolItem::new());

        let previous_button = RadioToolButton::from_stock("gtk-media-previous");
        toolbar.add(&previous_button);

        let play_button = RadioToolButton::from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = RadioToolButton::from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = RadioToolButton::from_stock("gtk-media-next");
        toolbar.add(&next_button);

        let remove_button = RadioToolButton::from_stock("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = RadioToolButton::from_stock("gtk-quit");
        toolbar.add(&quit_button);

        MusicToolbar {
            previous_button,
            play_button,
            stop_button,
            open_button,
            next_button,
            remove_button,
            quit_button,
            toolbar: toolbar,
        }
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}
