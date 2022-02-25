pub trait View<'a>: 'a {
    fn build(self) -> gtk::Widget;
    // widget is garaunteed to have same type_id as `widget_type_id`
    fn update(self, widget: &gtk::Widget);
}

pub trait DynView<'a>: 'a {
    fn build(&mut self) -> gtk::Widget;
    fn update(&mut self, widget: &gtk::Widget);
}

struct DynViewWrap<V>(Option<V>);

impl<'a, V: View<'a>> DynView<'a> for DynViewWrap<V> {
    fn build(&mut self) -> gtk::Widget {
        self.0.take().unwrap().build()
    }

    fn update(&mut self, widget: &gtk::Widget) {
        self.0.take().unwrap().update(widget);
    }
}
pub struct AnyView<'a> {
    inner: Box<dyn DynView<'a>>,
}

impl<'a> AnyView<'a> {
    pub fn new<V: View<'a>>(inner: V) -> Self {
        AnyView {
            inner: Box::new(DynViewWrap(Some(inner))),
        }
    }

    pub fn build(mut self) -> gtk::Widget {
        self.inner.build()
    }

    pub fn update(mut self, widget: &gtk::Widget) {
        self.inner.update(widget);
    }
}
