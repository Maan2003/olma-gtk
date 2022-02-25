#![feature(generic_associated_types)]
mod view;
pub mod views;

mod app;
pub use app::{launch, App};
pub use view::*;
