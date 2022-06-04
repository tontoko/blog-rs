use crate::components::layouts::blog_layout::BlogLayout;
use yew::prelude::*;

#[function_component(Post)]
pub fn post() -> Html {
    html! {
        <BlogLayout>
            <div class="flex flex-col gap-2 w-full">
              {"Article"}
            </div>
        </BlogLayout>
    }
}
