use crate::components::{layouts::blog_layout::BlogLayout, organisms::blog_post::BlogPost};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub name: String,
}

#[function_component(Post)]
pub fn post(PostProps { name }: &PostProps) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
        <BlogLayout>
            <Suspense {fallback}>
                <BlogPost name={name.clone()} />
            </Suspense>
        </BlogLayout>
    }
}
