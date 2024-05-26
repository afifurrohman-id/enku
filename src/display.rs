use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, Event, HtmlImageElement};

use crate::util;

pub fn change_message(msg: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .get_element_by_id("message")
        .unwrap()
        .set_text_content(Some(msg));
}

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

pub fn send_img_src_on_drag(e: DragEvent) {
    let t_id = e
        .target()
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap()
        .id();
    e.data_transfer().unwrap().clear_data().unwrap();

    e.data_transfer()
        .unwrap()
        .set_data("text/plain", &t_id)
        .unwrap();
}

pub fn swap_img_src_on_drag(e: DragEvent) {
    e.prevent_default();

    let document = web_sys::window().unwrap().document().unwrap();

    let target = e.target().unwrap().dyn_into::<HtmlImageElement>().unwrap();

    let id = e.data_transfer().unwrap().get_data("text/plain").unwrap();
    let source = document
        .get_element_by_id(&id)
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();
    let temp = target.src();

    target.set_src(&source.src());
    source.set_src(&temp);
}

pub fn swap_img_src_on_touch(e: Event) {
    let document = web_sys::window().unwrap().document().unwrap();
    let clicked = document.query_selector(r#"[data-touched="ok"]"#).unwrap();

    let t_id = e
        .current_target()
        .unwrap()
        .dyn_into::<Element>()
        .unwrap()
        .id();

    let target = document
        .get_element_by_id(&t_id)
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    if let Some(clicked) = clicked {
        let clicked = clicked.dyn_into::<HtmlImageElement>().unwrap();

        let temp = target.src();
        target.set_src(&clicked.src());
        clicked.set_src(&temp);
        clicked.remove_attribute("data-touched").unwrap()
    } else {
        target.set_attribute("data-touched", "ok").unwrap();
    }
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
