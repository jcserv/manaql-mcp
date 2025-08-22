use actix_web::{test, web, App};
use rust_api_template::books::{self, CreateBookSchema, UpdateBookSchema};
use rust_api_template::books::BookModel;

mod common;

#[actix_web::test]
async fn test_create_book() {
    let repo = common::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo))
            .configure(books::config),
    )
    .await;

    let new_book = CreateBookSchema {
        title: "Test Book".into(),
        author_id: 1,
    };

    let resp = test::TestRequest::post()
        .uri("/api/v1/books")
        .set_json(&new_book)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 201);
    
    let book: BookModel = test::read_body_json(resp).await;
    assert_eq!(book.title, new_book.title);
}

#[actix_web::test]
async fn test_get_book() {
    let repo = common::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .configure(books::config),
    )
    .await;

    let new_book = CreateBookSchema {
        title: "Test Book".into(),
        author_id: 1,
    };
    let created_book = repo.create(new_book).await.unwrap();

    let resp = test::TestRequest::get()
        .uri(&format!("/api/v1/books/{}", created_book.id))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 200);
    let book: BookModel = test::read_body_json(resp).await;
    assert_eq!(book.id, created_book.id);
}

#[actix_web::test]
async fn test_update_book() {
    let repo = common::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .configure(books::config),
    )
    .await;

    let new_book = CreateBookSchema {
        title: "Original Title".into(),
        author_id: 1,
    };
    let created_book = repo.create(new_book).await.unwrap();

    let update = UpdateBookSchema {
        title: Some("Updated Title".into()),
    };

    let resp = test::TestRequest::patch()
        .uri(&format!("/api/v1/books/{}", created_book.id))
        .set_json(&update)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 200);
    let book: BookModel = test::read_body_json(resp).await;
    assert_eq!(book.title, "Updated Title");
}

#[actix_web::test]
async fn test_delete_book() {
    let repo = common::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .configure(books::config),
    )
    .await;

    let new_book = CreateBookSchema {
        title: "To Be Deleted".into(),
        author_id: 1,
    };
    let created_book = repo.create(new_book).await.unwrap();

    let resp = test::TestRequest::delete()
        .uri(&format!("/api/v1/books/{}", created_book.id))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 204);

    let resp = test::TestRequest::get()
        .uri(&format!("/api/v1/books/{}", created_book.id))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 404);
}