#![feature(generic_associated_types, type_alias_impl_trait)]
use gtk::Align;
use olma_gtk::views::*;
use olma_gtk::{views::ViewExt, App, View};

struct Model {
    search: String,
    file: Vec<File>,
}

struct File {
    name: String,
    tags: Vec<String>,
}

#[derive(Debug)]
enum Msg {
    Update(String),
}

fn view_file(file: &File) -> impl View<'_> {
    Box::row()
        .child(Label::new(&file.name))
        .halign(Align::Center)
}

impl App for Model {
    type Msg = Msg;
    type View<'a> = impl View<'a>;

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Update(value) => self.search = value,
        }
    }

    fn view(&self) -> Self::View<'_> {
        let mut files = Box::column();
        for file in &self.file {
            files.add_child(view_file(file));
        }
        Box::column()
            .child(TextBox::new(&self.search).on_change(Msg::Update))
            .child(files)
    }
}

fn main() {
    olma_gtk::launch(Model {
        search: String::new(),
        file: vec![
            File {
                name: "file1".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
            },
            File {
                name: "file2".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
            },
        ],
    });
}
