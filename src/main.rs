use blog::components::pages;
use blog::Route;
use pages::home::Home;
use pages::post::Post;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Post => html! { <Post /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
