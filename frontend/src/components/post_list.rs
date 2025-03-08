use crate::components::delete_button::DeleteButton;
use crate::{components::tag_list::TagList, hooks::use_user_context::use_user_context};
use yew::{function_component, html, Html, Properties};

use crate::models::Post;

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    pub posts: Vec<Post>,
}

#[function_component]
pub fn PostList(PostListProps { posts }: &PostListProps) -> Html {
    let user = use_user_context();
    let posts: Vec<Post> = posts.clone().into_iter().filter(|p| p.is_published).collect();
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
                let buttons = match (*user).clone() {
                    Some(inner) => html! {
                        if post.author.id == inner.id || inner.is_staff {
                            <div class="btn-group gap-2">
                                if post.author.id == inner.id {
                                    <a href={ "./post/".to_string() + &post.slug + "/edit" } class="btn btn-primary">{ "Редактировать" }</a>
                                }
                                if post.author.id == inner.id || inner.is_staff {
                                    <DeleteButton slug={post.slug.clone()} />
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
                        <div>
                           {tags_list}
                        </div>
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
