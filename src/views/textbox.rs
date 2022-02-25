use std::rc::Rc;

use crate::{messages::Messenger, view::*};
use gtk::prelude::*;

pub struct TextBox<'a> {
    text: &'a str,
    on_change: Option<Rc<dyn Fn(String)>>,
}

impl<'a> TextBox<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            on_change: None,
        }
    }

    pub fn on_change<M: 'static>(mut self, on_change: impl Fn(String) -> M + 'static) -> Self {
        let msgr = Messenger::current().unwrap();
        self.on_change = Some(Rc::new(move |txt| {
            msgr.send(Box::new(on_change(txt)));
        }));
        self
    }
}

impl<'a> View<'a> for TextBox<'a> {
    fn build(self) -> gtk::Widget {
        let text = gtk::Entry::new();
        text.set_text(self.text);
        let on_change = self.on_change;
        let on_change2 = on_change.clone();
        text.connect_insert_text(move |text, _, _| {
            let text = text.text();
            if let Some(on_change) = &on_change2 {
                on_change(text.into());
            }
        });
        text.connect_delete_text(move |text, _, _| {
            let text = text.text();
            if let Some(on_change) = &on_change {
                on_change(text.into());
            }
        });
        text.upcast()
    }

    fn update(self, widget: &gtk::Widget) {
        let widget = widget.downcast_ref::<gtk::Entry>().unwrap();
        widget.set_text(self.text);
    }
}
