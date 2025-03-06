use yew::{html, Html};
use yew_router::prelude::*;
use crate::views::blog::{index::BlogIndex, post::BlogPost};

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/blog")]
    BlogRoute,
    #[at("/blog/*")]
    Blog,
}

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum BlogRoute {
    #[at("/blog/")]
    BlogIndexView,
    #[at("/blog/post/:slug")]
    PostView { slug: String },
    #[at("/blog/new")]
    NewPostView,
    #[at("/blog/drafts")]
    DraftsView,
    #[at("/blog/post/:slug/edit")]
    EditPostView { slug: String },
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <Redirect<BlogRoute> to={BlogRoute::BlogIndexView} /> },
        MainRoute::BlogRoute | MainRoute::Blog => html! { <Switch<BlogRoute> render={switch_blog} /> },
    }
}

pub fn switch_blog(route: BlogRoute) -> Html {
    match route {
        BlogRoute::BlogIndexView => html! { <BlogIndex/> },
        BlogRoute::PostView { slug } => html! { <BlogPost post_slug={slug} /> },
        BlogRoute::NewPostView => html! { <p>{ "New Post" }</p> },
        BlogRoute::DraftsView => html! { <p>{ "Drafts" }</p> },
        BlogRoute::EditPostView { slug } => html! { <p>{ format!("Edit Post: {}", slug) }</p> },
    }
}

