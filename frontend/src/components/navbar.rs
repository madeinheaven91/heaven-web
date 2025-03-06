use crate::{components::small_modal::{LoginForm, SmallModal}, shared::utils::logout};
use gloo::utils::window;
use yew::{function_component, html, Callback, Html, Properties};

use crate::models::User;

#[derive(Properties, PartialEq)]
pub struct NavbarProps{
    pub user: Option<User>
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
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
            if props.user.is_some() {
                <li class="nav-item">
                    <a class="nav-link" href="/blog/new">{ "Добавить пост" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/blog/drafts">{ "Черновики" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={Callback::from(|_| {
                            logout();
//    FIXME: if the window is reloaded, the user is not logged out
//                            window().location().reload().unwrap()
                        }
                    )}>
                        {"Выход"}
                    </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link text-warning">{ props.user.clone().unwrap().name }</a>
                </li>
            } else {
                <li class="nav-item">
                    <a type="button" class="nav-link" data-toggle="modal" data-target=".bd-example-modal-sm">{"Вход"}</a>
                </li>
            }
            </ul>
          </div>
        </div>
      </nav>
    <SmallModal child={
        html! {
            <LoginForm />
            }
        }
        title="Авторизация"
    />
    </header>
    }
}
