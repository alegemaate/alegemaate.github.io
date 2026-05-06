use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::{Route, switch};

#[component]
pub fn App() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>
  }
}
