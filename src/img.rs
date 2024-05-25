use std::io::Cursor;

use image::{GenericImageView, ImageFormat};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Url};

use crate::util;

const FIX_SIZE: u32 = 512;

pub fn crop_image(buff: &[u8]) -> Vec<Vec<u8>> {
    let mut img = image::load_from_memory_with_format(buff, image::ImageFormat::Png)
        .expect("image must be valid png format");

    let (width, height) = img.dimensions();
    if width != FIX_SIZE && height != FIX_SIZE {
        panic!("width or height must be 512");
    }

    // const PART_SIZE: u32 = 50;
    const PART_SIZE: u32 = 100;

    let parts_width = width / PART_SIZE;
    let parts_height = height / PART_SIZE;

    let mut results = Vec::new();

    for y in 0..parts_height {
        for x in 0..parts_width {
            let start_x = x * PART_SIZE;
            let start_y = y * PART_SIZE;

            let cropped_img = img.crop(start_x, start_y, PART_SIZE, PART_SIZE);

            let mut buffer = Cursor::new(Vec::new());
            cropped_img.write_to(&mut buffer, ImageFormat::Png).unwrap();
            let buffer = buffer.into_inner();
            results.push(buffer)
        }
    }
    results
}

pub fn create_imgs_url(imgs: &[Vec<u8>]) -> Vec<String> {
    let mut correct_data = Vec::new();

    for buff in imgs {
        let blob = util::bytes_to_js_blob(buff, ImageFormat::Png.to_mime_type());

        let url = Url::create_object_url_with_blob(&blob).unwrap();
        correct_data.push(url);
    }

    correct_data
}

pub async fn load_image(query: &str) -> Vec<u8> {
    #[cfg(not(debug_assertions))]
    let url = format!(
        "https://source.unsplash.com/random/512x512?{}",
        query.to_lowercase()
    );

    #[cfg(debug_assertions)]
    let url = format!(
        "{}sample.png",
        web_sys::window().unwrap().location().href().unwrap()
    );

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(&url, &opts).expect("request must be valid");
    let png_mime = ImageFormat::Png.to_mime_type();

    req.headers()
        .set("Accept", png_mime)
        .expect("header must be valid");

    let window = web_sys::window().unwrap();

    let res = JsFuture::from(window.fetch_with_request(&req))
        .await
        .expect("fetch must be success");

    let res = res
        .dyn_into::<Response>()
        .expect("response must be valid type");
    assert_eq!(res.status(), 200, "first request should 200");

    let url = res.url().replace("fm=jpg", "fm=png");

    let req = Request::new_with_str_and_init(&url, &opts).expect("request must be valid");
    let res = JsFuture::from(window.fetch_with_request(&req))
        .await
        .expect("fetch must be success");

    let res = res
        .dyn_into::<Response>()
        .expect("response must be valid type");

    if let Some(ct) = res.headers().get("Content-Type").unwrap_or_default() {
        if !ct.contains(png_mime) {
            panic!("image must be png format");
        }
    }

    let js_buff = JsFuture::from(res.array_buffer().expect("js array buffer must be valid"))
        .await
        .expect("js array buffer must be valid");

    util::js_buffer_to_bytes(&js_buff)
}
