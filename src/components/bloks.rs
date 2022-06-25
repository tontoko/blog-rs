use core::panic;
use std::str::FromStr;

use serde::Deserialize;
use strum::{Display, EnumString};
use yew::prelude::*;

use crate::{hooks::use_stories::Content, HtmlComment};

#[function_component(Feature)]
pub fn feature(Content { text, .. }: &Content) -> Html {
    html! {
        <p>{text.clone().unwrap_or_default()}</p>
    }
}

#[function_component(Grid)]
pub fn grid(Content { columns, .. }: &Content) -> Html {
    html! {
        <div class="w-full grid grid-flow-col auto-cols-auto">{for columns.clone().unwrap().iter().map(|blok| html! {
            <>
                <HtmlComment comment={blok._editable.clone().unwrap_or_default()} />
                {Bloks::render(blok)}
            </>
        })}
        </div>
    }
}

#[function_component(Teaser)]
pub fn teaser(Content { text, .. }: &Content) -> Html {
    html! {
        <h2>{text.clone().unwrap_or_default()}</h2>
    }
}

#[function_component(ImageWithDescription)]
pub fn image_with_description(Content { image, text, .. }: &Content) -> Html {
    if image.is_none() {
        log::info!("image is none");
        return html! {<></>};
    }
    let image = image.clone().unwrap();
    html! {
      <div class="flex flex-col">
        <img src={image.filename} alt={image.alt} class={if text.is_some() {"mb-0"} else {""}} />
        <p class="mt-1">{text.clone().unwrap_or_default()}</p>
      </div>
    }
}

#[derive(Display, EnumString, Deserialize, Debug, Clone, PartialEq)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Bloks {
    Feature,
    Grid,
    Teaser,
    ImageWithDescription,
    Page,
}
impl Bloks {
    pub fn render(props: &Content) -> Html {
        let component = Self::from_str(&props.component.to_string()).unwrap();
        match component {
            Bloks::Feature => html! {<Feature ..props.clone() />},
            Bloks::Grid => html! {<Grid ..props.clone() />},
            Bloks::Teaser => html! {<Teaser ..props.clone() />},
            Bloks::ImageWithDescription => html! {<ImageWithDescription ..props.clone() />},
            Bloks::Page => panic!("never"),
        }
    }
}
