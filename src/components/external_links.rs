use crate::components::external_link::{ExternalLink, ExternalLinkProps};
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ExternalLinksProps {
  pub links: Vec<ExternalLinkProps>,
}

#[function_component]
pub fn ExternalLinks(props: &ExternalLinksProps) -> Html {
  let links = props
    .links
    .iter()
    .map(|link| {
      html! {
        <ExternalLink href={link.href.clone()} text={link.text.clone()} icon={link.icon.clone()} />
      }
    })
    .collect::<Html>();

  html! {
    <div class={classes!("external-links")}>
      {links}
    </div>
  }
}
