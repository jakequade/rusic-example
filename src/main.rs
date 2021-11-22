extern crate druid;

mod components;

use druid::{AppLauncher, Data, Lens, WindowDesc};

use components::container::{AppContainer, AppContainerState};

#[derive(Clone, Data, Lens)]
struct HelloState {}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(|| AppContainer::init()).window_size((350.0, 500.0));

    // start the application
    AppLauncher::with_window(main_window)
        .launch(AppContainerState {})
        .expect("Failed to launch application");
}
