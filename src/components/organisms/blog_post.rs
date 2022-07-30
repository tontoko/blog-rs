use serde::Serialize;
use yew::prelude::*;

use crate::{components::bloks::Bloks, hooks::use_story::use_story, HtmlComment, Route};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub name: String,
}

#[derive(Clone, PartialEq, Serialize)]
struct TagQueryParams {
    tag: String,
}

#[function_component(BlogPost)]
pub fn blog_post(BlogPostProps { name }: &BlogPostProps) -> HtmlResult {
    let res = use_story(name.to_string())?;
    let result_html = match *res {
        Ok(ref story_res) => {
            let story = &story_res.story;
            html! {
                <>
                    <HtmlComment comment={story.content._editable.clone().unwrap_or_default()} />
                    <div class="flex flex-col">
                        <h2 class="text-center">{&story.name}</h2>
                        {for story.content.body.as_ref().unwrap().iter().map(|blok| html! {
                            <>
                                <HtmlComment comment={blok._editable.clone().unwrap_or_default()} />
                                {Bloks::render(blok)}
                            </>
                        })}
                        {if !story.tag_list.is_empty() { html! {
                            <>
                                <div class="w-full border-t-2 my-2" />
                                <div class="flex row gap-2">
                                    <div>{"Tags: "}</div>
                                    {for story.tag_list.iter().map(|tag| html! {
                                        <Link<Route, TagQueryParams>
                                            to={Route::Home}
                                            query={TagQueryParams {tag: tag.to_string()}}
                                        >
                                            <div>{tag}</div>
                                        </Link<Route, TagQueryParams>>
                                    })}
                                </div>
                            </>
                        }}
                        else {html!()}
                        }
                    </div>
                </>
            }
        }
        Err(ref failure) => {
            log::error!("{}", failure);
            html!(500)
        }
    };

    Ok(result_html)
}
