use std::borrow::Cow;

use crate::{messages::Messenger, view::View};
use gtk::prelude::*;

// TODO: allow updating the on_click
pub struct Button<'a> {
    label: Cow<'a, str>,
    on_click: Box<dyn Fn()>,
}

impl<'a> Button<'a> {
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            on_click: Box::new(|| {}),
        }
    }

    pub fn on_click<F: 'static, M: 'static>(mut self, on_click: F) -> Self
    where
        F: Fn() -> M,
    {
        let msgr = Messenger::current().unwrap();
        self.on_click = Box::new(move || {
            msgr.send(Box::new(on_click()));
        });
        self
    }
}

impl<'a> View<'a> for Button<'a> {
    fn build(self) -> gtk::Widget {
        let btn = gtk::Button::builder().label(&self.label).build();
        let _ = btn.connect_clicked(move |_| {
            (self.on_click)();
        });

        btn.upcast::<gtk::Widget>()
    }

    fn update(self, widget: &gtk::Widget) {
        widget
            .downcast_ref::<gtk::Button>()
            .unwrap()
            .set_label(&self.label);
    }
}

