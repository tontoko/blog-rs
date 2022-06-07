use yew::prelude::*;

use crate::hooks::use_stories::use_stories;

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub onclick: Callback<String>,
}

#[function_component(BlogList)]
pub fn blog_list(BlogListProps { onclick }: &BlogListProps) -> HtmlResult {
    let res = use_stories()?;
    let handle_click = |name: String| {
        let onclick = onclick.clone();
        Callback::from(move |_| onclick.emit(name.clone()))
    };
    let result_html = match *res {
        Ok(ref stories_res) => stories_res.stories.iter().map(|story| 
            html! {
                <div class="flex flex-col border border-slate-400 rounded p-2 min-h-[100px] cursor-pointer" onclick={handle_click(story.name.clone())}>
                <p>{&story.name}</p>
                <p>{"text......"}</p>
                </div>
            }
        ).collect(),
        Err(ref failure) => failure.to_string().into(),
    };

    Ok(result_html)
}
