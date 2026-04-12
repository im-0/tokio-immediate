// SPDX-License-Identifier: Apache-2.0 OR MIT

#![warn(clippy::pedantic)]

use ::eframe::wasm_bindgen::JsCast as _;
use ::egui::CentralPanel;

fn main() {
    ::console_error_panic_hook::set_once();

    let web_options = ::eframe::WebOptions::default();

    ::wasm_bindgen_futures::spawn_local(async {
        let document = ::web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<::web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        ::eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(ExampleApp {}))),
            )
            .await
            .expect("Failed to start eframe");

        // Remove the loading text and spinner.
        let loading_text = document
            .get_element_by_id("loading_text")
            .expect("Loading text element not found");
        loading_text.remove();
    });
}

struct ExampleApp {}

impl ::eframe::App for ExampleApp {
    fn ui(&mut self, ui: &mut ::egui::Ui, _frame: &mut ::eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Hello World!");
            })
        });
    }
}
