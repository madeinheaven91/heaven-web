use gloo::{console, utils::window};
use web_sys::{wasm_bindgen::JsCast, RequestCredentials};
use yew::{function_component, html, use_state, Callback, Children, Html, InputEvent, Properties, SubmitEvent};
use yew_hooks::use_async;

use crate::{hooks::use_user_context::use_user_context, models::User, shared::utils::{get_profile, login, Token}};


#[derive(Properties, PartialEq)]
pub struct SmallModalProps {
    pub title: String,
    pub id: String,
    pub children: Children
}

#[function_component]
pub fn SmallModal(SmallModalProps { children, title, id }: &SmallModalProps) -> Html {
    let class = format!("modal fade {id}");
    html!{ 
        <div class={class} tabindex="-1" role="dialog" aria-labelledby="mySmallModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-sm">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{ title }</h5>
                        <h2 type="button" class="close" data-dismiss="modal" aria-label="Close">
                            <span aria-hidden="true">{"×"}</span>
                        </h2>
                    </div>
                    <div class="modal-body">
                        {children}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct LoginInfo{
    username: String,
    password: String
}

#[function_component]
pub fn LoginForm() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let err_msg = use_state(String::new);

    // послать запрос серверу
    // если успешно, то сохранить токен и куки, перезагрузить страницу
    // если ошибка, то выдать сообщение об ошибке
    // let user_login = {
    //     let login_info = login_info.clone();
    //     use_state(async move || {
    //             let username = &login_info.username;
    //             let password = &login_info.password;
    //             let err_msg = err_msg.clone();
    //
    //             let response = login(username, password).await;
    //             match response {
    //                 Ok(token) => {
    //                     let user = get_profile().await.ok();
    //                     user_ctx.login(user, Some(token));
    //                     err_msg.set(String::new());
    //                     window().location().reload().unwrap()
    //                 }
    //                 Err(err) => {
    //                     let str = err.to_string();
    //                     console::log!(&str);
    //                     err_msg.set("Не удалось войти".to_string());
    //                 }
    //             }
    //     })
    // };
    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let username = &login_info.username;
            let password = &login_info.password;
            login(username, password).await
        })
    };
    //
    // let onsubmit = {
    //     let username = username.clone();
    //     let password = password.clone();
    //     let err_msg = err_msg.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default(); // Prevent page reload
    //         
    //         let username = username.clone();
    //         let password = password.clone();
    //         let err_msg = err_msg.clone();
    //
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let body = format!("name={}&password={}", *username, *password);
    //
    //             let response = Request::post("http://localhost:8000/api/v1/users/login")
    //                 .credentials(RequestCredentials::Include)
    //                 .header("Content-Type", "application/x-www-form-urlencoded")
    //                 .body(body)
    //                 .unwrap()
    //                 .send()
    //                 .await;
    //
    //             match response {
    //                 Ok(resp) if resp.ok() => {
    //                     let token: Token = resp.json().await.unwrap();
    //                     let token = token.access();
    //                     let response = Request::post("http://localhost:8000/api/v1/users/login")
    //                         .credentials(RequestCredentials::Include)
    //                         .header("Content-Type", "application/x-www-form-urlencoded")
    //                         .header("Authorization", token)
    //                         .body(body)
    //                         .unwrap()
    //                         .send()
    //                         .await;
    //                     let user: User = response.unwrap().json().await.unwrap();
    //
    //                     user_ctx.login(Some(user), Some(token.clone()));
    //                     // err_msg.set(String::new());
    //                     window().location().reload().unwrap()
    //                 }
    //                 _ => {
    //                     err_msg.set("Не удалось войти".to_string());
    //                 }
    //             }
    //         });
    //     })
    // };

    let username_oninput = {
        let login_info = login_info.clone();
        Callback::from( move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let mut new = (*login_info).clone();
            new.username = input.value();
            login_info.set(new);
    })};
    let password_oninput = Callback::from( move |e: InputEvent| {
        let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
        let mut new = (*login_info).clone();
        new.password = input.value();
        login_info.set(new);
    });
    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            user_login.run();
            // FIXME: if reloaded then it does not save access token
            // window().location().reload().unwrap()
        })
    };

    html! {
    <>
        if !err_msg.is_empty() {
            <p style="color: red;">{ (*err_msg).clone() }</p>
        }
        <form onsubmit={onsubmit}>
        <div class="form-group">
            <label for="usernameInput">{"Имя"}</label>
            <input type="text" class="form-control" id="usernameInput" placeholder="Дмитрий Прудников" oninput={username_oninput}/>
        </div>
        <div class="form-group mt-3">
            <label for="passwordInput">{"Пароль"}</label>
            <input type="password" class="form-control" id="passwordInput" placeholder="12345" oninput={password_oninput}/>
        </div>
        <div class="modal-footer mt-3 d-flex justify-content-around">
            <input type="submit" class="btn btn-primary" value={"Войти"}/>
            <button type="button" class="btn btn-secondary" data-dismiss="modal">{"Закрыть"}</button>
        </div>
        </form>
    </>
    }
}
