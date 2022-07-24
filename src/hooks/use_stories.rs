use serde::Deserialize;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, SuspensionResult, UseFutureHandle};

use crate::domain::story::Story;

#[derive(Deserialize, Debug, Clone)]
pub struct StoriesRes {
    pub stories: Vec<Story>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UseStoriesReturn {
    pub stories_res: StoriesRes,
    pub total_posts: i64,
}

async fn fetch_stories(url: String) -> Result<UseStoriesReturn, reqwasm::Error> {
    let res = reqwasm::http::Request::get(&url).send().await?;
    let total_posts = res.headers().get("total").unwrap_or_default();
    let json = res.json::<StoriesRes>().await?;
    Ok(UseStoriesReturn {
        stories_res: json,
        total_posts: total_posts.parse::<i64>().unwrap_or(0),
    })
}

#[hook]
pub fn use_stories(
    current_page: i32,
    tag: Option<&String>,
) -> SuspensionResult<UseFutureHandle<Result<UseStoriesReturn, reqwasm::Error>>> {
    let version = if option_env!("DEV").is_some() {
        "draft"
    } else {
        "published"
    };
    let unwrapped_tag = if let Some(tag) = tag { tag } else { "" };
    let tag_query = format!("&with_tag={unwrapped_tag}");
    let url = format!("https://api.storyblok.com/v2/cdn/stories?cv=1654353862&token=MqSFcDWDiuLzwkH3h7q4hwtt&version={version}&page={current_page}&per_page=10{tag_query}");

    use_future_with_deps(
        |url| async move { fetch_stories(url.to_string()).await },
        url,
    )
}
