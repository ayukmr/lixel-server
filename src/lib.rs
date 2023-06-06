use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};

use serde::{Serialize, Deserialize};

use std::fs;
use std::collections::HashMap;

// canvases
#[derive(Serialize, Deserialize)]
struct Canvases {
    // list of canvases
    canvases: Vec<Canvas>,
}

impl Canvases {
    // create canvases
    pub fn new(canvases: Vec<Canvas>) -> Self {
        Self { canvases }
    }
}

// canvas
#[derive(Serialize, Deserialize)]
pub struct Canvas {
    // canvas id
    id: u32,

    // canvas content
    content: Vec<Vec<String>>,
}

impl Canvas {
    // create canvas
    pub fn new(id: u32, content: Vec<Vec<String>>) -> Self {
        Self { id, content }
    }
}

// canvas in requests
#[derive(Serialize, Deserialize)]
pub struct ReqCanvas {
    // canvas content
    content: Vec<Vec<String>>,
}

// pixels
#[derive(Serialize, Deserialize)]
pub struct Pixels {
    // pixels
    pub pixels: Vec<Pixel>,
}

// pixel with color
#[derive(Serialize, Deserialize, Clone)]
pub struct Pixel {
    // pixel position
    x: usize,
    y: usize,

    // pixel hex color
    color: String,
}

// shorthand for creating json
macro_rules! json {
    () => {{
        let hash = HashMap::new();
        Json(hash)
    }};

    ($( $key:expr => $value:expr ),*) => {{
        let mut hash = HashMap::new();
        $( hash.insert(String::from($key), $value); )*

        Json(hash)
    }};
}

// create new canvas
pub fn create_canvas(req_canvas: ReqCanvas) -> impl IntoResponse {
    let canvas_id = random_id();

    update_canvases(|data| {
        // add new canvas to canvases
        let mut new_data = data;
        new_data.push(Canvas::new(canvas_id, req_canvas.content.to_vec()));

        new_data
    });

    (StatusCode::OK, json! { "id" => canvas_id })
}

// delete canvas
pub fn delete_canvas(canvas_id: u32) -> impl IntoResponse {
    update_canvases(|data| {
        data.into_iter()
            .filter(|canvas| canvas.id != canvas_id)
            .collect()
    });

    StatusCode::OK
}

// get content of canvas
pub fn get_canvas_content(canvas_id: u32) -> impl IntoResponse {
    let data = get_canvases();
    let canvas = data.into_iter().find(|canvas| canvas.id == canvas_id);

    match canvas {
        Some(canvas) => (StatusCode::OK, json! { "content" => canvas.content }),
        None => (StatusCode::NOT_FOUND, json! {}),
    }
}

// update content of canvas
pub fn update_canvas_content(canvas_id: u32, pixels: Vec<Pixel>) -> impl IntoResponse {
    update_canvases(|data| {
        let mut new_data = data;
        let canvas_idx = new_data.iter().position(|canvas| canvas.id == canvas_id);

        match canvas_idx {
            Some(canvas_idx) => {
                for pixel in pixels.to_vec() {
                    // update color of pixel
                    new_data[canvas_idx].content[pixel.y][pixel.x] = String::from(&pixel.color);
                }
            }
            None => {}
        }

        new_data
    });

    (StatusCode::OK, json! { "id" => canvas_id })
}

// generate random id
fn random_id() -> u32 {
    rand::random()
}

// get canvases as vector
fn get_canvases() -> Vec<Canvas> {
    let json = fs::read_to_string("./canvases.json").unwrap();
    let canvases: Canvases = serde_json::from_str(&json).unwrap();

    canvases.canvases
}

// update canvases with closure
fn update_canvases<F>(closure: F) where F: Fn(Vec<Canvas>) -> Vec<Canvas> {
    let canvases = closure(get_canvases());
    save_canvases(canvases);
}

// save canvases to file
fn save_canvases(canvases: Vec<Canvas>) {
    let canvases = Canvases::new(canvases);
    let json = serde_json::to_string(&canvases).unwrap();

    fs::write("./canvases.json", json).unwrap();
}
