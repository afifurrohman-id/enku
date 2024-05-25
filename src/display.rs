use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlImageElement};

use crate::util;

pub fn toggle_loading(display: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    if display {
        document
            .body()
            .unwrap()
            .insert_adjacent_html(
                "beforeEnd",
                r#"
        <div id="loader" class="blocked">
            <span class="spin"></span>
        </div>"#,
            )
            .unwrap()
    } else {
        document
            .get_element_by_id("loader")
            .expect("loader element should exists")
            .remove()
    }
}

pub fn display_urls_randomly(data: &[String]) {
    util::randomize(data, |i, url| {
        display_img_url(i, &url);
    });
}

pub fn compare_rand_urls(urls: &[String]) -> bool {
    let document = web_sys::window().unwrap().document().unwrap();
    let mut random_urls = Vec::new();
    let imgs = document.query_selector_all("img").unwrap();

    for i in 0..imgs.length() {
        let img = imgs
            .item(i)
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        let url = img.src();
        random_urls.push(url);
    }

    urls.eq(&random_urls)
}

pub fn send_src_url(e: DragEvent) {
    let el = e.target().unwrap().dyn_into::<HtmlImageElement>().unwrap();
    e.data_transfer().unwrap().clear_data().unwrap();

    e.data_transfer()
        .unwrap()
        .set_data("text/plain", &el.id())
        .unwrap();
}

pub fn swap_url(e: DragEvent) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    e.prevent_default();
    let el = e.target().unwrap().dyn_into::<HtmlImageElement>().unwrap();
    let src = el.src();

    let id = e.data_transfer().unwrap().get_data("text/plain").unwrap();
    let dst_el = document
        .get_element_by_id(&id)
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();
    let dst_src = dst_el.src();

    dst_el.set_src(&src);
    el.set_src(&dst_src);
}

pub fn display_img_url(id: usize, url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .query_selector(".wrapper")
        .expect("div element must be valid")
        .expect("div element must be exist")
        .insert_adjacent_html(
            "afterBegin",
            format!(
                r#"<img id="{}" src="{}" alt="dynamic image" title="dynamic image">"#,
                id, url
            )
            .as_str(),
        )
        .expect("image element must be valid");
}
