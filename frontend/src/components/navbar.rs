use crate::{components::small_modal::{LoginForm, SmallModal}, hooks::use_user_context::use_user_context, shared::utils::logout};
use gloo::console;
use yew::{function_component, html, platform::spawn_local, Callback, Hook, Html};
use yew_hooks::use_async;

use crate::models::User;

#[function_component]
pub fn Navbar() -> Html {
    let user_ctx = use_user_context();
    html! {
    <header>
      <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
          <a class="navbar-brand" href="/blog/">{"JazzCoding"}</a>
          <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
          </button>
          <div class="collapse navbar-collapse" id="navbarSupportedContent">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            if user_ctx.is_some() {
                {logged_in_view((*user_ctx).clone().unwrap())}
            } else {
                {logged_out_view()}
            }
            </ul>
          </div>
        </div>
      </nav>
    </header>
    }
}

fn logged_in_view(user: User) -> Html{
    let logout = {
        Callback::from(|_| {
            spawn_local(async {
                console::log!("logging out");
                let res = logout().await;
                if let Err(str) = res { console::log!(str) };
            });
            //    FIXME: if the window is reloaded, the user is not logged out
            //   window().location().reload().unwrap()
        })
    };
    html! {
    <>
        <li class="nav-item">
            <a class="nav-link" href="/blog/new">{ "Добавить пост" }</a>
        </li>
        <li class="nav-item">
            <a class="nav-link" href="/blog/drafts">{ "Черновики" }</a>
        </li>
        <li class="nav-item">
            <a class="nav-link" onclick={logout}>
                {"Выход"}
            </a>
        </li>
        <li class="nav-item">
            <a class="nav-link text-warning">{ user.name }</a>
        </li>
    </>
    }

}

fn logged_out_view() -> Html {
    html!{
    <>
        <li class="nav-item">
            <a type="button" class="nav-link" data-toggle="modal" data-target=".login_modal">{"Вход"}</a>
        </li>
        <SmallModal title="Авторизация" id="login_modal">
            <LoginForm />
        </SmallModal>
    </>
    }

}
