use serde::Deserialize;
use strum::EnumString;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, Suspension, UseFutureHandle};

use crate::components::bloks::Bloks;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Asset {
    pub id: i64,
    pub alt: String,
    pub name: String,
    pub focus: String,
    pub title: String,
    pub filename: String,
    pub copyright: Option<String>,
    pub fieldtype: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Marks {
    Bold,
    Strike,
    Underline,
    Italic,
    Link { attrs: LinkAttrs },
}

#[derive(Deserialize, Debug, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LinkType {
    Story,
    Url,
    Asset,
    Email,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LinkAttrs {
    pub href: String,
    pub uuid: Option<String>, // only story
    pub anchor: Option<String>,
    pub target: String, // _self or _blank
    pub linktype: LinkType,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ImageAttrs {
    pub alt: String,
    pub src: String,
    pub title: String,
    pub copyright: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeadingAttrs {
    pub level: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct OrderedListAttrs {
    pub order: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BlokAttrs {
    pub id: String,
    pub body: Vec<Content>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextContent {
    Doc {
        content: Vec<RichTextContent>,
    },
    Heading {
        attrs: HeadingAttrs,
        content: Vec<RichTextContent>,
    },
    Paragraph {
        content: Option<Vec<RichTextContent>>,
    },
    BulletList {
        content: Vec<RichTextContent>,
    },
    ListItem {
        content: Vec<RichTextContent>,
    },
    Text {
        text: String,
        marks: Option<Vec<Marks>>,
    },
    OrderedList {
        content: Vec<RichTextContent>,
        attrs: OrderedListAttrs,
    },
    Blockquote {
        content: Vec<RichTextContent>,
    },
    HorizontalRule,
    Image {
        attrs: ImageAttrs,
    },
    Blok {
        attrs: BlokAttrs,
    },
}

#[derive(Deserialize, Debug, Clone, Properties, PartialEq)]
pub struct Content {
    pub component: Bloks,
    pub _editable: Option<String>,
    pub body: Option<Vec<Content>>,
    pub text: Option<String>,
    pub rich_text: Option<RichTextContent>,
    pub image: Option<Asset>,
    pub columns: Option<Vec<Content>>,
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
    pub published_at: Option<String>,
    pub first_published_at: Option<String>,
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
) -> Result<UseFutureHandle<Result<UseStoriesReturn, reqwasm::Error>>, Suspension> {
    let version = if option_env!("DEV").is_some() {
        "draft"
    } else {
        "published"
    };
    let url = format!("https://api.storyblok.com/v2/cdn/stories?cv=1654353862&token=MqSFcDWDiuLzwkH3h7q4hwtt&version={version}&page={current_page}&per_page=10");

    use_future_with_deps(
        |url| async move { fetch_stories(url.to_string()).await },
        url,
    )
}
