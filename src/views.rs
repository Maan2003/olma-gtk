mod box_;
mod button;
mod label;
mod textbox;
mod widget;

pub use box_::Box;
pub use button::Button;
pub use label::Label;
pub use textbox::TextBox;

use crate::{AnyView, View};

pub use widget::WidgetWrap;

pub trait ViewExt<'a>: View<'a> + Sized {
    fn margin_top(self, margin: i32) -> WidgetWrap<'a> {
        WidgetWrap::new(AnyView::new(self)).margin_top(margin)
    }

    fn margin_bottom(self, margin: i32) -> WidgetWrap<'a> {
        WidgetWrap::new(AnyView::new(self)).margin_bottom(margin)
    }

    fn margin_start(self, margin: i32) -> WidgetWrap<'a> {
        WidgetWrap::new(AnyView::new(self)).margin_start(margin)
    }

    fn margin_end(self, margin: i32) -> WidgetWrap<'a> {
        WidgetWrap::new(AnyView::new(self)).margin_end(margin)
    }

    fn valign(self, valign: gtk::Align) -> WidgetWrap<'a> {
        WidgetWrap::new(AnyView::new(self)).valign(valign)
    }

    fn halign(self, halign: gtk::Align) -> WidgetWrap<'a> {
        WidgetWrap::new(AnyView::new(self)).halign(halign)
    }
}

impl<'a, V> ViewExt<'a> for V where V: View<'a> {}
