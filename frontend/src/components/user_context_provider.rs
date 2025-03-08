use yew_hooks::use_async;
use yew::use_effect_with;
use yew::UseStateHandle;
use yew::ContextProvider;
use yew::{function_component, html, use_state, Children, Html, Properties};
use yew_hooks::use_mount;

use crate::models::User;
use crate::shared::utils::get_profile;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children
}

#[function_component]
pub fn UserContextProvider(props: &Props) -> Html {
    let ctx = use_state(|| None::<User>);
    let user = use_async(async move { get_profile().await.map_err(|_| None::<User>) });

    {
        let user = user.clone();
        use_mount(move || {
            user.run();
        })
    }

    {
        let ctx = ctx.clone();
        use_effect_with(user, move |user| {
            if let Some(info) = &user.data {
                ctx.set(Some(info.clone()));
            }else {
                ctx.set(None)
            }
        });

    }

    html! {
        <ContextProvider<UseStateHandle<Option<User>>> context={ctx}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<Option<User>>>>
    }
}
