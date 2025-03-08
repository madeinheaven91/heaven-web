use crate::components::navbar::Navbar;
use crate::components::post_list::PostList;
use crate::components::tag_list::TagList;
use crate::models::{Post, Tag, User};
use crate::shared::statics::CONFIG;
use crate::shared::utils::{get_posts, get_profile, get_tags};
use yew::prelude::*;

#[function_component]
pub fn BlogIndex() -> Html {
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
                let data = get_posts().await.unwrap_or(Vec::new());
                posts_clone.set(data);
                let data = get_tags().await.unwrap_or(Vec::new());
                tags_clone.set(data);

                let data = get_profile().await;
                match data {
                    Ok(resp) => {
                        let data = resp;
                        user_clone.set(Some(data));
                    }
                    _ => user_clone.set(None),
                }
            });
            || ()
        });
    }
    html! {
        <main class="container mt-3">
                <h1>{"Посты"}</h1>
                <hr/>
                <h3>{"Фильтр по тегам"}</h3>
                <div class="gap-1 d-flex">
                    <TagList tags={(*tags).clone()} />
                </div>
                <hr/>
                <PostList posts={(*posts).clone()}/>
        </main>
    }
}
