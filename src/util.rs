use std::fmt::Display;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    js_sys::{Array, Math, Uint8Array},
    Blob, BlobPropertyBag, HtmlElement,
};

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

pub fn rand(count: usize) -> usize {
    Math::floor(Math::random() * count as f64) as usize
}

pub fn randomize<T, F>(data: &[T], mut f: F)
where
    T: Display + Clone,
    F: FnMut(usize, T),
{
    let mut data = Vec::from(data);
    loop {
        let count = data.len();

        if data.is_empty() || count == 0 {
            break;
        }

        let i = rand(count);

        let not_exist = data.get(i).is_none();
        if not_exist {
            continue;
        }

        let d = data.remove(i);

        f(count, d);
    }
}

#[cfg(test)]
mod tests {
    //TODO
}
