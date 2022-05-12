use yew::{function_component, Properties};
use yew::html;

pub mod about_me;
pub mod contact;
pub mod footer;
pub mod header;
pub mod projects;
pub mod timeline;

pub type Uri = String;

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    name: String,
    href: Uri,
}

impl LinkProps {
    pub fn new(name: &str, href: &str) -> Self {
	Self { name: name.into(), href: href.into() }
    }
}

#[function_component(Link)]
pub fn link(LinkProps { name, href }: &LinkProps) -> Html {
    html! {
      <a href={ href.clone() }>{ name.clone() }</a>
    }
}
