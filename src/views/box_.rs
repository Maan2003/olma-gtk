use std::iter;

use crate::{view::View, AnyView};
use gtk::prelude::*;
pub struct Box<'a> {
    children: Vec<AnyView<'a>>,
    axis: gtk::Orientation,
}

impl<'a> Box<'a> {
    pub fn column() -> Self {
        Self {
            children: Vec::new(),
            axis: gtk::Orientation::Vertical,
        }
    }

    pub fn row() -> Self {
        Self {
            children: Vec::new(),
            axis: gtk::Orientation::Horizontal,
        }
    }

    pub fn child<V: View<'a>>(mut self, child: V) -> Self {
        self.children.push(AnyView::new(child));
        self
    }

    pub fn add_child<V: View<'a>>(&mut self, child: V) {
        self.children.push(AnyView::new(child));
    }
}

impl<'a> View<'a> for Box<'a> {
    fn build(self) -> gtk::Widget {
        let box_ = gtk::Box::new(self.axis, 0);
        for child in self.children {
            box_.append(&child.build());
        }
        box_.upcast()
    }

    fn update(self, widget: &gtk::Widget) {
        let widget = widget.clone().downcast::<gtk::Box>().unwrap();
        widget.set_orientation(self.axis);
        let mut children = iter::successors(widget.first_child(), WidgetExt::next_sibling);
        let mut views = self.children.into_iter();
        loop {
            match (children.next(), views.next()) {
                (None, Some(view)) => {
                    let child = view.build();
                    widget.append(&child);
                }
                (Some(child), Some(view)) => {
                    view.update(&child);
                }
                (Some(child), None) => {
                    widget.remove(&child);
                }
                (None, None) => break,
            }
        }
    }
}
