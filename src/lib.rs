mod components;

use components::{
    about_me::AboutMe,
    footer::Footer,
    header::Header,
    projects::{Project, Projects},
    LinkProps,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let header_links = vec![
        LinkProps::new("About Me", "#"),
        LinkProps::new("Projects", "#"),
        LinkProps::new("Contact", "#"),
    ];
    let projects = vec![Project::new(
        "Quote-Reminder",
        // TODO: maybe I should move this out into a separate file and read the
        // content instead
        r"I faced the problem of remembering what I read into detail. I wanted
	to make the time I read books more productive and gain more insights
	from my reading for a longer time than just a month after reading the
	book. In order to keep the ideas of the book sticking in by head I
	created this tool to store all quotes which resonate with me into a
	small database and remind myself through regular emails with random
	quotes. I also combined it this project with the fact that I wanted to
	learn a bit of go. As it turns out go was a really good choice for such
	a quick project, as the language is easy to pick up and performs quite
	well. Very quickly the language becomes second nature as I could simply
	guess how the syntax would be without having to check for correctness,
	as the guesses were correct. This reduced the mental overhead of
	learning and remembering the syntax of the language while also solving
	problems. You see a web view of the database content of all the books I
	created quotes for. Maybe there are some books you have also read or you
	should check out :)",
        "",
        "https://github.com/yves-biener/quote-reminder/",
    )];
    let about_me_content = r"Welcome, I'm Yves! I strive to become a
    rusteriant, learn and improve at every opportunity I get. This website is
    still work in progress. You will pretty much only read this when you are
    skimming through the history of this repository over on github. So let me ask
    you: What are you doing reading this?";
    let about_me_image_src = "";
    let linked_in = LinkProps::new(
        "Linked In",
        "https://www.linkedin.com/in/yves-biener-3568a2225/",
    );
    let github = LinkProps::new("Github", "https://github.com/yves-biener/");
    let mail_to = LinkProps::new(
        "Mail",
        "mailto:yves.biener@gmx.de?subject=\"I reach out to you from your website\"",
    );
    html! {
      <>
        <Header links={ header_links } />
        <AboutMe content={ about_me_content } image_src={ about_me_image_src } />
        <Projects {projects} />
        <Footer linked_in={linked_in} github={github} mail_to={mail_to} />
      </>
    }
}
