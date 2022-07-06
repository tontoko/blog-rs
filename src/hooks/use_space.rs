use serde::Deserialize;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, SuspensionResult, UseFutureHandle};

#[derive(Deserialize, Debug, Clone)]
pub struct Space {
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub version: i32,
    pub language_codes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpaceRes {
    pub space: Space,
}

#[hook]
pub fn use_space() -> SuspensionResult<UseFutureHandle<Result<SpaceRes, reqwasm::Error>>> {
    let url =
        "https://api.storyblok.com/v2/cdn/spaces/me?cv=1654365742&token=MqSFcDWDiuLzwkH3h7q4hwtt";
    use_future_with_deps(
        |url| async move {
            reqwasm::http::Request::get(&url)
                .send()
                .await?
                .json::<SpaceRes>()
                .await
        },
        url,
    )
}
