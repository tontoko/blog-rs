use regex::Regex;
use yew::prelude::*;
use yew_router::Routable;

pub mod components {
    pub mod bloks;
    pub mod layouts {
        pub mod blog_layout;
    }
    pub mod pages {
        pub mod home;
        pub mod post;
    }
    pub mod organisms {
        pub mod blog_list;
        pub mod blog_post;
    }
    pub mod atoms {
        pub mod grid;
        pub mod teaser;
    }
    pub mod molecules {
        pub mod feature;
        pub mod image_with_description;
    }
}
pub mod hooks {
    pub mod use_space;
    pub mod use_stories;
    pub mod use_story;
}
pub mod domain {
    pub mod story;
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/posts/:id")]
    Post { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
pub struct HtmlCommentProps {
    pub comment: String,
}

#[function_component(HtmlComment)]
pub fn html_comment(props: &HtmlCommentProps) -> Html {
    let captured = Regex::new("<!--(.+?)-->").unwrap().captures(&props.comment);
    let raw_text = if captured.is_some() {
        captured.unwrap().get(1).unwrap().as_str()
    } else {
        ""
    };
    if raw_text.is_empty() {
        return html! {<></>};
    };

    let comment = gloo::utils::document().create_comment(raw_text);

    Html::VRef(comment.into())
}
