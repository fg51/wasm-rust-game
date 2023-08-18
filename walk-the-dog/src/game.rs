use std::collections::HashMap;

use async_trait::async_trait;

use anyhow::Result;

use web_sys::HtmlImageElement;

use serde::Deserialize;

use crate::engine::Rect;
use crate::engine::{Game, Renderer};
use crate::engine::{KeyState, Point};
use crate::{browser, engine};

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
    position: Point,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self {
            image: None,
            sheet: None,
            frame: 0,
            position: Point { x: 0, y: 0 },
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sheet: Sheet =
            serde_wasm_bindgen::from_value(browser::fetch_json("/assets/rhb.json").await?)
                .expect("Could not convert rhb.json into a Sheet structure");
        let image = engine::load_image("/assets/rhb.png").await?;

        Ok(Box::new(WalkTheDog {
            image: Some(image),
            sheet: Some(sheet),
            frame: self.frame,
            position: self.position,
        }))
    }

    fn update(&mut self, keystate: &KeyState) {
        let mut velocity = Point { x: 0, y: 0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3;
        }
        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3;
        }
        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3;
        }
        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3;
        }
        self.position.x += velocity.x;
        self.position.y += velocity.y;

        self.frame = if self.frame < 23 { self.frame + 1 } else { 0 };
    }

    fn draw(&self, renderer: &Renderer) {
        let current_sprite = self.frame / 3 + 1;
        let frame_name = format!("Run ({}).png", current_sprite);
        //context.clear_rect(0.0, 0.0, 600.0, 600.0);
        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");

        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        let _ = self.image.as_ref().map(|image| {
            renderer.draw_image(
                image,
                &Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
                &Rect {
                    x: self.position.x.into(),
                    y: self.position.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
            );
        });
    }
}

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}
