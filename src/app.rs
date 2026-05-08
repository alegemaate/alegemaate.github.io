use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
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

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
  pub url: AttrValue,
}

#[component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
  let history = AnyHistory::from(MemoryHistory::new());
  history.push(props.url.to_string());
  html! {
    <Router history={history}>
      <Switch<Route> render={switch} />
    </Router>
  }
}
