use crate::{AnyView, View};
use gtk::prelude::*;

pub struct WidgetWrap<'a> {
    inner: AnyView<'a>,
    margin_start: Option<i32>,
    margin_end: Option<i32>,
    margin_top: Option<i32>,
    margin_bottom: Option<i32>,
    valign: Option<gtk::Align>,
    halign: Option<gtk::Align>,
}

impl<'a> WidgetWrap<'a> {
    pub fn new(inner: AnyView<'a>) -> Self {
        WidgetWrap {
            inner,
            margin_start: None,
            margin_end: None,
            margin_top: None,
            margin_bottom: None,
            valign: None,
            halign: None,
        }
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }
}

impl<'a> View<'a> for WidgetWrap<'a> {
    fn build(self) -> gtk::Widget {
        let widget = self.inner.build();

        if let Some(margin_start) = self.margin_start {
            widget.set_margin_start(margin_start);
        }

        if let Some(margin_end) = self.margin_end {
            widget.set_margin_end(margin_end);
        }

        if let Some(margin_top) = self.margin_top {
            widget.set_margin_top(margin_top);
        }

        if let Some(margin_bottom) = self.margin_bottom {
            widget.set_margin_bottom(margin_bottom);
        }

        if let Some(valign) = self.valign {
            widget.set_valign(valign);
        }

        if let Some(halign) = self.halign {
            widget.set_halign(halign);
        }

        widget
    }

    fn update(self, widget: &gtk::Widget) {
        self.inner.update(widget);
        if let Some(margin_start) = self.margin_start {
            widget.set_margin_start(margin_start);
        }
        if let Some(margin_end) = self.margin_end {
            widget.set_margin_end(margin_end);
        }
        if let Some(margin_top) = self.margin_top {
            widget.set_margin_top(margin_top);
        }
        if let Some(margin_bottom) = self.margin_bottom {
            widget.set_margin_bottom(margin_bottom);
        }
        if let Some(valign) = self.valign {
            widget.set_valign(valign);
        }
        if let Some(halign) = self.halign {
            widget.set_halign(halign);
        }
    }
}
