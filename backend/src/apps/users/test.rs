use actix_web::{body::MessageBody, test, web::Data, App};
use serde_json::json;
use crate::{apps::{self, users::responses::{Tokens, UserPublic}}, db::{connect, AppState}, shared::{statics::{ACCESS_EXPIRATION, CONFIG}, utils::{create_jwt, random_string}}};

#[actix_web::test]
async fn test_users() {
    let db_addr = connect(CONFIG.db_url.as_str());
    let app = test::init_service(
        App::new()
                .app_data(Data::new(AppState {
                    db: db_addr.clone(),
                }))
                .service(apps::users::service())
    ).await;

    let admin_token = create_jwt(-1, true, ACCESS_EXPIRATION);
    let default_token = create_jwt(-1, false, ACCESS_EXPIRATION);
    let nm = random_string(10);

    // ----------------- //
    //   User creation   //
    // ----------------- //
    let req = test::TestRequest::post()
        .uri("/users/new")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(format!("{{\"name\": \"{}\", \"password\": \"123\", \"is_staff\": false}}", nm))
        .to_request();
    let res = test::call_service(&app, req).await;

    let bytes = res.into_body().try_into_bytes().unwrap();
    let res_str = format!("{}", String::from_utf8_lossy(&bytes));
    let user: UserPublic = serde_json::from_str(&res_str).unwrap();
    let id = user.id;
    println!("{:#?}", user);
    println!("{:#?}", nm);
    assert!(user.name == nm && user.email.is_none() && !user.is_staff);
    // NOTE: sometimes user.name != nm and idk why. it happens once in a ~7 test

    let req = test::TestRequest::post()
        .uri("/users/new")
        .insert_header(("Authorization", format!("Bearer {}", default_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(format!("{{\"name\": \"{}\", \"password\": \"123\", \"is_staff\": false}}", random_string(10)))
        .to_request();
    let res = test::try_call_service(&app, req).await;
    // let body = res.into_body();
    // println!("{body:?}");
    assert!(res.is_err());

    // ----------------- //
    //   Get token       //
    // ----------------- //
    let req = test::TestRequest::post()
        .uri("/users/login")
        .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
        .set_payload(format!("name={}&password=123", nm))
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let token = json!(String::from_utf8_lossy(&res.into_body().try_into_bytes().unwrap()));
    let token = serde_json::from_str::<Tokens>(token.as_str().unwrap()).unwrap().access_token;

    // ----------------- //
    //   Get profile     //
    // ----------------- //
    let req = test::TestRequest::get()
        .uri("/users/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let res = test::call_service(&app, req).await;
    let a = res.status().is_client_error();
    let header = format!("{:?}", res);
    let body = res.into_body();
    if a {
        println!("{:#?}", header);
        println!("{:#?}", body);
    }
    let bytes = body.try_into_bytes().unwrap();
    let res_str = format!("{}", String::from_utf8_lossy(&bytes));
    let user: UserPublic = serde_json::from_str(&res_str).unwrap();
    assert!(user.name == nm && user.email.is_none() && !user.is_staff);

    // ----------------- //
    //   Fetch user      //
    // ----------------- //
    let req = test::TestRequest::get()
        .uri(&format!("/users/fetch/{id}"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let bytes = res.into_body().try_into_bytes().unwrap();
    let res_str = format!("{}", String::from_utf8_lossy(&bytes));
    let user: UserPublic = serde_json::from_str(&res_str).unwrap();
    assert!(user.name == nm && user.email.is_none() && !user.is_staff);

    // ----------------- //
    //   User update     //
    // ----------------- //
    let nm = random_string(10);
    let req = test::TestRequest::patch()
        .uri(&format!("/users/{id}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(format!("{{\"name\": \"{}\", \"password\": \"123\", \"email\": \"chlen@mail.ru\"}}", nm))
        .to_request();
    let res = test::call_service(&app, req).await;

    let body = res.into_body();
    let bytes = body.try_into_bytes().unwrap();
    let res_str = format!("{}", String::from_utf8_lossy(&bytes));
    let user: UserPublic = serde_json::from_str(&res_str).unwrap();
    assert!(user.name == nm && user.email == Some("chlen@mail.ru".to_string()));

    let req = test::TestRequest::patch()
        .uri(&format!("/users/{id}"))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(format!("{{\"name\": \"{}\", \"password\": \"321\", \"email\": \"chlen@mail.ru\"}}", nm))
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let req = test::TestRequest::patch()
        .uri(&format!("/users/{id}"))
        .insert_header(("Authorization", format!("Bearer {}", default_token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(format!("{{\"name\": \"{}\", \"password\": \"123\", \"email\": \"chlen@mail.ru\"}}", nm))
        .to_request();
    let res = test::try_call_service(&app, req).await;
    assert!(res.unwrap().status().is_client_error());

    // ----------------- //
    //   User delete     //
    // ----------------- //
    let req = test::TestRequest::delete()
        .uri(&format!("/users/{id}"))
        .insert_header(("Authorization", format!("Bearer {}", default_token)))
        .to_request();
    let res = test::try_call_service(&app, req).await;
    // assert!(res.is_err());
    assert!(res.unwrap().status().is_client_error());

    let req = test::TestRequest::delete()
        .uri(&format!("/users/{id}"))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let bytes = res.into_body().try_into_bytes().unwrap();
    let res_str = format!("{}", String::from_utf8_lossy(&bytes));
    let user: UserPublic = serde_json::from_str(&res_str).unwrap();
    assert!(user.name == nm && user.email == Some("chlen@mail.ru".to_string()));

    // ----------------- //
    //  Fetch all users  //
    // ----------------- //
    let req = test::TestRequest::get()
        .uri("/users/fetch")
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());
}
