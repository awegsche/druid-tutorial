use druid::{AppDelegate, Handled, Selector};

use crate::data::AppState;

pub const SAVE: Selector = Selector::new("todo.save");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        env: &druid::Env,
    ) -> druid::Handled {
        if cmd.is(SAVE) {
            data.save_to_json().unwrap();
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}