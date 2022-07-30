use crate::components::{layouts::blog_layout::BlogLayout, organisms::blog_post::BlogPost};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: String,
}

#[function_component(Post)]
pub fn post(PostProps { id }: &PostProps) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
        <BlogLayout>
            <Suspense {fallback}>
                <BlogPost name={id.clone()} />
            </Suspense>
        </BlogLayout>
    }
}
