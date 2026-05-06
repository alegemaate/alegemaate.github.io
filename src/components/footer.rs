use yew::{Html, classes, component, html};

#[component]
pub fn Footer() -> Html {
  const YEAR: i32 = 2026;

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
