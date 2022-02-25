#![feature(generic_associated_types, type_alias_impl_trait)]
use olma_gtk::views::{Button, Label};
use olma_gtk::{AnyView, App, View};

struct Model {
    num: i32,
}

enum Msg {}
impl App for Model {
    type Msg = ();
    type View<'a> = impl View<'a>;

    fn update(&mut self, msg: Self::Msg) {
        self.num += 1;
    }

    fn view(&self) -> Self::View<'_> {
        Button::new(format!("count: {}", self.num)).on_click(|| {})
    }
}

fn main() {
    olma_gtk::launch(Model { num: 0 });
}
