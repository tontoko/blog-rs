use crate::{
    components::{layouts::blog_layout::BlogLayout, organisms::blog_list::BlogList},
    Route,
};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_navigator().unwrap();
    let onclick = Callback::from(move |id: String| {
        history.push(&Route::Post { id });
    });
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <BlogLayout>
            <Suspense {fallback}>
                <BlogList {onclick} />
            </Suspense>
        </BlogLayout>
    }
}
