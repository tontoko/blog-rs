use yew::{function_component, html, Html};

use crate::domain::story::Content;

#[function_component(Teaser)]
pub fn teaser(Content { text, .. }: &Content) -> Html {
    html! {
        <h2>{text.clone().unwrap_or_default()}</h2>
    }
}
