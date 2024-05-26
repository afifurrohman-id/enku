use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, Event, HtmlImageElement, HtmlLabelElement};

use crate::{data::Data, display, util};

pub fn puzzle_handler(urls: Vec<String>, data: Data) {
    let document = web_sys::window().unwrap().document().unwrap();
    let imgs = document.query_selector_all("figure img").unwrap();

    let urls = Rc::new(urls);
    let data = Rc::new(data);

    let drag_over_handler =
        Closure::new(Box::new(|e: DragEvent| e.prevent_default()) as Box<dyn FnMut(_)>);

    for i in 0..imgs.length() {
        let img = imgs
            .item(i)
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        let urls = Rc::clone(&urls);
        let data = Rc::clone(&data);

        if util::is_mobile() && !util::support_drag_drop() {
            display::change_message("Tekan 2 gambar untuk menukarnya");

            let click_handler = Closure::new(Box::new(move |e: Event| {
                display::swap_img_src_on_touch(e);

                let ok = display::compare_rand_urls(&urls);
                if ok {
                    success_handler(&data);
                }
            }) as Box<dyn FnMut(_)>);

            img.add_event_listener_with_callback(
                "touchstart",
                click_handler.as_ref().unchecked_ref(),
            )
            .unwrap();

            click_handler.forget();
            continue;
        }

        img.add_event_listener_with_callback(
            "dragover",
            drag_over_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

        let drag_start_handler =
            Closure::new(Box::new(display::send_img_src_on_drag) as Box<dyn FnMut(_)>);

        img.add_event_listener_with_callback(
            "dragstart",
            drag_start_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

        let drop_handler = Closure::new(Box::new(move |e: DragEvent| {
            display::swap_img_src_on_drag(e);

            let ok = display::compare_rand_urls(&urls);
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

    let document = web_sys::window().unwrap().document().unwrap();
    let figure = document.query_selector("main figure").unwrap().unwrap();
    figure
        .insert_adjacent_html(
            "beforeEnd",
            format!(
                r#"
    <figcaption>{}</figcaption>
    <div class="blocked"></div>
    "#,
                data.meaning()
            )
            .as_str(),
        )
        .unwrap();

    display_choices(data)
}

fn display_choices(data: &Data) {
    let mut rand_list = String::new();
    util::randomize(data.choices(), |i, choice| {
        let i = i.to_string();
        let id = if choice == data.choices()[0] {
            "correct"
        } else {
            i.as_str()
        };

        rand_list.push_str(
            format!(
                r#"
            <input type="radio" name="choice" id="{}" value="{}" >
            <label for="{}">{}</label>
            <br/>
        "#,
                id, choice, id, choice
            )
            .as_str(),
        )
    });

    let main = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("main")
        .unwrap()
        .unwrap();

    main.insert_adjacent_html(
        "beforeEnd",
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
    )
    .unwrap();

    set_choices_event(main)
}

fn set_choices_event(main: Element) {
    let fieldset = main.query_selector("fieldset").unwrap().unwrap();

    let input_choices = fieldset.query_selector_all("input").unwrap();

    let click_handler = Closure::new(Box::new(|e: DragEvent| {
        let v = r#"<div class="blocked"></div>"#;
        let this = e.current_target().unwrap().dyn_into::<Element>().unwrap();

        this.parent_element()
            .unwrap()
            .insert_adjacent_html("beforeEnd", v)
            .unwrap();

        colorize_label()
    }) as Box<dyn FnMut(_)>);

    for i in 0..input_choices.length() {
        let input = input_choices.item(i).unwrap();
        input
            .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())
            .unwrap();
    }
    click_handler.forget();
}

fn colorize_label() {
    let input_choices = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector_all("label")
        .unwrap();

    for i in 0..input_choices.length() {
        let input = input_choices
            .item(i)
            .unwrap()
            .dyn_into::<HtmlLabelElement>()
            .unwrap();
        if input.html_for() == "correct" {
            input.set_class_name("correct")
        } else {
            input.set_class_name("incorrect")
        }
    }
}
