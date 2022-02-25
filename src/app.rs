use crate::{messages::Messenger, view::AnyView, View};
use core::fmt;
use gtk::{glib, prelude::*};
use std::{any::Any, cell::RefCell, rc::Rc};

pub trait App {
    type Msg: fmt::Debug;
    type View<'a>: View<'a>
    where
        Self: 'a;
    fn update(&mut self, msg: Self::Msg);
    fn view<'a>(&'a self) -> Self::View<'a>;
}

pub fn launch<A: App + 'static>(app: A) {
    // Create a new application
    let application = gtk::Application::builder()
        .application_id("com.github.maan2003.olma-gtk")
        .build();

    let app = Rc::new(RefCell::new(app));

    // Connect to "activate" signal of `app`
    application.connect_activate(move |application| {
        let (tx, rx) = glib::MainContext::channel::<Box<dyn Any>>(glib::PRIORITY_DEFAULT);
        Messenger::set(tx.clone());
        let root = AnyView::new(app.borrow().view()).build();
        Messenger::unset();

        // Create a window and set the title
        let window = gtk::ApplicationWindow::builder()
            .application(application)
            .title("My GTK App")
            .child(&root)
            .build();

        let app2 = app.clone();
        rx.attach(None, move |msg| {
            app2.borrow_mut().update(*msg.downcast::<A::Msg>().unwrap());
            Messenger::set(tx.clone());
            {
                let app2 = app2.borrow();
                let view = AnyView::new(app2.view());
                view.update(&root);
            }
            Messenger::unset();
            Continue(true)
        });

        // Present window
        window.present();
    });

    // Run the application
    application.run();
}
