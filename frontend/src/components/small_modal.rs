use gloo::utils::window;
use gloo_net::http::Request;
use web_sys::{wasm_bindgen::JsCast, HtmlFormElement, RequestCredentials};
use yew::{function_component, html, use_state, Callback, Html, InputEvent, Properties, SubmitEvent};


#[derive(Properties, PartialEq)]
pub struct SmallModalProps {
    pub title: String,
    pub child: Html,
}

#[function_component(SmallModal)]
pub fn small_modal(SmallModalProps { child, title }: &SmallModalProps) -> Html {
    html!{ 
        <div class="modal fade bd-example-modal-sm" tabindex="-1" role="dialog" aria-labelledby="mySmallModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-sm">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{ title }</h5>
                        <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                            <span aria-hidden="true">{"×"}</span>
                        </button>
                    </div>
                    <div class="modal-body">
                        {child.clone()}
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn login_form() -> Html {
    let onsubmit = Callback::from(|event: web_sys::SubmitEvent| {
        event.prevent_default();
        let form = event.target().unwrap().dyn_into::<HtmlFormElement>().unwrap();
    });
    html! {
    <form id="loginForm" action="./" onsubmit={onsubmit}>
        <div class="form-group">
            <label for="usernameInput">{"Имя"}</label>
            <input type="text" class="form-control" id="usernameInput" name="name" placeholder="дмитрий прудников"/>
//            <small id="emailHelp" class="form-text text-muted">{"We'll never share your email with anyone else."}</small>
        </div>
        <div class="form-group mt-3">
            <label for="passwordInput">{"Пароль"}</label>
            <input type="text" class="form-control" id="passwordInput" name="password" placeholder="12345"/>
        </div>
        <div class="modal-footer mt-3 d-flex justify-content-around">
            <input type="submit" class="btn btn-primary" value={"Войти"}/>
            <button type="button" class="btn btn-secondary" data-dismiss="modal">{"Закрыть"}</button>
        </div>
    </form>
    }
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username = use_state(String::default);
    let password = use_state(String::default);
    let err_msg = use_state(String::default);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let err_msg = err_msg.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Prevent page reload
            
            let username = username.clone();
            let password = password.clone();
            let err_msg = err_msg.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let body = format!("name={}&password={}", *username, *password);

                let response = Request::post("http://localhost:8000/api/v1/users/login")
                    .credentials(RequestCredentials::Include)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        err_msg.set(String::new());
                        window().location().reload().unwrap()
                    }
                    _ => {
                        err_msg.set("Не удалось войти".to_string());
                    }
                }
            });
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
//            <input type="text" class="form-control" id="usernameInput" name="name" placeholder="дмитрий прудников"/>
//            <small id="emailHelp" class="form-text text-muted">{"We'll never share your email with anyone else."}</small>
            <input type="text" class="form-control" id="usernameInput" placeholder="Дмитрий Прудников" oninput={Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                username.set(input.value());
            })}/>
        </div>
        <div class="form-group mt-3">
            <label for="passwordInput">{"Пароль"}</label>
//            <input type="text" class="form-control" id="passwordInput" name="password" placeholder="12345"/>
            <input type="password" class="form-control" id="passwordInput" placeholder="12345" oninput={Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                password.set(input.value());
            })}/>
        </div>
        <div class="modal-footer mt-3 d-flex justify-content-around">
            <input type="submit" class="btn btn-primary" value={"Войти"}/>
//            <button type="submit" ckass=>{"Login"}</button>
            <button type="button" class="btn btn-secondary" data-dismiss="modal">{"Закрыть"}</button>
        </div>
        </form>
    </>
    }
}
