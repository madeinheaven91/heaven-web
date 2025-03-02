use chrono::NaiveDateTime;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let posts = use_state(Vec::new);
    let posts_clone = posts.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let data: Vec<Post> = Request::get("http://localhost:8000/api/v1/blog/posts")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            posts_clone.set(data);
        });
        || ()
    });
    html! {
        <>
            <Navbar />
            <main class="container mt-3">
                <h1>{"Посты"}</h1>
                <hr/>
                <h3>{"Фильтр по тегам"}</h3>
                <div>
                    <p>{"теги"}</p>
                </div>
                <PostList posts={(*posts).clone()} />
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM" crossorigin="anonymous"></script>
            </main>
        </>
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author_id: i32,
    pub is_published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    posts: Vec<Post>,
}

#[function_component(PostList)]
fn post_list(PostListProps { posts }: &PostListProps) -> Html {
    posts
        .iter()
        .map(|post| {
            html! {
                <div class="border p-2 d-flex flex-row justify-content-between align-items-center">
                    <div class="d-flex flex-column px-3">
                    <a href="{% url 'blog:post' post.slug %}" class="text-decoration-none text-primary fs-3">
                        { &post.title }
                    </a>
                    <h4 class="text-secondary">{{ post.author_id }}</h4>
                    </div>

                    <div class="btn-group gap-2">
                        <a href="{% url 'blog:edit_post' post.pk %}" class="btn btn-primary">{ "Редактировать" }</a>
                        <form method="post" action="{% url 'blog:delete_post' post.pk %}">
                        <button class="btn btn-danger" type="submit">{"Удалить"}</button>
                        </form>
                    </div>
                </div>
            }
        })
        .collect()
}

#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
    <header>
      <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
          <a class="navbar-brand" href="#">{"JazzCoding"}</a>
          <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
          </button>
          <div class="collapse navbar-collapse" id="navbarSupportedContent">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
              <li class="nav-item">
                <a class="nav-link" href="#">{ "Добавить пост" }</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" href="#">{ "Черновики" }</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" href="#">{ "Выход" }</a>
              </li>
              <li class="nav-item">
                <a class="nav-link text-warning">{ "юзернейм" }</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" href="#">{ "Вход" }</a>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </header>
    }
}
