use rust_api_template::books::CreateBookSchema;
use rust_api_template::error::Error;

mod common;

#[actix_web::test]
async fn test_create_book_in_repository() {
    let repo = common::setup_test_db().await;
    
    let new_book = CreateBookSchema {
        title: "New Book".into(),
        author_id: 1,
    };

    let result = repo.create(new_book).await;
    assert!(result.is_ok());
    
    let book = result.unwrap();
    assert_eq!(book.title, "New Book");
    assert_eq!(book.author_id, 1);
}

#[actix_web::test]
async fn test_get_nonexistent_book() {
    let repo = common::setup_test_db().await;
    let result = repo.get(999999).await;
    
    assert!(matches!(result, Err(Error::NotFound(_))));
}