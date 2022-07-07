use yew::{function_component, html, Html};

use crate::domain::story::Content;

#[function_component(ImageWithDescription)]
pub fn image_with_description(Content { image, text, .. }: &Content) -> Html {
    if image.is_none() {
        log::info!("image is none");
        return html! {<></>};
    }
    let image = image.clone().unwrap();
    html! {
      <div class="flex flex-col">
        <img
        width="320"
        height="180"
        src={image.filename}
        alt={image.alt}
        class={if text.is_some() {"mb-0 h-auto w-full"} else {"h-auto w-full"}}
        />
        <p class="mt-1">{text.clone().unwrap_or_default()}</p>
      </div>
    }
}
