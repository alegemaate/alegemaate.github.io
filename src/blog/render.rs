use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "katex"], js_name = render)]
  fn katex_render(latex: &str, el: &Element, options: &JsValue);

  #[wasm_bindgen(js_namespace = ["window", "hljs"], js_name = highlightElement)]
  fn highlight_element(el: &Element);
}

pub fn render_to_html(markdown: &str) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TASKLISTS);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_MATH);

  let parser = Parser::new_ext(markdown, options);

  let mut events: Vec<Event<'_>> = Vec::new();
  let mut in_jsx = false;
  let mut jsx_attrs = (String::new(), String::new());
  let mut jsx_source = String::new();
  let mut jsx_counter: usize = 0;

  for event in parser {
    match &event {
      Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
        if let Some(attrs) = jsxgraph_attrs(info) {
          in_jsx = true;
          jsx_attrs = attrs;
          jsx_source.clear();
          continue;
        }
      }
      Event::Text(t) if in_jsx => {
        jsx_source.push_str(t);
        continue;
      }
      Event::End(TagEnd::CodeBlock) if in_jsx => {
        in_jsx = false;
        let id = format!("jxg-{jsx_counter}");
        jsx_counter += 1;
        let (width, height) = &jsx_attrs;
        let placeholder = format!(
          "<div class=\"jxgbox\" id=\"{id}\" style=\"width: {width}; height: {height};\"></div>\n\
           <pre class=\"jsxgraph-source\" data-jxg-id=\"{id}\" style=\"display:none\">{}</pre>\n",
          escape_html_text(&jsx_source)
        );
        events.push(Event::Html(placeholder.into()));
        continue;
      }
      _ if in_jsx => continue,
      _ => {}
    }
    events.push(event);
  }

  let mut buf = String::new();
  html::push_html(&mut buf, events.into_iter());
  buf
}

/// Returns Some((width, height)) if `info` describes a JSXGraph block.
///
/// A block opts in via `jsxgraph` / `jsxgraph=true` inside the fence's
/// `{...}` args, e.g. ``` ```js {jsxgraph=true, width=500, height=500} ```.
/// Recognised attrs: `jsxgraph` (bool flag), `width`, `height`. Bare numeric
/// dimensions get `px` appended.
fn jsxgraph_attrs(info: &str) -> Option<(String, String)> {
  let end = info
    .find(|c: char| c.is_whitespace() || c == '{')
    .unwrap_or(info.len());
  let rest = info[end..].trim();
  let inner = rest
    .strip_prefix('{')
    .and_then(|s| s.strip_suffix('}'))
    .unwrap_or(rest);

  let mut is_jsx = false;
  let mut width = String::from("100%");
  let mut height = String::from("400px");

  for token in inner.split(',') {
    let token = token.trim();
    if token.is_empty() {
      continue;
    }
    let (key, value) = match token.split_once('=') {
      Some((k, v)) => (
        k.trim(),
        v.trim().trim_matches('"').trim_matches('\'').to_string(),
      ),
      None => (token, String::from("true")),
    };
    match key {
      "jsxgraph" => is_jsx = matches!(value.as_str(), "true" | "1" | ""),
      "width" => width = value,
      "height" => height = value,
      _ => {}
    }
  }

  if !is_jsx {
    return None;
  }

  if !width.is_empty() && width.chars().all(|c| c.is_ascii_digit()) {
    width.push_str("px");
  }
  if !height.is_empty() && height.chars().all(|c| c.is_ascii_digit()) {
    height.push_str("px");
  }

  Some((width, height))
}

fn escape_html_text(s: &str) -> String {
  let mut out = String::with_capacity(s.len());
  for c in s.chars() {
    match c {
      '<' => out.push_str("&lt;"),
      '>' => out.push_str("&gt;"),
      '&' => out.push_str("&amp;"),
      c => out.push(c),
    }
  }
  out
}

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
  pub source: String,
}

#[component]
pub fn Markdown(props: &MarkdownProps) -> Html {
  let node_ref = use_node_ref();
  let html_string = render_to_html(&props.source);

  #[cfg(target_arch = "wasm32")]
  {
    let node_ref = node_ref.clone();
    let dep = html_string.clone();
    use_effect_with(dep, move |_| {
      if let Some(el) = node_ref.cast::<Element>() {
        // Initialize JSXGraph boards first so neither highlight.js nor KaTeX
        // touches the staged source.
        init_jsxgraph_blocks(&el);

        // Syntax-highlight every <pre><code> block before KaTeX runs,
        // so KaTeX can ignore <pre>/<code> as designed.
        if let Ok(blocks) = el.query_selector_all("pre code") {
          for i in 0..blocks.length() {
            if let Some(node) = blocks.item(i)
              && let Ok(code_el) = node.dyn_into::<Element>()
            {
              highlight_element(&code_el);
            }
          }
        }

        if let Ok(math_blocks) = el.query_selector_all(".math") {
          for i in 0..math_blocks.length() {
            if let Some(node) = math_blocks.item(i)
              && let Ok(math_el) = node.dyn_into::<Element>()
            {
              let class_name = math_el.class_name();
              let is_display = class_name.split_whitespace().any(|c| c == "math-display");
              let latex = math_el.text_content().unwrap_or_default();

              let katex_options = js_sys::Object::new();
              let _ =
                js_sys::Reflect::set(&katex_options, &"displayMode".into(), &is_display.into());
              let _ = js_sys::Reflect::set(&katex_options, &"throwOnError".into(), &false.into());

              katex_render(&latex, &math_el, &katex_options);
            }
          }
        }
      }
      || ()
    });
  }

  html! {
    <div ref={node_ref} class="markdown">
      { Html::from_html_unchecked(AttrValue::from(html_string)) }
    </div>
  }
}

#[cfg(target_arch = "wasm32")]
fn init_jsxgraph_blocks(root: &Element) {
  let Ok(blocks) = root.query_selector_all("pre.jsxgraph-source[data-jxg-id]") else {
    return;
  };

  for i in 0..blocks.length() {
    let Some(node) = blocks.item(i) else {
      continue;
    };
    let Ok(pre) = node.dyn_into::<Element>() else {
      continue;
    };
    let Some(id) = pre.get_attribute("data-jxg-id") else {
      continue;
    };
    let source = pre.text_content().unwrap_or_default();

    let script = format!("(function(BOARDID, id) {{\n{source}\n}})({id:?}, {id:?});");
    let _ = js_sys::eval(&script);

    if let Some(parent) = pre.parent_element() {
      let _ = parent.remove_child(&pre);
    }
  }
}
