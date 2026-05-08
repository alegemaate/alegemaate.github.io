use std::fs;
use std::path::PathBuf;

use alegemaate_github_io::blog::posts;
use alegemaate_github_io::{ServerApp, ServerAppProps};

const MOUNT_PLACEHOLDER: &str = "<div id=\"app\"></div>";

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let dist = std::env::args()
    .nth(1)
    .map(PathBuf::from)
    .expect("usage: ssg <dist-dir>");

  let template = fs::read_to_string(dist.join("index.html")).expect("read dist/index.html");
  if !template.contains(MOUNT_PLACEHOLDER) {
    panic!("dist/index.html missing `{MOUNT_PLACEHOLDER}` mount node");
  }

  let mut routes: Vec<(String, PathBuf)> = vec![
    ("/".into(), dist.join("index.html")),
    ("/projects".into(), dist.join("projects/index.html")),
    ("/blog".into(), dist.join("blog/index.html")),
  ];
  for post in posts::all() {
    routes.push((
      format!("/blog/{}", post.slug),
      dist.join(format!("blog/{}/index.html", post.slug)),
    ));
  }

  for (url, out) in routes {
    let renderer = yew::ServerRenderer::<ServerApp>::with_props({
      let url = url.clone();
      move || ServerAppProps { url: url.into() }
    });
    let body = renderer.render().await;

    let html = template.replacen(
      MOUNT_PLACEHOLDER,
      &format!("<div id=\"app\">{body}</div>"),
      1,
    );

    if let Some(parent) = out.parent() {
      fs::create_dir_all(parent).expect("mkdir -p");
    }
    fs::write(&out, html).expect("write rendered html");
    println!("ssg: {url} -> {}", out.display());
  }
}
