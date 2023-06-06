use lixel::*;

use axum::{
    Json, Server, Router,
    response::IntoResponse,
    extract::Path,
    routing::{get, post, delete},
};

use tower_http::cors::CorsLayer;

// create new canvas
async fn create_canvas_req(Json(canvas): Json<ReqCanvas>) -> impl IntoResponse {
    create_canvas(canvas)
}

// delete canvas
async fn delete_canvas_req(Path(canvas_id): Path<u32>) -> impl IntoResponse {
    delete_canvas(canvas_id)
}

// get content of canvas
async fn get_canvas_content_req(Path(canvas_id): Path<u32>) -> impl IntoResponse {
    get_canvas_content(canvas_id)
}

// update content of canvas
async fn update_canvas_content_req(Path(canvas_id): Path<u32>, Json(pixels): Json<Pixels>) -> impl IntoResponse {
    update_canvas_content(canvas_id, pixels.pixels)
}

#[tokio::main]
async fn main() {
    // create routes
    let app = Router::new()
        .route("/canvas",                    post(create_canvas_req))
        .route("/canvas/:canvas_id",         delete(delete_canvas_req))
        .route("/canvas/:canvas_id/content", get(get_canvas_content_req)
                                            .put(update_canvas_content_req))
        .layer(CorsLayer::permissive());

    let addr = "127.0.0.1:5000".parse().unwrap();
    println!("listening on http://{}", addr);

    // start server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error occured");
}
