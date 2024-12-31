use yew::{classes, function_component, html, Html};

#[function_component]
pub fn AboutPanel() -> Html {
  html! {
    <div class={classes!("about")}>
      <h2>{"Who am I?"}</h2>
      <p>{"I am a 27 year old Computer Science graduate from Queen's University."}</p>
      <h2>{"What do I do?"}</h2>
      <p>{"I am currently working as an Engineering Lead at Adeptmind. In my spare time I make games, study plants and produce music."}</p>
      <h2>{"Contact me"}</h2>
      <div class="contact">
        <div  class={classes!("contact__icon", "icon")}>
          <svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path fill="currentColor" d="M464 64C490.5 64 512 85.49 512 112C512 127.1 504.9 141.3 492.8 150.4L275.2 313.6C263.8 322.1 248.2 322.1 236.8 313.6L19.2 150.4C7.113 141.3 0 127.1 0 112C0 85.49 21.49 64 48 64H464zM217.6 339.2C240.4 356.3 271.6 356.3 294.4 339.2L512 176V384C512 419.3 483.3 448 448 448H64C28.65 448 0 419.3 0 384V176L217.6 339.2z"/>
          </svg>
        </div>
        <a href="mailto:alegemaate@gmail.com"><p>{"alegemaate@gmail.com"}</p></a>
      </div>
    </div>
  }
}
