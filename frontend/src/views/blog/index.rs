use crate::components::navbar::Navbar;
use crate::components::post_list::PostList;
use crate::components::tag_list::TagList;
use crate::models::{Post, Tag, User};
use crate::shared::statics::CONFIG;
use crate::shared::utils::logged_in;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(BlogIndex)]
pub fn blog_index() -> Html {
    let (posts, tags, user) = (
        use_state(Vec::new),
        use_state(Vec::new),
        use_state(|| None::<User>),
    );

    {
        let user_clone = user.clone();
        let (posts_clone, tags_clone) = (posts.clone(), tags.clone());
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data: Vec<Post> = Request::get(
                    format!("http://localhost:{}/api/v1/blog/posts", CONFIG.backend_port).as_str(),
                )
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                posts_clone.set(data);
                let data: Vec<Tag> = Request::get(
                    format!("http://localhost:{}/api/v1/blog/tags", CONFIG.backend_port).as_str(),
                )
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                tags_clone.set(data);

                let data = Request::get(
                    format!(
                        "http://localhost:{}/api/v1/users/profile",
                        CONFIG.backend_port
                    )
                    .as_str(),
                )
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await;
                match data {
                    Ok(resp) if resp.ok() => {
                        let data: User = resp.json().await.unwrap();
                        user_clone.set(Some(data));
                    }
                    _ => user_clone.set(None),
                }
            });
            || ()
        });
    }
    let logged_in = user.is_some();
    html! {
        <>
            <Navbar user={(*user).clone()}/>
            <main class="container mt-3">
                    <h1>{"Посты"}</h1>
                    <hr/>
                    <h3>{"Фильтр по тегам"}</h3>
                    <div class="gap-1 d-flex">
                        <TagList tags={(*tags).clone()} />
                    </div>
                    <hr/>
                    <PostList posts={(*posts).clone()} user={(*user).clone()}/>
            </main>
        </>
    }
}
