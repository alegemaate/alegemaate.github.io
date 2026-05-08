use alegemaate_github_io::App;

fn main() {
  let window = web_sys::window().expect("missing window");
  let document = window.document().expect("missing document");
  let root = document
    .get_element_by_id("app")
    .expect("missing #app mount node");

  let ssr_url = root.get_attribute("data-ssr-url");
  let current_url = window.location().pathname().unwrap_or_default();

  if ssr_url.as_deref() == Some(current_url.as_str()) {
    yew::Renderer::<App>::with_root(root).hydrate();
  } else {
    root.set_inner_html("");
    yew::Renderer::<App>::with_root(root).render();
  }
}
