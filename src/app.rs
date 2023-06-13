use yew::classes;
use yew::prelude::*;

use crate::components::about_panel::AboutPanel;
use crate::components::container::Container;
use crate::components::external_link::ExternalLinkProps;
use crate::components::external_links::ExternalLinks;

use crate::icons::adsgames::AdsGamesIcon;
use crate::icons::freesound::FreesoundIcon;
use crate::icons::github::GithubIcon;
use crate::icons::soundcloud::SoundcloudIcon;

#[function_component]
pub fn App() -> Html {
    let links = vec![
        ExternalLinkProps {
            href: "https://github.com/alegemaate",
            text: "github",
            icon: html! {<GithubIcon/>},
        },
        ExternalLinkProps {
            href: "https://soundcloud.com/allan-legemaate",
            text: "soundcloud",
            icon: html! {<SoundcloudIcon/>},
        },
        ExternalLinkProps {
            href: "https://freesound.org/people/alegemaate/",
            text: "freesound",
            icon: html! {<FreesoundIcon />},
        },
        ExternalLinkProps {
            href: "https://www.adsgames.net",
            text: "adsgames",
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
        </Container>
    }
}
