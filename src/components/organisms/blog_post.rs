use yew::prelude::*;

use crate::{components::blok::Bloks, hooks::use_story::use_story, HtmlComment};

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub name: String,
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
                        <div class="flex justify-center">
                            <h1 class="font-bold text-3xl mb-4">{&story.name}</h1>
                        </div>
                        {for story.content.body.as_ref().unwrap().iter().map(|blok| html! {
                            <>
                                <HtmlComment comment={blok._editable.clone().unwrap_or_default()} />
                                {Bloks::render(blok)}
                            </>
                        })}
                    </div>
                </>
            }
        }
        Err(ref failure) => failure.to_string().into(),
    };

    Ok(result_html)
}
