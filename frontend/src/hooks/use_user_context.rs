use std::ops::Deref;

use crate::{models::User, shared::utils::{remove_access_token, set_access_token}};
use yew::{hook, use_context, UseStateHandle};

#[derive(Debug, Clone, PartialEq)]
pub struct UseUserContextHandle{
    inner: UseStateHandle<Option<User>>,
}

impl UseUserContextHandle{
    pub fn set_user(&self, user: Option<User>, token: Option<String>){
        self.inner.set(user);
        if let Some(token) = token {
            set_access_token(token);
        }
    }

    pub fn remove_user(&self) {
        self.inner.set(None);
        remove_access_token();
    }
}

impl Deref for UseUserContextHandle{
    type Target = Option<User>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<Option<User>>>().unwrap();
    UseUserContextHandle { inner }
}
