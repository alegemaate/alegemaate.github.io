use yew::classes;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::about_panel::AboutPanel;
use crate::components::container::Container;
use crate::components::external_link::ExternalLinkProps;
use crate::components::external_links::ExternalLinks;
use crate::route::Route;

use crate::icons::adsgames::AdsGamesIcon;
use crate::icons::freesound::FreesoundIcon;
use crate::icons::github::GithubIcon;
use crate::icons::soundcloud::SoundcloudIcon;

#[component]
pub fn Home() -> Html {
  let links = vec![
    ExternalLinkProps {
      href: String::from("https://github.com/alegemaate"),
      text: String::from("github"),
      icon: html! {<GithubIcon/>},
    },
    ExternalLinkProps {
      href: String::from("https://www.frequency303.com"),
      text: String::from("music"),
      icon: html! {<SoundcloudIcon/>},
    },
    ExternalLinkProps {
      href: String::from("https://freesound.org/people/alegemaate/"),
      text: String::from("freesound"),
      icon: html! {<FreesoundIcon />},
    },
    ExternalLinkProps {
      href: String::from("https://www.adsgames.net"),
      text: String::from("adsgames"),
      icon: html! {<AdsGamesIcon/>},
    },
  ];

  html! {
    <Container>
      <div class={classes!("grid-item", "grid-item__title")}>
        <h1>{ "Allan Legemaate" }</h1>
      </div>
      <div class={classes!("grid-item", "grid-item__image", "grid-item__primary-image")}>
        <img title="Hey it's me" alt="Image of Allan Legemaate" width="700" height="700" src="/me.jpg" />
      </div>
      <div class={classes!("grid-item", "grid-item__padded")}>
        <AboutPanel/>
      </div>
      <div class={classes!("grid-item", "grid-item__padded")}>
        <h2>{"My Pages"}</h2>
        <ExternalLinks links={links}/>
        <h2>{"More"}</h2>
        <div class={classes!("internal-links")}>
          <Link<Route> to={Route::Projects} classes={classes!("blog-link")}>
            { "Projects →" }
          </Link<Route>>
          <Link<Route> to={Route::Blog} classes={classes!("blog-link")}>
            { "Blog →" }
          </Link<Route>>
        </div>
      </div>
    </Container>
  }
}
