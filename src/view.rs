
use druid::{Insets, Widget, WidgetExt, widget::{Button, Checkbox, Flex, Label, List, TextBox}};

use crate::{controller::TodoItemController, data::*};

fn new_todo_textbox() -> impl Widget<AppState> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder("Add a new todo")
        .expand_width()
        .lens(AppState::new_todo);

    let add_todo_button = Button::new("Add").on_click(AppState::click_add);

    Flex::row()
    .with_flex_child(new_todo_textbox, 1.0)
    .with_child(add_todo_button)
    .padding(Insets::new(10.0, 10.0, 10.0, 10.0))
}

fn todo_item() -> impl Widget<TodoItem> {
    let checkbox = Checkbox::new("").lens(TodoItem::done);
    let label = Label::raw().lens(TodoItem::text);

    Flex::row().with_child(checkbox).with_flex_child(label, 1.0).controller(TodoItemController)
}

pub fn build_ui() -> impl Widget<AppState> {
    Flex::column()
    .with_child(new_todo_textbox())
    .with_child( List::new(todo_item).lens(AppState::todos).padding(Insets::new(10.0, 10.0, 10.0, 10.0)))
}