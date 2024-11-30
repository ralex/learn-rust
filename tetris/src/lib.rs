use js_sys::{Function, Reflect};
use tetris::{Direction, Tetris};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
    export_components, h,
    hooks::{use_callback, use_effect, use_state, Deps},
    props::Style,
    Callback, Component, VNode,
};
use web_sys::{window, Element, HtmlElement, KeyboardEvent};

mod shape;
mod tetris;

pub struct App {
    width: u32,
    height: u32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(App {
            width: Reflect::get(&value, &"width".into())?
                .as_f64()
                .unwrap_or(10.0) as u32,
            height: Reflect::get(&value, &"height".into())?
                .as_f64()
                .unwrap_or(30.0) as u32,
        })
    }
}

impl Component for App {
    fn render(&self) -> VNode {
        let tetris = use_state(|| Tetris::new(self.width, self.height));
        let cells: VNode = tetris
            .value()
            .iter_positions()
            .map(|pos| {
                if let Some(typ) = tetris.value().get(pos) {
                    h!(div)
                        .style(&Style::new().margin_top("-0.2em"))
                        .build(typ) // Create a div for each position
                } else {
                    h!(div).build("") // Fallback for None
                }
            })
            .collect();
        let speed = use_state(|| 500);
        let focus_ref = use_callback(Callback::from(
            |element: Option<Element>| {
                if let Some(element) = element {
                    if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                        let _ = html_element.focus();
                    }
                }
            }),
            Deps::none(),
        );


        use_effect(
            {
                let tetris = tetris.clone();
                let speed = *speed.value();

                move || {
                    let tick_closure = Closure::new({
                        let mut tetris = tetris.clone();
                        move || {
                            tetris.set(|mut tetris| {
                                tetris.tick();
                                tetris
                            });
                        }
                    });

                    let handle = window()
                        .unwrap_throw()
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                            speed,
                        )
                        .unwrap_throw();

                    move || {
                        drop(tick_closure);
                        window().unwrap_throw().clear_interval_with_handle(handle);
                    }
                }
            },
            Deps::some(*speed.value()),
        );

        let handle_key_down = {
            let mut tetris = tetris.clone();
            let mut speed = speed.clone();

            Callback::from(move |event: KeyboardEvent| {
                let code = event.code();
                let direction = match &*code {
                    "ArrowLeft" => Some(Direction::Left),
                    "ArrowRight" => Some(Direction::Right),
                    _ => None,
                };

                if let Some(direction) = direction {
                    tetris.set(|mut tetris| {
                        tetris.shift(direction);
                        tetris
                    });
                }

                if code == "ArrowUp" {
                    tetris.set(|mut tetris| {
                        tetris.rotate();
                        tetris
                    });
                } else if code == "ArrowDown" {
                    speed.set(|_| 50)
                }
            })
        };

        let handle_key_up = {
            let mut speed = speed.clone();

            Callback::from({
                move |event: KeyboardEvent| {
                    let code = event.code();
                    if code == "ArrowDown" {
                        speed.set(|_| 500)
                    }
                }
            })
        };

        h!(div)
            .ref_callback(&focus_ref)
            .tabindex(0)
            .on_keydown(&handle_key_down)
            .on_keyup(&handle_key_up)
            .style(
                &Style::new()
                    .display("inline-grid")
                    .grid_template(format!(
                        "repeat({}, 1em) / repeat({}, 1em)",
                        self.height, self.width
                    ))
                    .border("1px solid grey")
                    .outline("none"),
            )
            .build(cells)
    }
}

export_components! { App }
