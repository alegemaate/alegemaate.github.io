use yew::{Html, classes, component, html};
use yew_router::prelude::*;

use crate::blog::posts;
use crate::components::container::Container;
use crate::route::Route;

#[component]
pub fn BlogList() -> Html {
  let posts = posts::all();

  let items = posts
    .iter()
    .map(|post| {
      html! {
        <article class="blog-list__item">
          <Link<Route> to={Route::Post { slug: post.slug.to_string() }} classes={classes!("blog-list__link")}>
            <h2 class="blog-list__title">{ post.title }</h2>
          </Link<Route>>
          <div class="blog-list__date">{ post.date }</div>
          <p class="blog-list__description">{ post.description }</p>
        </article>
      }
    })
    .collect::<Html>();

  html! {
    <Container>
      <div class={classes!("grid-item", "grid-item__title", "grid-item__title--small", "grid-item__full")}>
        <h1>{ "Blog" }</h1>
      </div>
      <div class={classes!("grid-item", "grid-item__padded", "grid-item__full")}>
        <Link<Route> to={Route::Home} classes={classes!("blog-back")}>
          { "← Home" }
        </Link<Route>>
        <div class="blog-list">
          { items }
        </div>
      </div>
    </Container>
  }
}
