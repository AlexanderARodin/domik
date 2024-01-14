#![allow(non_snake_case)]

mod sound_check;

mod root_app;
use root_app::RootApp;

mod console_log;
use console_log::*;

#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    log("MAIN has beed entered..");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.,500.])
            .with_min_inner_size([200.,300.]),
        ..Default::default()
    };

    let res = eframe::run_native(
        "DoMiK",
        options,
        Box::new( |cc| Box::new(RootApp::new(cc)) )
   );
    log("..MAIN ends");
    return res;
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    log("[WASM]: MAIN has beed entered..");

    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "egui_canvas_id",
                options,
                Box::new( |cc| Box::new(RootApp::new(cc)) ),
            )
            .await
            .expect("failure with starting EFRAME");
    });
    log("..MAIN ends");
}


