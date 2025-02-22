use yew::{classes, function_component, html, Html};

#[function_component]
pub fn Footer() -> Html {
  const YEAR: i32 = 2025;

  html! {
    <footer>
      <div class={classes!("container__footer")}>
        <div class={classes!("container__footer__content")}>
          <div class={classes!("container__footer__content__text")}>
            {"© "}
            {YEAR}
            {" Allan Legemaate"}
          </div>
        </div>
      </div>
    </footer>
  }
}
