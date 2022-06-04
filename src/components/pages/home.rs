use crate::{
    components::{atoms::blog_list::BlogList, layouts::blog_layout::BlogLayout},
    Route,
};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_navigator().unwrap();
    let onclick = Callback::from(move |_e: MouseEvent| {
        history.push(&Route::Post);
        log::info!("clicked");
    });
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <BlogLayout>
            <div class="flex flex-col gap-2 w-full">
                <Suspense {fallback}>
                    <BlogList {onclick} />
                </Suspense>
            </div>
            <div class="flex flex-col w-full items-center mt-2">
                <p>{"ページ"}</p>
                <div class="flex gap-3 w-full justify-center mt-2">
                    <p>{"前へ"}</p>
                    <p>{"1/10"}</p>
                    <p>{"次へ"}</p>
                </div>
            </div>
        </BlogLayout>
    }
}
