#![feature(generic_associated_types, type_alias_impl_trait)]
use olma_gtk::views::Label;
use olma_gtk::{AnyView, App, View};

struct Model;
impl App for Model {
    type Msg = ();
    type View<'a> = impl View<'a>;

    fn update(&mut self, msg: Self::Msg) {}

    fn view(&self) -> Self::View<'_> {
        Label("Hello world!")
    }
}

fn main() {
    olma_gtk::launch(Model);
}
