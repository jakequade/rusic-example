use druid::widget::Label;
use druid::widget::{Align, Container, Flex, TextBox, Widget};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, WidgetExt, WindowDesc};

use crate::components::song_view::buttons_bar;

/// Contains all the elements that make up the app.
/// Elements should be added before `for child in children` here.
pub struct AppContainer {
    pub container: Box<dyn Widget<u32>>,
}

#[derive(Clone, Data, Lens)]
pub struct AppContainerState {}

impl AppContainer {
    pub fn init() -> impl Widget<AppContainerState> {
        // let data = 0_u32;

        // let mut children = vec![];

        // let container = Box::from(Flex::column());

        // let music_actions_bar = buttons_bar::ButtonsBar::new();
        // AppContainer { container }

        // a label that will determine its text based on the current app data.
        let label = Label::new("hi there").padding(5.0).center();

        // arrange the two widgets vertically, with some padding
        let layout = Flex::column().with_child(label).with_spacer(2.0);

        layout
    }
}
