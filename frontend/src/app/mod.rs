pub mod blog;

use crate::components::user_context_provider::UserContextProvider;
use yew::{function_component, Html, html};
use yew_router::prelude::*;
use crate::{app::blog::{editor::Editor, index::BlogIndex, post::BlogPost}, components::navbar::Navbar};


#[derive(Routable, Clone, Debug, PartialEq)]
pub enum BlogRoute {
    #[at("/blog/")]
    Index,
    #[at("/blog/drafts")]
    Drafts,
    #[at("/blog/new")]
    NewPost,
    #[at("/blog/post/:slug")]
    Post { slug: String },
    #[at("/blog/post/:slug/edit")]
    EditPost { slug: String },
}

pub fn switch_blog(route: BlogRoute) -> Html {
    let main = match route {
        BlogRoute::Index => html! { <BlogIndex/> },
        BlogRoute::Drafts => html! { <p>{ "Drafts" }</p> },
        BlogRoute::NewPost => html! { <p>{ "New Post" }</p> },
        BlogRoute::Post { slug } => html! { <BlogPost slug={slug} /> },
        BlogRoute::EditPost { slug } => html! { <Editor slug={slug}  /> },
    };
    html! {
        <>
            <Navbar />
            {main}
        </>
    }
}

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/blog")]
    BlogRoute,
    #[at("/blog/*")]
    Blog,
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <Redirect<BlogRoute> to={BlogRoute::Index} /> }, // NOTE: TEMPORARY
        MainRoute::BlogRoute | MainRoute::Blog => html! { <Switch<BlogRoute> render={switch_blog} /> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Switch<MainRoute> render={switch_main} />
            </UserContextProvider>
        </BrowserRouter>
    }
}
