use gtk::prelude::*;

use gtk::{
    Box, Button, ButtonBuilder, IconSize::SmallToolbar, Image, Orientation, RadioToolButton,
    Separator, Toolbar,
};

use gdk_pixbuf::Pixbuf;

const PLAY_STOCK: &str = "gtk-media-play";

pub struct MusicToolbar {
    pub shuffle: Button,
    pub previous: Button,
    pub play: Button,
    pub next: Button,
    pub repeat: Button,
    pub toolbar: Box,
}

impl MusicToolbar {
    pub fn new() -> MusicToolbar {
        let toolbar = Box::builder()
            .orientation(Orientation::Horizontal)
            .vexpand(true)
            .build();

        let shuffle = Button::new();

        shuffle.set_image(Some(&Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale("src/svg/shuffle.svg", 25, 25, true).unwrap(),
        ))));

        toolbar.add(&shuffle);

        toolbar.add(&Separator::new(Orientation::Horizontal));

        let previous = Button::new();

        previous.set_image(Some(&Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale("src/svg/skip-back.svg", 25, 25, true).unwrap(),
        ))));

        toolbar.add(&previous);

        let play = Button::new();
        play.set_image(Some(&Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale("src/svg/play-circle.svg", 35, 35, true).unwrap(),
        ))));

        toolbar.add(&play);

        let next = Button::new();

        next.set_image(Some(&Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale("src/svg/skip-forward.svg", 25, 25, true).unwrap(),
        ))));

        toolbar.add(&next);

        let repeat = Button::new();

        repeat.set_image(Some(&Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale("src/svg/repeat.svg", 25, 25, true).unwrap(),
        ))));

        toolbar.add(&repeat);

        toolbar.add(&Separator::new(Orientation::Horizontal));

        MusicToolbar {
            shuffle,
            previous,
            play,
            next,
            repeat,
            toolbar: toolbar,
        }
    }

    pub fn toolbar(&self) -> &Box {
        &self.toolbar
    }
}
