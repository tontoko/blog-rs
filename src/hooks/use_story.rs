use crate::hooks::use_stories::Story;
use serde::Deserialize;
use yew::prelude::*;
use yew::suspense::{use_future, Suspension, UseFutureHandle};

#[derive(Deserialize, Debug, Clone)]
pub struct StoryRes {
    pub story: Story,
}

#[hook]
pub fn use_story(
    name: String,
) -> Result<UseFutureHandle<Result<StoryRes, reqwasm::Error>>, Suspension> {
    let version = if option_env!("DEV").is_some() {
        "draft"
    } else {
        "published"
    };
    let url = format!("https://api.storyblok.com/v2/cdn/stories/{name}?cv=1654365742&token=MqSFcDWDiuLzwkH3h7q4hwtt&version={version}");
    use_future(|| async move {
        reqwasm::http::Request::get(&url)
            .send()
            .await?
            .json::<StoryRes>()
            .await
    })
}
