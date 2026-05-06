use yew::{Html, Properties, classes, component, html};

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
  pub href: String,
  pub text: String,
  pub icon: Html,
}

#[component]
pub fn ExternalLink(props: &ExternalLinkProps) -> Html {
  let text = format!("/{}/", props.text);

  html! {
    <a href={props.href.clone()} rel="noopener noreferrer" target="_blank" class={classes!("external-link")}>
      <div class={classes!("external-link__text")}>{text}</div>
      <div class={classes!("external-link__icon")}>{props.icon.clone()}</div>
    </a>
  }
}
