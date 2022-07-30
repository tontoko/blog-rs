use std::collections::HashMap;

use serde::Serialize;
use yew::{prelude::*};
use yew_router::{hooks::{use_location, use_navigator}, components::Link};

use crate::{hooks::use_stories::{use_stories}, Route};
use crate::{domain::story::RichTextContent };

#[derive(Clone ,PartialEq, Serialize)]
struct QueryParams {
    page: String
}

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub onclick: Callback<String>,
}

fn rich_text_description_builder(content: &RichTextContent) -> String {
    match content {
        RichTextContent::Paragraph { content } => {
            if content.is_none() {return "".to_string()}
            content.as_ref().unwrap().iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        },
        RichTextContent::Text { text, .. } => {
            text.to_string()
        }
        RichTextContent::HorizontalRule => "".to_string(),
        RichTextContent::Image { .. } => "".to_string(),
        RichTextContent::Blok { .. } => "".to_string(),
        RichTextContent::Doc { content } => {
            content.iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        }
        RichTextContent::Heading {  content,.. } => {
            content.iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        }
        RichTextContent::BulletList { content } =>   {
            content.iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        }
        RichTextContent::ListItem { content } =>   {
            content.iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        }
        RichTextContent::OrderedList { content, .. } => {
            content.iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        }
        RichTextContent::Blockquote { content } =>{
            content.iter().map(rich_text_description_builder).collect::<Vec<_>>().join("\n")
        }
    }
}

#[function_component(BlogList)]
pub fn blog_list(BlogListProps { onclick }: &BlogListProps) -> HtmlResult {
    let location = use_location();
    let navigator = use_navigator();

    let query;
    let mut current_page = 1;
    let mut tag = None;
    if let Some(location) = location {
        query = location.query::<HashMap<String, String>>().unwrap_or_default();
        current_page = query.get("page").unwrap_or(&String::from("1")).parse().unwrap_or(1);
        tag = query.get("tag");
    };
    
    let res = use_stories(current_page, tag)?;

    let handle_click = |name: String| {
        let onclick = onclick.clone();
        Callback::from(move |_| onclick.emit(name.clone()))
    };
    let result_item_html: Html = match *res {
        Ok(ref res) => res.stories_res.stories.iter().map(|story| {
            let description = &story.content.body.as_ref().unwrap_or(&Vec::new()).iter().map(|body| {
                if let Some(text) = &body.text {
                    text.clone()
                } else if let Some(rich_text) = &body.rich_text {
                    rich_text_description_builder(rich_text)         
                } else {
                    "".to_string()
                }
            }).collect::<Vec<_>>().join("\n");
            html! {
                <div class="flex flex-col border border-slate-400 rounded-lg p-3 min-h-[100px] cursor-pointer" onclick={handle_click(story.name.to_lowercase().clone())}>
                    <h4>{&story.name}</h4>
                    <p class="whitespace-pre-line line-clamp-4 md:line-clamp-6">{description}</p>
                </div>        
            }}
        ).collect(),
        Err(ref failure) => failure.to_string().into(),
    };

    let result_pagination_html: Html = match *res {
        Ok(ref res) => {
            let total_posts = res.total_posts;
            let total_pages = ((total_posts as f32) / 10.0).ceil();

            if current_page as f32 > total_pages {
                navigator.clone().unwrap().push_with_query(&Route::Home, &QueryParams {page: total_pages.to_string()}).unwrap_or_default();
            }
            if current_page < 1 {
                navigator.unwrap().push_with_query(&Route::Home, &QueryParams {page: "1".to_string()}).unwrap_or_default();
            }

            let pages = format!("{current_page}/{total_pages}");
            html! {
                <div class="flex gap-3 w-full justify-center mt-2">
                    <Link<Route, QueryParams> to={Route::Home} query={QueryParams {page: (current_page-1).to_string()}}>{"前へ"}</Link<Route, QueryParams>>
                    <p>{pages}</p>
                    <Link<Route, QueryParams> to={Route::Home} query={QueryParams {page: (current_page+1).to_string()}}>{"次へ"}</Link<Route, QueryParams>>
                </div>
        }},
        Err(ref failure) => {
            log::error!("{}", failure);
            html!(500)},
    };

    Ok(html! { 
        <>
            <div class="flex flex-col gap-5 w-full">
                {result_item_html}
            </div>
            <div class="flex flex-col w-full items-center mt-2">
                {result_pagination_html}
            </div>   
        </>     
    })
}
