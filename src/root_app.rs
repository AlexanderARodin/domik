#![allow(non_snake_case)]

use crate::log_view::LogView;
use crate::raadbg::log;
use crate::base_domik_view::BaseDomikView;
use crate::midi_audio::MidiAudio;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
    pub example_text: String,

    #[serde(skip)]
    log_view: LogView,
    #[serde(skip)]
    midi_audio: MidiAudio,
    #[serde(skip)]
    base_domik_view: BaseDomikView,
    #[serde(skip)]
    is_wasm: bool,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {
            example_text:"<empty>".to_owned(), 
            log_view: LogView::new(),
            midi_audio: MidiAudio::new(),
            base_domik_view: BaseDomikView::new(),
            is_wasm:is_wasm(), 
        }
    }
}


#[ cfg(not(target_arch = "wasm32")) ]
fn is_wasm() -> bool  {
    false
}
#[ cfg(target_arch = "wasm32") ]
fn is_wasm() -> bool  {
    true
}

impl RootApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage{
            log::simple("trying to load..");
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for RootApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        log::simple("saving..");
        eframe::set_value(storage, eframe::APP_KEY, self);
        log::simple("..saved");
    }

    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        
        egui::TopBottomPanel::bottom("bot_pan_banner").show( ctx, |ui| {
            self.showBanner( ui );
        });
        
        egui::Window::new(self.base_domik_view.title.clone()).show( ctx, |ui| {
            self.base_domik_view.updateUI( ui, &mut self.example_text );
        });

        egui::Window::new("logs").show( ctx, |ui| {
            self.log_view.updateUI( ui );
        });
    }
}

impl RootApp {
    fn showBanner( &mut self, ui: &mut egui::Ui ){
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to("eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
    }
}
