use yew::{Properties, Html, html, function_component};

use super::{LinkProps, Link};

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub linked_in: LinkProps,
    pub github: LinkProps,
    pub mail_to: LinkProps,
}

#[function_component(Footer)]
pub fn footer(FooterProps { linked_in, github, mail_to }: &FooterProps) -> Html {
    let links: Html = [linked_in, github, mail_to].iter().map(|link| html! {
	<li><Link name={link.name.clone()} href={link.href.clone()} /></li>
    }).collect();
    html! {
	<footer>
	    <ul style="padding:0;">
	        { links }
	    </ul>
	</footer>
    }
}
