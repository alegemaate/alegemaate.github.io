use std::fs;
use std::path::PathBuf;

use alegemaate_github_io::blog::posts;
use alegemaate_github_io::{ServerApp, ServerAppProps};

const MOUNT_PLACEHOLDER: &str = "<div id=\"app\"></div>";
const CANONICAL_PLACEHOLDER: &str = "<link rel=\"canonical\" href=\"https://alegemaate.com\" />";
const SITE_ORIGIN: &str = "https://alegemaate.com";
const DEFAULT_DESCRIPTION: &str = "Allan Legemaate is a Software Developer from Ontario, Canada";

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
  if !template.contains(CANONICAL_PLACEHOLDER) {
    panic!("dist/index.html missing canonical link `{CANONICAL_PLACEHOLDER}`");
  }

  let mut routes: Vec<(String, PathBuf, String)> = vec![
    ("/".into(), dist.join("index.html"), DEFAULT_DESCRIPTION.into()),
    (
      "/projects".into(),
      dist.join("projects.html"),
      "Projects and open-source work by Allan Legemaate.".into(),
    ),
    (
      "/blog".into(),
      dist.join("blog.html"),
      "Writing by Allan Legemaate on software, audio, and side projects.".into(),
    ),
  ];
  for post in posts::all() {
    let description = if post.description.is_empty() {
      format!("{} — a post by Allan Legemaate.", post.title)
    } else {
      post.description.to_string()
    };
    routes.push((
      format!("/blog/{}", post.slug),
      dist.join(format!("blog/{}.html", post.slug)),
      description,
    ));
  }

  for (url, out, description) in routes {
    let renderer = yew::ServerRenderer::<ServerApp>::with_props({
      let url = url.clone();
      move || ServerAppProps { url: url.into() }
    });
    let body = renderer.render().await;

    let canonical = format!(
      "<link rel=\"canonical\" href=\"{SITE_ORIGIN}{}\" />",
      if url == "/" { "/" } else { url.as_str() },
    );
    let html = template
      .replacen(
        MOUNT_PLACEHOLDER,
        &format!("<div id=\"app\">{body}</div>"),
        1,
      )
      .replacen(CANONICAL_PLACEHOLDER, &canonical, 1)
      .replace(DEFAULT_DESCRIPTION, &html_escape(&description));

    if let Some(parent) = out.parent() {
      fs::create_dir_all(parent).expect("mkdir -p");
    }
    fs::write(&out, html).expect("write rendered html");
    println!("ssg: {url} -> {}", out.display());
  }
}

fn html_escape(s: &str) -> String {
  s.replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;")
}
