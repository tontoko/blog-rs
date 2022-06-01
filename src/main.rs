use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct BlogLayoutProps {
    children: Children,
}

#[function_component(BlogLayout)]
fn blog_layout(BlogLayoutProps { children }: &BlogLayoutProps) -> Html {
    html! {
        <>
            <div class="flex flex-col w-full items-center">
                <div class="flex h-20 justify-center items-center"><h1>{"blog"}</h1></div>
                <div class="flex w-full lg:w-4/5 px-4 flex-col justify-center">
                    {for children.iter()}
                </div>
            </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
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
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
