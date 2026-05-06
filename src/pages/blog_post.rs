use yew::{Html, Properties, classes, component, html};
use yew_router::prelude::*;

use crate::blog::posts;
use crate::blog::render::Markdown;
use crate::components::container::Container;
use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
  pub slug: String,
}

#[component]
pub fn BlogPost(props: &BlogPostProps) -> Html {
  let Some(post) = posts::find(&props.slug) else {
    return html! {
      <Container>
        <div class={classes!("grid-item", "grid-item__title", "grid-item__title--small", "grid-item__full")}>
          <h1>{ "Not found" }</h1>
        </div>
        <div class={classes!("grid-item", "grid-item__padded", "grid-item__full")}>
          <p>{ "That post doesn't exist." }</p>
          <Link<Route> to={Route::Blog} classes={classes!("blog-back")}>
            { "← Back to blog" }
          </Link<Route>>
        </div>
      </Container>
    };
  };

  html! {
    <Container>
      <div class={classes!("grid-item", "grid-item__title", "grid-item__title--small", "grid-item__full")}>
        <h1>{ post.title }</h1>
      </div>
      <div class={classes!("grid-item", "grid-item__padded", "grid-item__full")}>
        <Link<Route> to={Route::Blog} classes={classes!("blog-back")}>
          { "← Back to blog" }
        </Link<Route>>
        <div class="blog-post__date">{ post.date }</div>
        <div class="blog-post__tags">
          { for post.tags.iter().map(|tag| html! {
            <span class="blog-post__tag">{ tag }</span>
          }) }
        </div>
        <div class="blog-post__author">{ post.author }</div>
        <Markdown source={post.body.to_string()} />
      </div>
    </Container>
  }
}
