use crate::components::tag_list::TagList;
// use gloo_net::http::Request;
use yew::{function_component, html, Html, Properties};

use crate::models::{Post, User};

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    pub posts: Vec<Post>,
    pub user: Option<User>
}

#[function_component(PostList)]
pub fn post_list(PostListProps { posts, user }: &PostListProps) -> Html {
    if posts.is_empty() {
        html! {
            <h1 class="text-secondary">{ "Нет постов..." }</h1>
        }
    } else {
        posts
            .iter()
            .map(|post| {
                // let slug = post.slug.clone();
                let tags_list = match &post.tags.is_empty() {
                    true => html! {},
                    false => html! {
                        <TagList tags={post.tags.clone()} />
                    },
                    
                };
                let buttons = match user {
                    Some(inner) => html! {
                        if post.author.id == inner.id || inner.is_staff {
                            <div class="btn-group gap-2">
                                if post.author.id == inner.id {
                                    <a href={ "./post/".to_string() + &post.slug + "/edit" } class="btn btn-primary">{ "Редактировать" }</a>
                                }
                                if post.author.id == inner.id || inner.is_staff {
                                    <a class="btn btn-danger" 
//                                        onclick={
//                                    let slug = slug.clone();
//                                    Callback::from(move |_| {
//                                            spawn_local(async move {
//                                                delete_post(&slug.clone()).await;
//                                            })})}
                                    >{"Удалить"}</a>
                                }
                            </div>
                        }
                    },
                    None => html! {}
                };
                html! {
                    <div class="border p-2 d-flex flex-row justify-content-between align-items-center">
                        <div class="d-flex flex-column px-3">
                        <a href={ "./post/".to_string() + &post.slug } class="text-decoration-none text-primary fs-3">
                            { &post.title }
                        </a>
                        {tags_list}
                        <h4 class="text-secondary">{{ &post.author.name }}</h4>
                        </div>
                        {buttons}
                    </div>
                }
            })
            .collect()
    }
}

// async fn delete_post(slug: &str) {
//     Request::delete(format!("http://localhost:8000/api/v1/blog/posts/{slug}").as_str())
//         .credentials(web_sys::RequestCredentials::Include)
//         .send();
// }
