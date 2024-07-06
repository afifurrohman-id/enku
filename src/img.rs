use std::io::Cursor;

use image::{GenericImageView, ImageFormat};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Url};

use crate::util::rand;

use super::common;

const IMAGE_FORMAT: ImageFormat = ImageFormat::Png;

pub fn crop_image(buff: &[u8]) -> Vec<Vec<u8>> {
    const DIMENSION_SIZE: u32 = 512;
    const PART_SIZE: u32 = 100;
    // const PART_SIZE: u32 = 50;

    let mut img = image::load_from_memory_with_format(buff, IMAGE_FORMAT).unwrap();

    let (width, height) = img.dimensions();
    assert_eq!(width, DIMENSION_SIZE, "width must be 512");
    assert_eq!(height, DIMENSION_SIZE, "height must be 512");

    let parts_width = width / PART_SIZE;
    let parts_height = height / PART_SIZE;

    let mut results = Vec::new();

    for y in 0..parts_height {
        for x in 0..parts_width {
            let start_x = x * PART_SIZE;
            let start_y = y * PART_SIZE;

            let cropped_img = img.crop(start_x, start_y, PART_SIZE, PART_SIZE);

            let mut buffer = Cursor::new(Vec::new());
            cropped_img.write_to(&mut buffer, IMAGE_FORMAT).unwrap();
            let buffer = buffer.into_inner();
            results.push(buffer)
        }
    }
    results
}

pub fn create_imgs_url(imgs: &[Vec<u8>]) -> Vec<String> {
    let mut urls = Vec::new();

    for buff in imgs {
        let blob = common::bytes_to_js_blob(buff, IMAGE_FORMAT.to_mime_type());

        let url = Url::create_object_url_with_blob(&blob).unwrap();
        urls.push(url);
    }

    urls
}

#[allow(unused_variables)]
pub async fn load_image(query: &str) -> Vec<u8> {
    const IMAGE_COUNT: usize = 3;

    let x = rand(IMAGE_COUNT) + 1;

    let ext = IMAGE_FORMAT.extensions_str().first().unwrap();

    #[cfg(not(debug_assertions))]
    let url = format!(
        "https://acbtwffwdoxzwlfgpisu.supabase.co/storage/v1/object/public/projects/enku/images/random/{}/{}.{}",
        query.to_lowercase(),
        x,
        ext
    );

    #[cfg(debug_assertions)]
    let url = format!(
        "{}sample.{}",
        web_sys::window().unwrap().location().href().unwrap(),
        ext
    );

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(&url, &opts).unwrap();
    let mime_img = IMAGE_FORMAT.to_mime_type();

    req.headers().set("Accept", mime_img).unwrap();

    let window = web_sys::window().unwrap();

    let req = Request::new_with_str_and_init(&url, &opts).unwrap();
    let res = JsFuture::from(window.fetch_with_request(&req))
        .await
        .unwrap();

    let res = res.dyn_into::<Response>().unwrap();

    if let Some(ct) = res.headers().get("Content-Type").unwrap_or_default() {
        if !ct.contains(mime_img) {
            panic!("image must be {} format", mime_img);
        }
    }

    let js_buff = JsFuture::from(res.array_buffer().unwrap()).await.unwrap();

    common::js_buffer_to_bytes(&js_buff)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn create_imgs_url() {
        let buff = &[vec![0b01100001], vec![0b01100001], vec![0b01100001]];
        let urls = super::create_imgs_url(buff);
        assert_eq!(urls.len(), buff.len());
        urls.iter()
            .for_each(|url| assert!(url.starts_with("blob:")))
    }
}
