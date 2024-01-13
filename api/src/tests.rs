use hyper::{Body, Request, StatusCode};
use rand::Rng;
use tower::ServiceExt;

use super::*;

async fn connect_pg() -> PgPool {
    let pg_url = "postgresql://postgres:password@localhost:5432/anomity".to_owned();
    PgPool::connect(&pg_url).await.unwrap()
}

#[tokio::test]
async fn hello_world() {
    let app = app(connect_pg().await).await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_register() {
    let app = app(connect_pg().await).await;
    let mut rng = rand::thread_rng();

    let json_payload = serde_json::json!({ "username": format!("test-{}", rng.gen_range(0..500)), "password": "test@complicated034", "email": format!("test-{}@test.dev", rng.gen_range(0..500)) });

    let response = app
        .oneshot(
            Request::builder()
                .method(hyper::Method::POST)
                .uri("/api/users")
                .header(hyper::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login() {
    let app = app(connect_pg().await).await;
    let mut rng = rand::thread_rng();

    let username = format!("test-{}", rng.gen_range(0..500));
    let password = "test@complicated034";
    let email = format!("test-{}@test.dev", rng.gen_range(0..500));

    let json_payload =
        serde_json::json!({ "username": username, "password": password, "email": email });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(hyper::Method::POST)
                .uri("/api/users")
                .header(hyper::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    //println!("{:?}", response.into_body().data().await);
    assert_eq!(response.status(), StatusCode::OK);

    //// login
    let login_payload = serde_json::json!({ "email": email, "password": password });

    let login_response = app
        .oneshot(
            Request::builder()
                .method(hyper::Method::POST)
                .uri("/api/users/login")
                .header(hyper::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(login_response.status(), StatusCode::OK);
}
