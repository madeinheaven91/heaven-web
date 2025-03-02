use crate::apps::users::insertables::UserUpdate;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::shared::utils::hash_password;
use crate::shared::utils::verify_password;
use crate::{
    apps::users::insertables::NewUser,
    db::{models::User, DbActor},
    shared::statics::LEXICON,
};
use actix::{Handler, Message};
use diesel::{query_dsl::methods::{FindDsl, FilterDsl}, QueryResult};
use serde::Deserialize;

use super::forms::{LoginForm, CreateUser};

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUsers;

impl Handler<FetchUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _: FetchUsers, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let a = users.get_results(&mut conn);
        a
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


impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let new_user = NewUser {
            name: msg.name,
            password: hash_password(&msg.password).unwrap(),
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

        let changes = UserUpdate {
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

impl Handler<LoginForm> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: LoginForm, _ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let user = users
            .filter(name.eq(msg.name)).first::<User>(&mut conn);
        match user {
            Ok(user) => {
                if verify_password(&msg.password, &user.password) {
                    Ok(user)
                }else{
                    Err(diesel::result::Error::NotFound)
                }
            }
            Err(err) => Err(err)
        }
    }
}
