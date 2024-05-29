mod common;
mod data;
mod img;
mod puzzle;
mod util;

use data::Data;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
async fn init() {
    console_error_panic_hook::set_once();

    let data = Data::rand();
    puzzle::toggle_loading(true);

    let image = img::load_image(data.meaning()).await;
    let imgs = img::crop_image(&image);
    let urls = img::create_imgs_url(&imgs);
    puzzle::display_urls_randomly(&urls);

    puzzle::toggle_loading(false);

    puzzle::puzzle_handler(urls, data);
}
