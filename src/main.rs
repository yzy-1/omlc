#![feature(lazy_cell)]

mod app;
mod board;
mod model;
mod post;
mod status;

use crate::app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
