use std::borrow::Cow;

use crate::view::View;
use gtk::prelude::*;

pub struct Label<'a> {
    text: Cow<'a, str>,
}

impl<'a> Label<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self { text: text.into() }
    }
}

impl<'a> View<'a> for Label<'a> {
    fn build(self) -> gtk::Widget {
        gtk::Label::builder()
            .label(&self.text)
            .build()
            .upcast::<gtk::Widget>()
    }

    fn update(self, widget: &gtk::Widget) {
        let label = widget.downcast_ref::<gtk::Label>().unwrap();
        label.set_label(&self.text);
    }
}

