use gtk::prelude::*;
use gtk::{Box, BoxBuilder, Button, Image, Orientation};

use gdk_pixbuf::Pixbuf;

pub struct SongTitleLikeBar {
    song_title: String,
    is_liked: bool,
    bar: Box,
}

impl SongTitleLikeBar {
    pub fn new() -> Self {
        let bar = BoxBuilder::new()
            .vexpand(true)
            .orientation(Orientation::Horizontal)
            .build();

        let like_button = Button::new();
        like_button.set_image(Some(&Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale("src/svg/heart.svg", 25, 25, true).unwrap(),
        ))));

        bar.add(&like_button);
        let song_title = "Some song".to_string();

        SongTitleLikeBar {
            song_title,
            is_liked: false,
            bar,
        }
    }
    pub fn component(&self) -> &Box {
        &self.bar
    }
}
