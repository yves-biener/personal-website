use yew::{html, Properties, function_component, Html};

use super::{Link, A};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub links: Vec<A>,
}

#[function_component(Header)]
pub fn header(HeaderProps { links }: &HeaderProps) -> Html {
    let links: Html = links.iter().map(|a| html! {
	<li><Link {a} /></li>
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
