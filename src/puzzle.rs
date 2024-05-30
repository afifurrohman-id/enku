use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, Event, HtmlImageElement, HtmlLabelElement};

use super::common;
use super::data::Data;
use super::util;

pub fn puzzle_handler(urls: Vec<String>, data: Data) {
    let own_urls = Rc::new(urls);
    let own_data = Rc::new(data);

    if common::is_mobile() && !common::support_drag_drop() {
        mobile_event_handler(&own_data, &own_urls);
        return;
    }
    desktop_event_handler(&own_data, &own_urls);
}

fn mobile_event_handler(data: &Rc<Data>, urls: &Rc<Vec<String>>) {
    let imgs = common::query_all_element("figure img").unwrap();

    imgs.into_iter().for_each(|img| {
        let urls = Rc::clone(urls);
        let data = Rc::clone(data);

        common::set_text_content("#message", "Tekan 2 gambar untuk menukarnya");

        let click_handler = Closure::new(Box::new(move |e: Event| {
            swap_img_src_on_touch(e);

            let ok = compare_rand_urls(&urls);
            if ok {
                success_handler(&data);
            }
        }) as Box<dyn FnMut(_)>);

        img.add_event_listener_with_callback("touchstart", click_handler.as_ref().unchecked_ref())
            .unwrap();

        click_handler.forget();
    });
}

fn desktop_event_handler(data: &Rc<Data>, urls: &Rc<Vec<String>>) {
    let drag_over_handler =
        Closure::new(Box::new(|e: DragEvent| e.prevent_default()) as Box<dyn FnMut(_)>);

    let imgs = common::query_all_element("figure img").unwrap();
    for img in imgs {
        let urls = Rc::clone(urls);
        let data = Rc::clone(data);

        let img = img.dyn_into::<HtmlImageElement>().unwrap();

        img.add_event_listener_with_callback(
            "dragover",
            drag_over_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

        let drag_start_handler =
            Closure::new(Box::new(send_img_query_on_drag) as Box<dyn FnMut(_)>);

        img.add_event_listener_with_callback(
            "dragstart",
            drag_start_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

        let drop_handler = Closure::new(Box::new(move |e: DragEvent| {
            swap_img_src_on_drag(e);

            let ok = compare_rand_urls(&urls);
            if ok {
                success_handler(&data);
            }
        }) as Box<dyn FnMut(_)>);

        img.add_event_listener_with_callback("drop", drop_handler.as_ref().unchecked_ref())
            .unwrap();

        drop_handler.forget();
        drag_start_handler.forget();
    }

    drag_over_handler.forget();
}

fn success_handler(data: &Data) {
    // console::info_1(&"SUCCESS".into());

    common::insert_end(
        "main figure",
        format!(
            r#"
    <figcaption inert>{}</figcaption>
    <div class="blocked"></div>
                "#,
            data.meaning()
        )
        .as_str(),
    );

    display_choices(data)
}

fn display_choices(data: &Data) {
    let mut rand_list = String::new();

    util::randomize(data.choices(), |i, choice| {
        let i = i.to_string();
        let correct = data.choices()[0];

        let id = if choice == correct {
            "correct"
        } else {
            i.as_str()
        };

        rand_list.push_str(
            format!(
                r#"
            <input type="radio" name="choice" id="{}" value="{}" >
            <label for="{}" inert>{}</label>
            <br/>
        "#,
                id, choice, id, choice
            )
            .as_str(),
        )
    });

    common::insert_end(
        "main",
        format!(
            "
    <h1>Tebak maknanya</h1>
    <fieldset>
        {}
    </fieldset>
    ",
            rand_list
        )
        .as_str(),
    );

    set_choices_event()
}

fn set_choices_event() {
    let input_choices = common::query_all_element("fieldset input").unwrap();

    let click_handler = Closure::new(Box::new(|_| {
        let v = r#"<div class="blocked"></div>"#;
        common::insert_end("fieldset", v);

        colorize_labels()
    }) as Box<dyn FnMut(Event)>);

    input_choices.iter().for_each(|input| {
        input
            .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())
            .unwrap();
    });
    click_handler.forget();
}

fn colorize_labels() {
    let labels = common::query_all_element("label").unwrap();

    labels.into_iter().for_each(|label| {
        let label = label.dyn_into::<HtmlLabelElement>().unwrap();

        if label.html_for() == "correct" {
            label.set_class_name("correct")
        } else {
            label.set_class_name("incorrect")
        }
    })
}

pub fn toggle_loading(display: bool) {
    if display {
        common::insert_end(
            "body",
            r#"
        <div id="loader" class="blocked">
            <span class="spin"></span>
        </div>
        "#,
        );
    } else {
        common::query_element("#loader").unwrap().remove()
    }
}

pub fn display_urls_randomly(data: &[String]) {
    util::randomize(data, |i, url| {
        display_img_url(i, &url);
    });
}

fn compare_rand_urls(urls: &[String]) -> bool {
    let mut random_urls = Vec::new();
    let imgs = common::query_all_element("img").unwrap();

    for img in imgs {
        let img = img.dyn_into::<HtmlImageElement>().unwrap();

        let url = img.src();
        random_urls.push(url);
    }

    urls.eq(&random_urls)
}

fn send_img_query_on_drag(e: DragEvent) {
    e.data_transfer().unwrap().clear_data().unwrap();

    let mut t_id = e
        .target()
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap()
        .id();
    t_id.insert(0, '#');

    e.data_transfer()
        .unwrap()
        .set_data("text/plain", &t_id)
        .unwrap();
}

fn swap_img_src_on_drag(e: DragEvent) {
    e.prevent_default();

    let target = e.target().unwrap().dyn_into::<HtmlImageElement>().unwrap();

    let query = e.data_transfer().unwrap().get_data("text/plain").unwrap();

    let source = common::query_element(&query)
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();
    let temp = target.src();

    target.set_src(&source.src());
    source.set_src(&temp);
}

fn swap_img_src_on_touch(e: Event) {
    let clicked = common::query_element(r#"[data-touched="ok"]"#);

    let mut t_id = e
        .current_target()
        .unwrap()
        .dyn_into::<Element>()
        .unwrap()
        .id();
    t_id.insert(0, '#');

    let target = common::query_element(&t_id)
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

fn display_img_url(id: usize, url: &str) {
    common::insert_begin(
        ".wrapper",
        format!(
            r#"<img id="img-{}" src="{}" alt="dynamic image" title="dynamic image">"#,
            id, url
        )
        .as_str(),
    );
}
