use druid::widget::{Container, CrossAxisAlignment, Flex, Label};

pub struct ButtonsBar {
    is_song_playing: bool,
}

impl ButtonsBar {
    pub fn new() -> Self {
        let container = Flex::row()
            .with_flex_child(Label::new("hi there"), 1.0)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .must_fill_main_axis(true);

        let shuffle_icon = match include_str!("../svg/shuffle.svg").parse::<SvgData>() {
            Ok(shuffle) => shuffle,
        };

        ButtonsBar {
            is_song_playing: false,
            container,
        }
    }
}
