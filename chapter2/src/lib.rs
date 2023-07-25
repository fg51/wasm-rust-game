//use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::JsValue;
use web_sys::console;

use wasm_bindgen::JsCast;

use rand::prelude::{thread_rng, Rng};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    // Your code goes here!
    //console::log_1(&JsValue::from_str("Hello world!"));
    log::debug!("Hello world!");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    //context.move_to(300.0, 0.0);  // top of triangle
    //context.begin_path();
    //context.line_to(0.0, 600.0);  // bottom left of triangle
    //context.line_to(600.0, 600.0);  // bottom right of triangle
    //context.line_to(300.0, 0.0);  // back to top of triangle
    //context.close_path();
    //context.stroke();
    //context.fill();

    //sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 2);
    sierpinski(
        &context,
        [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
        (0, 255, 0),
        5,
    );

    Ok(())
}

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
) {
    let [top, left, right] = points;

    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    context.move_to(top.0, top.1); // top of triangle
    context.begin_path();
    context.line_to(left.0, left.1); // bottom left of triangle
    context.line_to(right.0, right.1); // bottom right of triangle
    context.line_to(top.0, top.1); // back to top of triangle
    context.close_path();
    context.stroke();
    context.fill();
}

fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
    depth: u8,
) {
    //draw_triangle(context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);
    draw_triangle(context, points, color);
    let depth = depth - 1;

    let [top, left, right] = points;
    if depth > 0 {
        let mut rng = thread_rng();

        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );

        //let left_middle = ((top.0 + left.0) / 2., (top.1 + left.1) / 2.);
        let left_middle = midpoint(top, left);
        //let right_middle = ((top.0 + right.0) / 2., (top.1 + right.1) / 2.);
        let right_middle = midpoint(top, right);
        //let bottom_middle = (top.0, right.1);
        let bottom_middle = midpoint(left, right);
        sierpinski(context, [top, left_middle, right_middle], next_color, depth);
        sierpinski(
            context,
            [left_middle, left, bottom_middle],
            next_color,
            depth,
        );
        sierpinski(
            context,
            [right_middle, bottom_middle, right],
            next_color,
            depth,
        );
    }
}

fn midpoint(point1: (f64, f64), point2: (f64, f64)) -> (f64, f64) {
    ((point1.0 + point2.0) / 2.0, (point1.1 + point2.1) / 2.)
}
