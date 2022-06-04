use crate::{components::layouts::blog_layout::BlogLayout, Route};
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| {
        history.push(Route::Post);
        log::info!("clicked");
    });
    html! {
        <BlogLayout>
            <div class="flex flex-col gap-2 w-full">
                {for (1..=5).map(|_| {
                    html! {
                        <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer" onclick={&onclick}>
                        <p>{"タイトル"}</p>
                        <p>{"説明..."}</p>
                        </div>
                    }
                })}
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
