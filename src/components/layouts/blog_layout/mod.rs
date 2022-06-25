use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct BlogLayoutProps {
    pub children: Children,
}

#[function_component(BlogLayout)]
pub fn blog_layout(BlogLayoutProps { children }: &BlogLayoutProps) -> Html {
    let history = use_navigator().unwrap();
    let onclick = || {
        Callback::from(move |_| {
            history.push(&Route::Home);
        })
    };
    html! {
        <div class="prose max-w-full flex flex-col w-full items-center break-words my-10">
            <div class="flex h-20 justify-center items-center">
                <h1 class="cursor-pointer" onclick={onclick()}>{"Blog"}</h1>
            </div>
            <div class="flex w-full md:w-4/5 lg:w-3/5 px-4 flex-col justify-center">
                {for children.iter()}
            </div>
        </div>
    }
}
