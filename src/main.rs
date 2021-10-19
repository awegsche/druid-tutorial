use std::time::Duration;

use druid::{AppDelegate, AppLauncher, WindowDesc};

use crate::{data::{AppState, TodoItem}, delegate::Delegate, view::build_ui};

mod data;
mod view;
mod delegate;
mod controller;

const TOKIO_DELAY: u64 = 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let main_window = WindowDesc::new(build_ui())
        .title("Todo Tut")
        .window_size((400.0, 400.0));

    let initial_state = AppState::load_from_json();
    let launcher = AppLauncher::with_window(main_window)
    .delegate(Delegate{})
    .log_to_console();

    let handle = launcher.get_external_handle();

    tokio::spawn(wait_and_execute(handle));

    launcher.launch(initial_state)
    .expect("mskfhaseg");
}

async fn wait_and_execute(event_sink: druid::ExtEventSink) {
    tokio::time::sleep(Duration::from_millis(TOKIO_DELAY * 1000)).await;

    println!("{}s has passed", TOKIO_DELAY);
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.add_todo();
    })

}
