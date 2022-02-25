use std::{any::Any, cell::RefCell};

use gtk::glib;

#[derive(Clone)]
pub struct Messenger {
    msgs: glib::Sender<Box<dyn Any>>,
}

thread_local! {
    static CURRENT_MESSENGER: RefCell<Option<Messenger>> = RefCell::new(None);
}

impl Messenger {
    pub(crate) fn set(sender: glib::Sender<Box<dyn Any>>) {
        CURRENT_MESSENGER.with(|cell| {
            cell.replace(Some(Messenger { msgs: sender }));
        });
    }
    pub(crate) fn unset() {
        CURRENT_MESSENGER.with(|cell| {
            cell.replace(None);
        });
    }

    pub fn current() -> Option<Messenger> {
        CURRENT_MESSENGER.with(|cell| cell.borrow().clone())
    }

    pub fn send(&self, msg: Box<dyn Any>) {
        self.msgs.send(msg).unwrap();
    }
}
