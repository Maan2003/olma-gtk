#![feature(generic_associated_types, fn_traits, unboxed_closures)]
mod messages;
mod view;
pub mod views;

mod app;
pub use app::{launch, App};
pub use view::*;
