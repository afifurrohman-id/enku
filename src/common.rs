use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    js_sys::{Array, Uint8Array},
    Blob, BlobPropertyBag, Element, HtmlElement,
};

pub fn query_all_element(query: &str) -> Option<Vec<Element>> {
    let document = web_sys::window().unwrap().document().unwrap();

    let elements = document.query_selector_all(query).unwrap();
    if elements.length() == 0 {
        return None;
    }

    let mut result = Vec::new();
    for i in 0..elements.length() {
        let element = elements.item(i).unwrap().dyn_into::<Element>().unwrap();
        result.push(element)
    }

    Some(result)
}

pub fn query_element(query: &str) -> Option<Element> {
    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector(query).unwrap()
}

pub fn set_text_content(query: &str, content: &str) {
    query_element(query)
        .unwrap()
        .set_text_content(Some(content));
}

pub fn insert_begin(query: &str, element: &str) {
    query_element(query)
        .unwrap()
        .insert_adjacent_html("afterBegin", element)
        .unwrap()
}

pub fn insert_end(query: &str, element: &str) {
    query_element(query)
        .unwrap()
        .insert_adjacent_html("beforeEnd", element)
        .unwrap()
}

pub fn is_mobile() -> bool {
    let window = web_sys::window().unwrap();

    let touch_start_exists = window.ontouchstart().is_some();
    let touch_points = window.navigator().max_touch_points();
    touch_start_exists || touch_points > 0
}

pub fn support_drag_drop() -> bool {
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    div.draggable()
        && div.ondragstart().is_some()
        && div.ondrop().is_some()
        && div.ondragover().is_some()
}

pub fn js_buffer_to_bytes(js_buff: &JsValue) -> Vec<u8> {
    let js_bytes = Uint8Array::new(js_buff);
    js_bytes.to_vec()
}

pub fn bytes_to_js_blob(buffer: &[u8], mime: &str) -> Blob {
    let js_bytes = Uint8Array::new_with_length(buffer.len() as u32);
    js_bytes.copy_from(buffer);

    let arr = Array::new();
    arr.push(&js_bytes.buffer());

    let mut opts = BlobPropertyBag::new();
    opts.type_(mime);

    Blob::new_with_u8_array_sequence_and_options(&arr, &opts).unwrap()
}
