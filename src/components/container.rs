use yew::{Children, Html, Properties, classes, component, html};

use crate::components::footer::Footer;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
  pub children: Children,
}

#[component]
pub fn Container(props: &ContainerProps) -> Html {
  html! {
    <div class={classes!("container")}>
      <main class={classes!("container__main")}>
        <div class={classes!("container__main__content")}>
          {props.children.clone()}
        </div>
      </main>
      <Footer/>
    </div>
  }
}
