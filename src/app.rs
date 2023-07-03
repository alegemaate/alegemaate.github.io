use yew::classes;
use yew::prelude::*;

use crate::components::about_panel::AboutPanel;
use crate::components::container::Container;
use crate::components::external_link::ExternalLinkProps;
use crate::components::external_links::ExternalLinks;
use crate::components::projects_panel::ProjectsPanel;

use crate::icons::adsgames::AdsGamesIcon;
use crate::icons::freesound::FreesoundIcon;
use crate::icons::github::GithubIcon;
use crate::icons::soundcloud::SoundcloudIcon;

#[function_component]
pub fn App() -> Html {
  let links = vec![
    ExternalLinkProps {
      href: String::from("https://github.com/alegemaate"),
      text: String::from("github"),
      icon: html! {<GithubIcon/>},
    },
    ExternalLinkProps {
      href: String::from("https://soundcloud.com/allan-legemaate"),
      text: String::from("soundcloud"),
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
      <div class={classes!("grid-item", "grid-item__image")}>
        <img title="Hey it's me" alt="Image of Allan Legemaate" width="700" height="700" src="/me.webp" />
      </div>
      <div class={classes!("grid-item", "grid-item__padded")}>
        <AboutPanel/>
      </div>
      <div class={classes!("grid-item", "grid-item__padded")}>
        <h2>{"My Pages"}</h2>
        <ExternalLinks links={links}/>
      </div>
      <div class={classes!("grid-item", "grid-item__padded", "grid-item__full")}>
        <ProjectsPanel/>
      </div>

    </Container>
  }
}
