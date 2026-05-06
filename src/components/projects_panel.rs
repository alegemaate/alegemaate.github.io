use yew::{Html, classes, component, html};

#[component]
pub fn ProjectsPanel() -> Html {
  html! {
    <div class={classes!("about")}>
      <h3>{"ASW"}</h3>
      <p>{"A C++ game library built on SDL3."}</p>
      <a href="https://github.com/AdsGames/asw">{"View on GitHub"}</a>

      <h3>{"Herb Codex"}</h3>
      <p>{"A simple web app for looking up herbs and their uses."}</p>
      <a href="https://github.com/alegemaate/herb-codex">{"View on GitHub"}</a>

      <h3>{"Allegro.ts"}</h3>
      <p>{"A TypeScript \"port\" of the Allegro game library."}</p>
      <a href="https://github.com/alegemaate/allegro-ts">{"View on GitHub"}</a>
    </div>
  }
}
