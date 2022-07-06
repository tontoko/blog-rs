use yew::{function_component, html, Html};

use crate::{components::bloks::Bloks, hooks::use_stories::Content, HtmlComment};

#[function_component(Grid)]
pub fn grid(Content { columns, .. }: &Content) -> Html {
    html! {
        <div class="w-full grid grid-flow-col auto-cols-auto">{for columns.clone().unwrap().iter().map(|blok| html! {
            <>
                <HtmlComment comment={blok._editable.clone().unwrap_or_default()} />
                {Bloks::render(blok)}
            </>
        })}
        </div>
    }
}
