mod components;

use components::{header::Header, A};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let header_links = vec![
        A::new("About Me", "#"),
        A::new("Projects", "#"),
        A::new("Contact", "#"),
    ];
    html! {
      <>
        <Header links={ header_links } />
        <h1>{ "Hello World! I'm starting to like this" }</h1>
	<hr/>
      </>
    }
}
