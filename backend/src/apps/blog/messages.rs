use crate::apps::blog::insertables::{NewTag, PostUpdate, TagAssignment, TagUpdate};
use slug::slugify;
use actix::{Handler, Message};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};
use serde::Deserialize;
use crate::{apps::blog::insertables::NewPost, db::{models::{Post, Tag}, DbActor}, shared::statics::LEXICON};

// ------------------------- //
//          Posts            //
// ------------------------- //

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

        let changes = PostUpdate {
            title: msg.title,
            body: msg.body,
            is_published: msg.is_published,
            updated_at: chrono::Utc::now().naive_utc()
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

// ------------------------- //
//          Tags             //
// ------------------------- //

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Tag>")]
pub struct CreateTag{
    pub name: String,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>
}

impl Handler<CreateTag> for DbActor{
    type Result = QueryResult<Tag>;
    fn handle(&mut self, msg: CreateTag, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let new_tag = NewTag {
            slug: slugify(&msg.name),
            name: msg.name,
            background_color: msg.background_color,
            foreground_color: msg.foreground_color
        };
        diesel::insert_into(tags)
            .values(new_tag)
            .get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Vec<Tag>>")]
pub struct GetTags;

impl Handler<GetTags> for DbActor {
    type Result = QueryResult<Vec<Tag>>;
    fn handle(&mut self, _: GetTags, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        tags.get_results(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Tag>")]
pub struct GetTag{
    pub slug: String
}

impl Handler<GetTag> for DbActor {
    type Result = QueryResult<Tag>;
    fn handle(&mut self, msg: GetTag, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        tags.find(msg.slug).get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Tag>")]
pub struct UpdateTag {
    pub slug: String,
    pub name: Option<String>,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>
}

impl Handler<UpdateTag> for DbActor {
    type Result = QueryResult<Tag>;
    fn handle(&mut self, msg: UpdateTag, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);

        let changes = TagUpdate {
            name: msg.name,
            background_color: msg.background_color,
            foreground_color: msg.foreground_color
        };

        diesel::update(tags.find(msg.slug.clone()))
            .set(changes)
            .get_result(&mut conn)
    }
}


#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Tag>")]
pub struct DeleteTag{
    pub slug: String
}

impl Handler<DeleteTag> for DbActor {
    type Result = QueryResult<Tag>;
    fn handle(&mut self, msg: DeleteTag, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        diesel::delete(tags.find(msg.slug))
            .get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Vec<Tag>>")]
pub struct GetPostTags{
    pub slug: String
}

impl Handler<GetPostTags> for DbActor {
    type Result = QueryResult<Vec<Tag>>;
    fn handle(&mut self, msg: GetPostTags, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags::dsl::*;
        use crate::db::schema::tags_to_posts::dsl::*;
        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        tags_to_posts
            .filter(post.eq(msg.slug))
            .inner_join(tags)
            .select(tags::all_columns())
            .load::<Tag>(&mut conn)
    }
}


#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct AssignTagToPost{
    pub post_slug: String,
    pub tag_slug: String,
}

impl Handler<AssignTagToPost> for DbActor {
    type Result = QueryResult<Post>;
    fn handle(&mut self, msg: AssignTagToPost, _ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags_to_posts::dsl::*;        
        use crate::db::schema::posts::dsl::*;        

        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);
        let junction = TagAssignment {
            post: msg.post_slug.clone(),
            tag: msg.tag_slug.clone()
        };
        diesel::insert_into(tags_to_posts)
            .values(junction)
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Error assigning tag {} to post {}", msg.tag_slug, msg.post_slug));
        posts.find(msg.post_slug).get_result(&mut conn)
    }
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct RemoveTagFromPost{
    pub post_slug: String,
    pub tag_slug: String,
}

impl Handler<RemoveTagFromPost> for DbActor {
    type Result = QueryResult<Post>;
    fn handle(&mut self, msg: RemoveTagFromPost, _ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::tags_to_posts::dsl::*;        
        use crate::db::schema::posts::dsl::*;        

        let mut conn = self.pool.get().expect(LEXICON["db_pool_error"]);

        diesel::delete(tags_to_posts.filter(post.eq(&msg.post_slug)).filter(tag.eq(&msg.tag_slug)))
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Error removing tag {} from post {}", msg.tag_slug, msg.post_slug));
        posts.find(msg.post_slug).get_result(&mut conn)
    }
}
