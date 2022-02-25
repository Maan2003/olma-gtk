#![feature(generic_associated_types, type_alias_impl_trait)]
use olma_gtk::views::*;
use olma_gtk::{App, View};

struct Model {
    value: String,
}

#[derive(Debug)]
enum Msg {
    Update(String),
}

impl App for Model {
    type Msg = Msg;
    type View<'a> = impl View<'a>;

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Update(value) => self.value = value,
        }
    }

    fn view(&self) -> Self::View<'_> {
        TextBox::new(&self.value).on_change(Msg::Update)
    }
}

fn main() {
    olma_gtk::launch(Model {
        value: String::new(),
    });
}
