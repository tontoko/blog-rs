use yew::prelude::*;

use crate::hooks::use_stories::use_stories;

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(BlogList)]
pub fn blog_list(BlogListProps { onclick }: &BlogListProps) -> HtmlResult {
    let blogs = use_stories()?;
    let result_html = match *blogs {
        Ok(ref res) => html! {<>{for res.stories.iter().map(|story| {
        html! {
            <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer" onclick={onclick}>
            <p>{&story.name}</p>
            <p>{"text......"}</p>
            </div>
        }
        })}</>},
        Err(ref failure) => failure.to_string().into(),
    };

    Ok(result_html)
}
