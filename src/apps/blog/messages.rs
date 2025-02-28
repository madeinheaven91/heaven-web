use crate::apps::blog::insertables::PostUpdateForm;
use actix::{Handler, Message};
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;
use crate::{apps::blog::insertables::NewPost, db::{models::Post, pg::DbActor}, shared::{slugify, LEXICON}};

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub author_id: i32
}

impl Handler<CreatePost> for DbActor {
    type Result = QueryResult<Post>;
    fn handle(&mut self, msg: CreatePost, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::posts::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let new_post = NewPost {
            slug: slugify(&msg.title),
            title: msg.title,
            body: msg.body,
            author_id: msg.author_id
        };
        diesel::insert_into(posts)
            .values(new_post)
            .get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct GetPosts;

impl Handler<GetPosts> for DbActor {
    type Result = QueryResult<Vec<Post>>;
    fn handle(&mut self, _: GetPosts, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::posts::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        posts.get_results(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct GetPost{
    pub slug: String
}

impl Handler<GetPost> for DbActor {
    type Result = QueryResult<Post>;
    fn handle(&mut self, msg: GetPost, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::posts::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        posts.find(msg.slug).get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct UpdatePost {
    pub slug: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_published: Option<bool>,
}

impl Handler<UpdatePost> for DbActor {
    type Result = QueryResult<Post>;
    fn handle(&mut self, msg: UpdatePost, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::posts::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);

        let changes = PostUpdateForm {
            title: msg.title,
            body: msg.body,
            is_published: msg.is_published,
        };
        diesel::update(posts.find(msg.slug))
            .set(changes)
            .get_result(&mut conn)
    }
}


#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct DeletePost{
    pub slug: String
}

impl Handler<DeletePost> for DbActor {
    type Result = QueryResult<Post>;
    fn handle(&mut self, msg: DeletePost, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::posts::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        diesel::delete(posts.find(msg.slug)).get_result(&mut conn)
    }
}
