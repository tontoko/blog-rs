use yew::prelude::*;

use crate::{hooks::use_story::use_story, HtmlComment};

#[function_component(Feature)]
pub fn feature() -> Html {
    html! {
        <h1></h1>
    }
}

#[function_component(Grid)]
pub fn grid() -> Html {
    html! {
        <div></div>
    }
}

#[function_component(Teaser)]
pub fn teaser() -> Html {
    html! {
        <h1></h1>
    }
}

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
                    <div class="flex flex-col items-center">
                        <h1 class="font-bold text-3xl mb-4">{&story.name}</h1>
                        {for story.content.body.iter().map(|blok| html! {
                            <>
                                <HtmlComment comment={blok.get("_editable").unwrap_or(&"".to_string()).clone()} />
                                <div class="blok">
                                    {blok.get("name").unwrap_or(&"".to_string())}
                                </div>
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
