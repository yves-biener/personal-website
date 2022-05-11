use yew::{function_component, html::ImplicitClone, Properties};
use yew::html;

pub mod about_me;
pub mod contact;
pub mod footer;
pub mod header;
pub mod projects;
pub mod timeline;

#[derive(Clone, PartialEq)]
pub struct A {
    name: String,
    href: String,
}

impl A {
    pub fn new(name: &str, href: &str) -> Self {
	A { name: name.into(), href: href.into() }
    }
}

impl ImplicitClone for A {}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    a: A,
}

#[function_component(Link)]
pub fn link(LinkProps { a }: &LinkProps) -> Html {
    html! {
        <a href={ a.href.clone() }>{ a.name.clone() }</a>
    }
}
