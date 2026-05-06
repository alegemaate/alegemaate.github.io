use yew::{Html, html};
use yew_router::prelude::*;

use crate::pages::blog_list::BlogList;
use crate::pages::blog_post::BlogPost;
use crate::pages::home::Home;
use crate::pages::projects::Projects;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/projects")]
  Projects,
  #[at("/blog")]
  Blog,
  #[at("/blog/:slug")]
  Post { slug: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(route: Route) -> Html {
  match route {
    Route::Home => html! { <Home /> },
    Route::Projects => html! { <Projects /> },
    Route::Blog => html! { <BlogList /> },
    Route::Post { slug } => html! { <BlogPost slug={slug} /> },
    Route::NotFound => html! { <Home /> },
  }
}
