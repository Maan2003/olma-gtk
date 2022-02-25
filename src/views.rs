mod label {
    use crate::view::View;
    use gtk::prelude::*;

    pub struct Label<'a>(pub &'a str);

    impl<'a> View<'a> for Label<'a> {
        fn build(self) -> gtk::Widget {
            gtk::Label::builder()
                .label(self.0)
                .build()
                .upcast::<gtk::Widget>()
        }

        fn update(self, widget: &gtk::Widget) {
            todo!()
        }
    }
}

pub use label::Label;
