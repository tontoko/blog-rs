use crate::components::layouts::blog_layout::BlogLayout;
use yew::prelude::*;

#[function_component(List)]
pub fn list() -> Html {
    html! {
        <BlogLayout>
            <div class="flex flex-col gap-2 w-full">
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer">
                    <p>{"タイトル"}</p>
                    <p>{"説明..."}</p>
                </div>
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer">
                    <p>{"タイトル"}</p>
                    <p>{"説明..."}</p>
                </div>
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer">
                    <p>{"タイトル"}</p>
                    <p>{"説明..."}</p>
                </div>
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer">
                    <p>{"タイトル"}</p>
                    <p>{"説明..."}</p>
                </div>
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer">
                    <p>{"タイトル"}</p>
                    <p>{"説明..."}</p>
                </div>
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer">
                    <p>{"タイトル"}</p>
                    <p>{"説明..."}</p>
                </div>
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
