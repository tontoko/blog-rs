use crate::domain::story::Story;
use serde::Deserialize;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, SuspensionResult, UseFutureHandle};

#[derive(Deserialize, Debug, Clone)]
pub struct StoryRes {
    pub story: Story,
}

#[hook]
pub fn use_story(
    name: String,
) -> SuspensionResult<UseFutureHandle<Result<StoryRes, reqwasm::Error>>> {
    let version = if option_env!("DEV").is_some() {
        "draft"
    } else {
        "published"
    };
    let lowered_name = name.to_lowercase();
    let url = format!("https://api.storyblok.com/v2/cdn/stories/{lowered_name}?cv=1654365742&token=MqSFcDWDiuLzwkH3h7q4hwtt&version={version}");
    use_future_with_deps(
        |url| async move {
            reqwasm::http::Request::get(&url)
                .send()
                .await?
                .json::<StoryRes>()
                .await
        },
        url,
    )
}
