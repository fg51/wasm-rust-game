use std::collections::HashMap;

use async_trait::async_trait;

use anyhow::anyhow;
use anyhow::Result;

use web_sys::HtmlImageElement;

use serde::Deserialize;

use crate::engine::Rect;
use crate::engine::{Game, Renderer};
use crate::engine::{KeyState, Point};
use crate::{browser, engine};

mod red_hat_boy_states;
use red_hat_boy_states::{Event, RedHatBoyState, RedHatBoyStateMachine};

const HEIGHT: i16 = 600;

pub enum WalkTheDog {
    Loading,
    Loaded(RedHatBoy),
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self::Loading
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            Self::Loading => {
                let json = browser::fetch_json("/assets/rhb.json").await?;

                let rhb = RedHatBoy::new(
                    serde_wasm_bindgen::from_value(json)
                        .expect("Could not convert rhb.json into a Sheet structure"),
                    engine::load_image("/assets/rhb.png").await?,
                );
                Ok(Box::new(Self::Loaded(rhb)))
            }
            Self::Loaded(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let Self::Loaded(rhb) = self {
            let mut velocity = Point { x: 0, y: 0 };
            if keystate.is_pressed("ArrowDown") {
                rhb.slide();
            }
            if keystate.is_pressed("ArrowUp") {
                velocity.y -= 3;
            }
            if keystate.is_pressed("ArrowRight") {
                velocity.x += 3;
                rhb.run_right();
            }
            if keystate.is_pressed("ArrowLeft") {
                velocity.x -= 3;
            }
            if keystate.is_pressed("Space") {
                rhb.jump();
            }

            rhb.update();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        if let Self::Loaded(rhb) = self {
            rhb.draw(renderer);
        }
    }
}

#[derive(Deserialize, Clone)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize, Clone)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize, Clone)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        Self {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
            sprite_sheet: sheet,
            image,
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let frame_name = format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        );

        let sprite = self
            .sprite_sheet
            .frames
            .get(&frame_name)
            .expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &Rect {
                x: self.state_machine.context().position.x.into(),
                y: self.state_machine.context().position.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
        );
    }

    fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }

    fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
    }

    fn slide(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Slide);
    }

    fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }
}
