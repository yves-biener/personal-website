use yew::{function_component, html, Html, Properties};

use super::Uri;

#[derive(Clone, PartialEq)]
pub struct Project {
    name: String,
    description: String,
    embedding_link: Uri,
    repository_link: Uri,
}

impl Project {
    pub fn new(name: &str, description: &str, embedding_link: &str, repository_link: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            embedding_link: embedding_link.into(),
            repository_link: repository_link.into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    pub projects: Vec<Project>,
}

#[function_component(Projects)]
pub fn projects(ProjectProps { projects }: &ProjectProps) -> Html {
    let projects: Vec<Html> = projects
        .iter()
        .map(|project| -> Html {
            html! {
                <div>
                  <h2>{ project.name.clone() }</h2>
                  <hr />
                  <p>{ project.description.clone() }</p>
                  <iframe src={ project.embedding_link.clone() } />
                  <a href={ project.repository_link.clone() }>{ "Github" }</a>
                </div>
            }
        })
        .collect();
    html! {
    <>
        { for projects }
    </>
    }
}
