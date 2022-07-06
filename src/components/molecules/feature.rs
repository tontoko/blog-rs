use crate::{
    domain::story::{Content, LinkAttrs, LinkType, Marks, RichTextContent},
    Route,
};
use yew::{classes, function_component, html, Html, Properties};
use yew_router::components::Link;

use crate::components::bloks::Bloks;

#[derive(Properties, PartialEq)]
pub struct RichTextProps {
    pub content: RichTextContent,
}

#[function_component(RichText)]
fn rich_text(RichTextProps { content }: &RichTextProps) -> Html {
    match content {
        RichTextContent::Doc { content } => {
            html! {<>{content.iter().map(|c| html!{<RichText content={c.clone()} />}).collect::<Vec<_>>()}</>}
        }
        RichTextContent::Heading { attrs, content } => {
            let inner = html! {
            <>{
                  content.iter()
                  .map(|c| html!{<RichText content={c.clone()} />})
                  .collect::<Vec<_>>()
              }</>};
            if attrs.level == 1 {
                return html! {<h1>{inner}</h1>};
            }
            if attrs.level == 2 {
                return html! {<h2>{inner}</h2>};
            }
            if attrs.level == 3 {
                return html! {<h3>{inner}</h3>};
            }
            if attrs.level == 4 {
                return html! {<h4>{inner}</h4>};
            }
            if attrs.level == 5 {
                return html! {<h5>{inner}</h5>};
            }
            html! {<h6>{inner}</h6>}
        }
        RichTextContent::Paragraph { content } => html! {
            <p>{
                if let Some(c) = content {
                c.iter()
                .map(|c| html!{<RichText content={c.clone()} />})
                .collect::<Vec<_>>()} else {vec![html!()]}
            }</p>
        },
        RichTextContent::BulletList { content } => html! {
        <ul>{
          content.iter()
          .map(|c| html!{<RichText content={c.clone()} />})
          .collect::<Vec<_>>()
        }</ul>},
        RichTextContent::ListItem { content } => html! {
        <li>{
          content.iter()
          .map(|c| html!{<RichText content={c.clone()} />})
          .collect::<Vec<_>>()
        }</li>},
        RichTextContent::Text { text, marks } => {
            let mut link_attrs: Option<&LinkAttrs> = None;
            let class = if let Some(marks) = marks {
                marks
                    .iter()
                    .map(|m| match m {
                        Marks::Bold => "font-bold",
                        Marks::Strike => "line-through",
                        Marks::Underline => "underline",
                        Marks::Italic => "italic",
                        Marks::Link { attrs } => {
                            link_attrs = Some(attrs);
                            ""
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join(" ")
            } else {
                "".to_string()
            };
            if let Some(link_attrs) = link_attrs {
                let href = match link_attrs.linktype {
                    LinkType::Story => link_attrs.href.split('/').last().unwrap().to_string(),
                    LinkType::Email => format!("mailto:{}", link_attrs.href),
                    _ => link_attrs.href.clone(),
                };
                if let LinkType::Story = link_attrs.linktype {
                    return html! {
                        <Link<Route>
                        classes={classes!(class)}
                        to={Route::Post { name: href }}
                        >{text}</Link<Route>>
                    };
                }
                return html! {
                    <a
                    classes={classes!(class)}
                    href={href.to_string()}
                    target={link_attrs.target.clone()}
                    >{text}</a>
                };
            };
            html! {
                <span class={class}>{text}</span>
            }
        }
        RichTextContent::OrderedList { content, attrs } => html! {
        <ol type={attrs.order.to_string()}>{
          content.iter()
          .map(|c| html!{<RichText content={c.clone()} />})
          .collect::<Vec<_>>()
        }</ol>},
        RichTextContent::Blockquote { content } => html! {
        <blockquote>{
          content.iter()
          .map(|c| html!{<RichText content={c.clone()} />})
          .collect::<Vec<_>>()
        }</blockquote>},
        RichTextContent::HorizontalRule => html! {<div class="w-full border-t-2" />},
        RichTextContent::Image { attrs } => {
            html! {
              <a href={attrs.src.clone()} target="_blank">
                <img class="inline" src={attrs.src.clone()} alt={attrs.alt.clone()} />
              </a>
            }
        }
        RichTextContent::Blok { attrs } => {
            html! {
                <>{
                    attrs.body
                    .iter()
                    .map(|content| { Bloks::render(content) })
                    .collect::<Vec<_>>()
                }</>
            }
        }
    }
}

#[function_component(Feature)]
pub fn feature(
    Content {
        rich_text: rich_text_props,
        ..
    }: &Content,
) -> Html {
    if rich_text_props.is_none() {
        return html!();
    };
    html!(<div><RichText content={rich_text_props.clone().unwrap()} /></div>)
}
