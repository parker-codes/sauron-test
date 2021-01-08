use log::trace;
use sauron::html::attributes::attr;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};

#[derive(Debug)]
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    count: i32,
}

impl App {
    pub fn new() -> Self {
        App { count: 0 }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main class="h-full w-full bg-gray-100 flex flex-col items-center">
                <h1 class="mt-24 text-3xl text-center">"Counter"</h1>

                <div class="mt-16 flex items-center" {attr("data-id", 1)}>
                    <button
                        class="p-1 w-8 h-8 rounded-full bg-red-300 border border-red-400 text-red-800 text-xl flex justify-center items-center"
                        on_click={|_| {
                            trace!("Decrement button clicked");
                            Msg::Decrement
                        }}
                    >
                        "-"
                    </button>

                    <div class="px-3">
                        {text(format!("Clicked: {}", self.count))}
                    </div>

                    <button
                        class="p-1 w-8 h-8 rounded-full bg-blue-300 border border-blue-400 text-blue-800 text-xl flex justify-center items-center"
                        on_click={|_| {
                            trace!("Increment button clicked");
                            Msg::Increment
                        }}
                    >
                        "+"
                    </button>
                </div>
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        trace!("App is updating with msg: {:?}", msg);
        match msg {
            Msg::Decrement => self.count -= 1,
            Msg::Increment => self.count += 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
