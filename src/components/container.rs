use gtk::prelude::*;
use gtk::{Box, BoxBuilder, Orientation, SeparatorBuilder};

use crate::components::{song_title_like_bar::SongTitleLikeBar, toolbar::MusicToolbar};

/// Contains all the elements that make up the app.
pub struct Container {
    pub container: Box,
}

impl Container {
    pub fn new() -> Self {
        let container = BoxBuilder::new()
            .vexpand(true)
            .hexpand(true)
            .orientation(Orientation::Vertical)
            .build();

        let mut children = vec![];

        let song_title = SongTitleLikeBar::new();
        children.push(song_title.component());

        let music_toolbar = MusicToolbar::new();
        children.push(music_toolbar.toolbar());

        for child in children {
            container.add(child);
            container.add(&SeparatorBuilder::new().margin(10).build())
        }

        Container { container }
    }
}
