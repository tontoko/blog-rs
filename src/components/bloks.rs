use core::panic;
use std::str::FromStr;

use serde::Deserialize;
use strum::{Display, EnumString};
use yew::prelude::*;

use crate::domain::story::Content;

use super::{
    atoms::{grid::Grid, teaser::Teaser},
    molecules::{feature::Feature, image_with_description::ImageWithDescription},
};

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
