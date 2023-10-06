mod app;

use crate::app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
