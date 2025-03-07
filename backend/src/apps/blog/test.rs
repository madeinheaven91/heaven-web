use actix_web::{body::MessageBody, test, web::Data, App};
use serde_json::json;
use crate::{apps::{self, users::responses::UserPublic}, db::{connect, models::{Post, Tag}, AppState}, shared::{statics::{ACCESS_EXPIRATION, CONFIG}, utils::{create_jwt, random_string}}};

#[actix_web::test]
async fn test_blog() {
    let db_addr = connect(CONFIG.db_url.as_str());
    let app = test::init_service(
        App::new()
                .app_data(Data::new(AppState { db: db_addr.clone(),
                }))
                .service(apps::users::service())
                .service(apps::blog::service())
    ).await;

    let admin_token = create_jwt(-1, true, ACCESS_EXPIRATION);
    let name = random_string(10);
    let post_title = random_string(10);

    // ----------------- //
    // # CREATE USER     //
    // ----------------- //
    let req = test::TestRequest::post()
        .uri("/users/new")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(format!("{{\"name\": \"{}\", \"password\": \"123\", \"is_staff\": false}}", name))
        .to_request();
    let res = test::call_service(&app, req).await;
    let bytes = res.into_body().try_into_bytes().unwrap();
    let res_str = format!("{}", String::from_utf8_lossy(&bytes));
    let user: UserPublic = serde_json::from_str(&res_str).unwrap();
    let id = user.id;
    let token = create_jwt(user.id, false, ACCESS_EXPIRATION);

    // --------------- //
    // # CREATE POST   //
    // --------------- //
    let req = test::TestRequest::post()
        .uri("/blog/posts/new")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(json!({
            "title": post_title,
            "body": "body",
        }).to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let post = res.into_body().try_into_bytes().unwrap();
    let post = serde_json::from_str::<Post>(&String::from_utf8_lossy(&post)).unwrap();
    let post_slug = post.slug;
    assert_eq!(post_title, post.title);

    // --------------- //
    // # READ POST     //
    // --------------- //
    let req = test::TestRequest::get()
        .uri(&format!("/blog/posts/fetch/{post_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    let tag = res.into_body().try_into_bytes().unwrap();
    let post = serde_json::from_str::<Post>(&String::from_utf8_lossy(&tag)).unwrap();
    assert_eq!(post_title, post.title);

    // --------------- //
    // # UPDATE POST   //
    // --------------- //
    let req = test::TestRequest::patch()
        .uri(&format!("/blog/posts/{post_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(json!({
            "title": "New title",
        }).to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let npost = res.into_body().try_into_bytes().unwrap();
    let npost = serde_json::from_str::<Post>(&String::from_utf8_lossy(&npost)).unwrap();
    let npost_title = npost.title;
    assert!(npost_title == "New title");

    // --------------- //
    // # DELETE POST   //
    // --------------- //
    let req = test::TestRequest::delete()
        .uri(&format!("/blog/posts/{post_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    let npost = res.into_body().try_into_bytes().unwrap();
    let npost = serde_json::from_str::<Post>(&String::from_utf8_lossy(&npost)).unwrap();
    let npost_title = npost.title;
    assert!(npost_title == "New title");

    // --------------- //
    // # CREATE POST   //
    // --------------- //
    let snd_post_title = random_string(10);
    let req = test::TestRequest::post()
        .uri("/blog/posts/new")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(json!({
            "title": snd_post_title,
            "body": "body",
        }).to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let post = res.into_body().try_into_bytes().unwrap();
    let post = serde_json::from_str::<Post>(&String::from_utf8_lossy(&post)).unwrap();
    let snd_post_slug = post.slug;
    assert_eq!(snd_post_title, post.title);

    // --------------- //
    // # CREATE TAG    //
    // --------------- //
    let tag_name = random_string(10);
    let req = test::TestRequest::post()
        .uri("/blog/tags/new")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(json!({
            "name": tag_name,
        }).to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let tag = res.into_body().try_into_bytes().unwrap();
    let tag = serde_json::from_str::<Tag>(&String::from_utf8_lossy(&tag)).unwrap();
    let tag_slug = tag.slug;
    assert_eq!(tag_name, tag.name);
    assert_eq!("000000", tag.background_color);
    assert_eq!("ffffff", tag.foreground_color);

    // --------------- //
    // # READ TAG      //
    // --------------- //
    let req = test::TestRequest::get()
        .uri(&format!("/blog/tags/fetch/{tag_slug}"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let tag = res.into_body().try_into_bytes().unwrap();
    let tag = serde_json::from_str::<Tag>(&String::from_utf8_lossy(&tag)).unwrap();
    assert_eq!(tag_name.to_lowercase(), tag.slug);

    // --------------- //
    // # UPDATE TAG    //
    // --------------- //
    let snd_tag_name = random_string(10);
    let req = test::TestRequest::patch()
        .uri(&format!("/blog/tags/{tag_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(json!({
            "name": snd_tag_name,
            "background_color": "aaaaaa",
        }).to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let tag = res.into_body().try_into_bytes().unwrap();
    let tag = serde_json::from_str::<Tag>(&String::from_utf8_lossy(&tag)).unwrap();
    assert_eq!(snd_tag_name, tag.name);
    assert_eq!("aaaaaa", tag.background_color);
    assert_eq!("ffffff", tag.foreground_color);

    // --------------- //
    // # ASSIGN TAG    //
    // --------------- //
    let req = test::TestRequest::post()
        .uri(&format!("/blog/posts/{snd_post_slug}/assign/{tag_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    // --------------- //
    // # WITHDRAW TAG  //
    // --------------- //
    let req = test::TestRequest::delete()
        .uri(&format!("/blog/posts/{snd_post_slug}/withdraw/{tag_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    // --------------- //
    // # DELETE TAG   //
    // --------------- //
    let req = test::TestRequest::delete()
        .uri(&format!("/blog/tags/{tag_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    // ----------------- //
    // # DELETE USER     //
    // ----------------- //
    let req = test::TestRequest::delete()
        .uri(&format!("/users/{id}"))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());


    // --------------- //
    // # DELETE POST   //
    // --------------- //
    let req = test::TestRequest::delete()
        .uri(&format!("/blog/posts/{snd_post_slug}"))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    let npost = res.into_body().try_into_bytes().unwrap();
    let npost = serde_json::from_str::<Post>(&String::from_utf8_lossy(&npost)).unwrap();
    let npost_author = npost.author_id;
    assert!(npost_author.is_none());
}
