use alegemaate_github_io::App;

fn main() {
  let root = web_sys::window()
    .and_then(|w| w.document())
    .and_then(|d| d.get_element_by_id("app"))
    .expect("missing #app mount node");
  yew::Renderer::<App>::with_root(root).hydrate();
}
