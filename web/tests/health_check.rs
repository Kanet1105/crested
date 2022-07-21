use std::net::TcpListener;

#[tokio::test]
async fn health_check() {
    // run the web server.
    spawn_app();
    let client = reqwest::Client::new();

    // send a request.
    let response = client
        .get("http://127.0.0.1:8080")
        .send()
        .await
        .expect("Failed to execute the request.");

    // assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind the port");
    let server = web::run(listener).expect("Failed to bind the address.");
    let _ = tokio::spawn(server);
}