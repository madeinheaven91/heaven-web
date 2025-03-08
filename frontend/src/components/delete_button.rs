use crate::components::small_modal::SmallModal;
use crate::{components::tag_list::TagList, hooks::use_user_context::use_user_context, services::requests::delete_post};
// use gloo_net::http::Request;
use yew::{function_component, html, Callback, Html, Properties};
use yew_hooks::use_async;

use crate::models::{Post, User};

#[derive(Properties, PartialEq)]
pub struct DeleteProps {
    pub slug: String
}

#[function_component]
pub fn DeleteButton(DeleteProps { slug }: &DeleteProps) -> Html {
    let delete = {
        let slug = slug.clone();
        use_async(async move {
            delete_post(&slug).await
        })
    };
    let onclick = {
        Callback::from(move |_| {
            delete.run()
        })
    };
    let target = format!(".{slug}");
    html! {
    <>
        <a type="button" class="btn btn-danger" data-toggle="modal" data-target={target}>{"Удалить"}</a>
        <SmallModal title="Вы уверены?" id={slug.clone()}>
            <div class="d-flex mt-3 justify-content-between">
                <a class="btn btn-danger" onclick={onclick}>{"Удалить"}</a>
                <button type="button" class="btn btn-secondary" data-dismiss="modal">{"Отмена"}</button>
            </div>
        </SmallModal>
    </>
    }
}
