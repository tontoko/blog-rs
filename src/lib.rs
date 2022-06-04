use yew_router::Routable;

pub mod components {
    pub mod layouts {
        pub mod blog_layout;
    }
    pub mod pages {
        pub mod home;
        pub mod post;
    }
    pub mod atoms {
        pub mod blog_list;
    }
}
pub mod hooks {
    pub mod use_stories;
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
