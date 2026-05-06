pub struct Post {
  pub slug: &'static str,
  pub title: &'static str,
  pub date: &'static str,
  pub description: &'static str,
  pub tags: Vec<&'static str>,
  pub author: &'static str,
  pub body: &'static str,
}

const RAW_POSTS: &[(&str, &str)] = &[(
  "scanned-synthesis",
  include_str!("../../posts/scanned-synthesis.md"),
)];

pub fn all() -> Vec<Post> {
  let mut posts: Vec<Post> = RAW_POSTS
    .iter()
    .map(|(slug, raw)| parse(slug, raw))
    .collect();
  posts.sort_by(|a, b| b.date.cmp(a.date));
  posts
}

pub fn find(slug: &str) -> Option<Post> {
  RAW_POSTS
    .iter()
    .find(|(s, _)| *s == slug)
    .map(|(s, raw)| parse(s, raw))
}

fn parse(slug: &&'static str, raw: &&'static str) -> Post {
  let (frontmatter, body) = split_frontmatter(raw);
  let mut title = "Untitled";
  let mut date = "";
  let mut description = "";
  let mut tags: Vec<&'static str> = Vec::new();
  let mut author = "";

  for line in frontmatter.lines() {
    if let Some((key, value)) = line.split_once(':') {
      let value = value.trim();
      match key.trim() {
        "title" => title = value,
        "date" => date = value,
        "description" => description = value,
        "tags" => {
          tags = value
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .collect();
        }
        "author" => author = value,
        _ => {}
      }
    }
  }

  Post {
    slug,
    title,
    date,
    description,
    tags,
    author,
    body,
  }
}

fn split_frontmatter(raw: &'static str) -> (&'static str, &'static str) {
  let trimmed = raw.trim_start_matches('\u{feff}');
  let Some(rest) = trimmed.strip_prefix("---") else {
    return ("", trimmed);
  };
  let rest = rest.trim_start_matches('\n');
  let Some(end) = rest.find("\n---") else {
    return ("", trimmed);
  };
  let frontmatter = &rest[..end];
  let body = rest[end + 4..].trim_start_matches('\n');
  (frontmatter, body)
}
