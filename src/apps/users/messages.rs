use crate::apps::users::insertables::UserUpdateForm;
use crate::{
    apps::users::insertables::NewUser,
    db::{models::User, pg::DbActor},
    shared::LEXICON,
};
use actix::{Handler, Message};
use diesel::{query_dsl::methods::FindDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUsers;

impl Handler<FetchUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _: FetchUsers, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        users.get_results(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct FetchUser {
    pub id: i32,
}

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        users.find(msg.id).get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub is_staff: bool,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let new_user = NewUser {
            name: msg.name,
            password: msg.password,
            email: msg.email,
            is_staff: msg.is_staff,
        };
        diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct UpdateUser {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_staff: Option<bool>,
}


impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);

        let changes = UserUpdateForm {
            name: msg.name,
            password: msg.password,
            email: msg.email,
            is_staff: msg.is_staff,
        };
        diesel::update(users.find(msg.id))
            .set(changes)
            .get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct DeleteUser {
    pub id: i32,
}

impl Handler<DeleteUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        diesel::delete(users.find(msg.id)).get_result(&mut conn)
    }
}
