use yew::{classes, function_component, html, Html};

#[function_component]
pub fn ProjectsPanel() -> Html {
  html! {
    <div class={classes!("about")}>
      <h2>{"Projects"}</h2>

      <h3>{"Herb Codex"}</h3>
      <p>{"A simple web app for looking up herbs and their uses."}</p>
      <a href="https://github.com/alegemaate/herb-codex">{"View on GitHub"}</a>

      <h3>{"Allegro.ts"}</h3>
      <p>{"A TypeScript \"port\" of the Allegro game library."}</p>
      <a href="https://github.com/alegemaate/allegro-ts">{"View on GitHub"}</a>

      <h3>{"AfkLib"}</h3>
      <p>{"A C++ game library built on SDL2."}</p>
      <a href="https://github.com/AdsGames/AfkLib">{"View on GitHub"}</a>
    </div>
  }
}
