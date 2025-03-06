use crate::components::tag_list::TagList;
use crate::components::navbar::Navbar;
use crate::models::{User, Post};
use crate::shared::statics::CONFIG;
use crate::shared::utils::logged_in;
use gloo::utils::window;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_markdown::Markdown;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps{
    pub post_slug: String
}

#[function_component(BlogPost)]
pub fn blog_post(BlogPostProps { post_slug }: &BlogPostProps) -> Html {
    let logged_in = logged_in();
    let (post, user) = (
        use_state(Post::default), 
        use_state(|| None::<User>)
    );
    let slug = post_slug.clone();

    {
        let user_clone = user.clone();
        let post_clone = post.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data: Post = Request::get(format!("http://localhost:{}/api/v1/blog/posts/{}", CONFIG.backend_port, &slug).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await.unwrap();
                post_clone.set(data);

                if !logged_in {
                    user_clone.set(None);
                }else{
                    let data = Request::get(format!("http://localhost:{}/api/v1/users/profile", CONFIG.backend_port).as_str())
                        .credentials(web_sys::RequestCredentials::Include)
                        .send()
                        .await;
                match data {
                    Ok(resp) if resp.ok() => {
                        let data: User = resp.json().await.unwrap();
                        user_clone.set(Some(data));
                    }
                    _ => user_clone.set(None)
                    }
                }
            });
            || ()
        });
    }
    html!{
        <>
            <Navbar user={(*user).clone()}/>
            <main class="container mt-3">
                <h1>{ &post.title }</h1>
                <hr />
                <h4 class="mt-3"><i>{ &post.author.name }</i></h4>
                <TagList tags={post.tags.clone()} />
                <hr />
                <Markdown src={ post.body.clone() }/>
                <button onclick={Callback::from(move |_| { window().history().unwrap().back().unwrap() })} class="mt-3 btn btn-primary">{"Назад"}</button>
            </main>
        </>
    }
}

