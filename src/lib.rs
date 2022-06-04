use yew_router::Routable;

pub mod components {
    pub mod layouts {
        pub mod blog_layout;
    }
    pub mod pages {
        pub mod home;
        pub mod post;
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/post")]
    Post,
    #[not_found]
    #[at("/404")]
    NotFound,
}
