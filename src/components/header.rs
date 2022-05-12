use yew::{html, Properties, function_component, Html};

use super::{Link, LinkProps};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub links: Vec<LinkProps>,
}

#[function_component(Header)]
pub fn header(HeaderProps { links }: &HeaderProps) -> Html {
    let links: Html = links.iter().map(|l| html! {
	<li><Link name={l.name.clone()} href={l.href.clone()} /></li>
    }).collect();
    html! {
        <header>
	  <nav>
            <ul style="padding:0;">
              { links }
            </ul>
	  </nav>
        </header>
    }
}
