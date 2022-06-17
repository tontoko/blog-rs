use std::collections::HashMap;

use serde::Serialize;
use yew::{prelude::*};
use yew_router::{hooks::{use_location, use_navigator}, components::Link};

use crate::{hooks::use_stories::use_stories, Route};

#[derive(Clone ,PartialEq, Serialize)]
struct QueryParams {
    page: String
}

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub onclick: Callback<String>,
}

#[function_component(BlogList)]
pub fn blog_list(BlogListProps { onclick }: &BlogListProps) -> HtmlResult {
    let location = use_location();
    let navigator = use_navigator();

    let query;
    let mut current_page = 1;
    if let Some(location) = location {
        query = location.query::<HashMap<String, String>>();
        current_page = query.unwrap_or_default().get("page").unwrap_or(&String::from("1")).parse().unwrap_or(1);
    };
    
    let res = use_stories(current_page)?;

    let handle_click = |name: String| {
        let onclick = onclick.clone();
        Callback::from(move |_| onclick.emit(name.clone()))
    };
    let result_item_html: Html = match *res {
        Ok(ref res) => res.stories_res.stories.iter().map(|story| {
            let description = &story.content.body.as_ref().unwrap_or(&Vec::new()).iter().map(|body| {
                body.text.as_ref().unwrap_or(&String::from("")).clone()
            }).collect::<Vec<_>>().join(" ");
            html! {
                <div class="flex flex-col border border-slate-400 rounded-lg p-2 min-h-[100px] cursor-pointer" onclick={handle_click(story.name.to_lowercase().clone())}>
                    <p>{&story.name}</p>
                    <p class="line-clamp-1 md:line-clamp-2">{description}</p>
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
            <>
                <p>{"ページ"}</p>
                <div class="flex gap-3 w-full justify-center mt-2">
                    <Link<Route, QueryParams> to={Route::Home} query={QueryParams {page: (current_page-1).to_string()}}>{"前へ"}</Link<Route, QueryParams>>
                    <p>{pages}</p>
                    <Link<Route, QueryParams> to={Route::Home} query={QueryParams {page: (current_page+1).to_string()}}>{"次へ"}</Link<Route, QueryParams>>
                </div>
            </>
        }},
        Err(ref failure) => failure.to_string().into(),
    };

    Ok(html! { 
        <>
            <div class="flex flex-col gap-2 w-full">
                {result_item_html}
            </div>
            <div class="flex flex-col w-full items-center mt-2">
                {result_pagination_html}
            </div>   
        </>     
    })
}
