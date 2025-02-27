use crate::db::schema::users::dsl::users;
use crate::{
    apps::users::insertables::NewUser,
    db::{models::User, pg::DbActor},
};
use actix::{Handler, Message};
use diesel::{query_dsl::methods::FindDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUsers;

impl Handler<FetchUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: FetchUsers, _ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.0.get().expect("couldn't get db connection from pool");
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
        let mut conn = self.0.get().expect("couldn't get db connection from pool");
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

    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.0.get().expect("couldn't get db connection from pool");
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
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub is_staff: bool,
}

impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateUser, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.0.get().expect("couldn't get db connection from pool");

        let update = User {
            id: msg.id,
            name: msg.name,
            password: msg.password,
            email: msg.email,
            is_staff: msg.is_staff,
        };

        diesel::update(users.find(msg.id))
            .set(update)
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

    fn handle(&mut self, msg: DeleteUser, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.0.get().expect("couldn't get db connection from pool");
        diesel::delete(users.find(msg.id)).get_result(&mut conn)
    }
}
