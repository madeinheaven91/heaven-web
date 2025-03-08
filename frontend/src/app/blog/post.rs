use crate::components::tag_list::TagList;
use crate::components::navbar::Navbar;
use crate::models::{User, Post};
use crate::shared::statics::CONFIG;
use crate::shared::utils::{get_post, get_profile};
use gloo::utils::window;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps{
    pub slug: String
}

#[function_component]
pub fn BlogPost(BlogPostProps { slug }: &BlogPostProps) -> Html {
    let (post, user) = (
        use_state(Post::default), 
        use_state(|| None::<User>)
    );

    let slug = slug.clone();
    {
        let user_clone = user.clone();
        let post_clone = post.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data = get_post(slug).await.unwrap_or_default();
                post_clone.set(data);

                let data = get_profile().await;
                match data {
                    Ok(resp) => {
                        let data: User = resp;
                        user_clone.set(Some(data));
                    }
                    _ => user_clone.set(None)
                }
            });
            || ()
        });
    }
    html!{
        <main class="container mt-3">
            <h1>{ &post.title }</h1>
            <hr />
            <h4 class="mt-3"><i>{ &post.author.name }</i></h4>
            <TagList tags={post.tags.clone()} />
            <hr />
            { view_body(&post.body) }
            <button onclick={Callback::from(move |_| { window().history().unwrap().back().unwrap() })} class="mt-3 btn btn-primary">{"Назад"}</button>
        </main>
    }
}


/// Dangerously set innerHTML for article body
fn view_body(body: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(body);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(html_text.as_str());
    let node = Node::from(div);
    VNode::VRef(node)
}
