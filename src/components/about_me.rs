use yew::{Properties, function_component, html};

use super::Uri;

#[derive(Properties, PartialEq)]
pub struct AboutMeProps {
    pub content: String,
    pub image_src: Uri,
}

#[function_component(AboutMe)]
pub fn about_me(AboutMeProps { content, image_src }: &AboutMeProps) -> Html {
    html! {
	<>
	    <h1>{ "Hello there follow internet wanderer." }</h1>
	    <p>{ content.clone() }</p>
	    <img src={ image_src.clone() }/>
	</>
    }
}
