use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogLayoutProps {
    pub children: Children,
}

#[function_component(BlogLayout)]
pub fn blog_layout(BlogLayoutProps { children }: &BlogLayoutProps) -> Html {
    html! {
        <div class="flex flex-col w-full items-center">
            <div class="flex h-20 justify-center items-center"><h1>{"blog"}</h1></div>
            <div class="flex w-full lg:w-4/5 px-4 flex-col justify-center">
                {for children.iter()}
            </div>
        </div>
    }
}
