use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::container::Container;
use crate::components::projects_panel::ProjectsPanel;
use crate::route::Route;

#[component]
pub fn Projects() -> Html {
  html! {
    <Container>
      <div class={classes!("grid-item", "grid-item__title")}>
        <h1>{ "Projects" }</h1>
      </div>
      <div class={classes!("grid-item", "grid-item__padded", "grid-item__full")}>
        <Link<Route> to={Route::Home} classes={classes!("blog-back")}>
          { "← Home" }
        </Link<Route>>
        <ProjectsPanel/>
      </div>
    </Container>
  }
}
