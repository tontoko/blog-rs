use std::collections::HashMap;

use serde::Deserialize;
use yew::prelude::*;
use yew::suspense::{use_future, Suspension, UseFutureHandle};

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    pub component: String, // and fields you define yourself are in here
    pub _editable: Option<String>,
    pub body: Vec<HashMap<String, String>>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct TranslatedSlug {
    pub path: String,
    pub name: String,
    pub lang: String,
    // only gets included if the translatable slug app is installed
}
#[derive(Deserialize, Debug, Clone)]
pub struct Alternate {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub full_slug: String,
    pub is_folder: bool,
    pub parent_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Story {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub full_slug: String,
    pub default_full_slug: Option<String>,
    pub created_at: String,
    pub published_at: String,
    pub first_published_at: String,
    pub release_id: Option<String>,
    pub lang: String,
    pub content: Content,
    pub position: i64,
    pub is_startpage: bool,
    pub parent_id: i64,
    pub group_id: String,
    pub translated_slugs: Option<Vec<TranslatedSlug>>,
    pub alternates: Vec<Alternate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StoriesRes {
    pub stories: Vec<Story>,
}

#[hook]
pub fn use_stories() -> Result<UseFutureHandle<Result<StoriesRes, reqwasm::Error>>, Suspension> {
    let version = if option_env!("DEV").is_some() {
        "draft"
    } else {
        "published"
    };
    let url = format!("https://api.storyblok.com/v2/cdn/stories?cv=1654353862&token=MqSFcDWDiuLzwkH3h7q4hwtt&version={version}");
    use_future(|| async move {
        reqwasm::http::Request::get(&url)
            .send()
            .await?
            .json::<StoriesRes>()
            .await
    })
}
