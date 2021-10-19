use druid::{Widget, widget::Controller};

use crate::{data::TodoItem, delegate::SAVE};

pub struct TodoItemController;

impl<W: Widget<TodoItem>> Controller<TodoItem, W> for TodoItemController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut TodoItem, env: &druid::Env) {
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoItem,
        env: &druid::Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &TodoItem, data: &TodoItem, env: &druid::Env) {
        if old_data.done != data.done {
            ctx.submit_command(SAVE);
        }
        child.update(ctx, old_data, data, env);
    }
}