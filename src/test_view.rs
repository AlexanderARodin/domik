use egui::Color32;

use audio_server::AudioServer;
use raalog::log;


static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../../SoundFonts/Organ Chorus.SF2");


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct TestView {
    needsRepaint: bool,
    pub title: String,
    audio: AudioServer,
}
impl TestView {
    pub fn new() -> Self {
        Self{
            needsRepaint: false,
            title: "testing view".to_owned(),
            audio: AudioServer::new(),
        }
    }
}
impl Default for TestView {
    fn default() -> Self {
        Self::new()
    }
}

//  //  //  //  //  //  //  //
//      impl
//  //  //  //  //  //  //  //
impl TestView {
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        let b = ui.button("rrr");
            if b.clicked() {
                self.audio = AudioServer::new();
            }
        ui.separator();
        if self.audio.state() == "REALTIME" {
            self.needsRepaint = true;
        }
        ui.scope(|ui|{
            let btn_txt;
            let clr;
            match self.audio.state() {
                "inactive" => {
                    btn_txt = "[-]";
                    clr = Color32::BROWN;
                },
                "running" => {
                    btn_txt = "[+]";
                    clr = Color32::GREEN;
                },
                "REALTIME" => {
                    btn_txt = "[#]";
                    clr = Color32::YELLOW;
                },
                _ => {
                    btn_txt = "[?]";
                    clr = Color32::GRAY;
                },
            };
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = clr;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = clr;
            let btn = ui.button(btn_txt);
            if btn.clicked() {
                if self.audio.state() == "inactive" {
                    let _ = self.audio.exec("start");
                }else{
                    let _ = self.audio.exec("stop");
                }
            }
        });
        ui.separator();
        ui.separator();
        ui.label("select synthesizer:");
        ui.horizontal( |ui| {
                let btnN = ui.button( "None" );
                if btnN.clicked(){
                    let setup = "[AudioSource]\nName = 'None'";
                    self.applySetup( setup, None );
                }
                let btnS = ui.button( "SimpleSynth" );
                if btnS.clicked(){
                    let setup = "[AudioSource]\nName = 'Simple'";
                    self.applySetup( setup, None );
                }
                let btnRA = ui.button( "RustySynt - Strings" );
                if btnRA.clicked(){
                    let setup = "[AudioSource]\nName = 'RustySynth'";
                    self.applySetup( setup, Some(SF_STRINGS) );
                }
                let btnRB = ui.button( "RustySynt - Piano" );
                if btnRB.clicked(){
                    let setup = "[AudioSource]\nName = 'RustySynth'";
                    self.applySetup( setup, Some(SF_PIANO) );
                }
                let btnRA = ui.button( "Sequencer:Simple" );
                if btnRA.clicked(){
                    let setup = "[AudioSource]\nName = 'Sequencer'\n[AudioSource.Sequencer]\nMainVoice = 'Simple'";
                    self.applySetup( setup, None );
                }
                let btnRA = ui.button( "Sequencer:RustySynt - Strings" );
                if btnRA.clicked(){
                    let setup = "[AudioSource]\nName = 'Sequencer'\n[AudioSource.Sequencer]\nMainVoice = 'RustySynth'";
                    self.applySetup( setup, Some(SF_STRINGS) );
                }
            });
        ui.separator();
        ui.separator();
        ui.label("playing notes:");
        ui.horizontal( |ui| {
            let btnO = ui.button( "[-]" );
            if btnO.clicked(){
                if let Err(e) = self.audio.exec( "seq 1") {
                    log::error(&e.to_string());
                }
            }
            let btnO1 = ui.button( "[+]" );
            if btnO1.clicked(){
                if let Err(e) = self.audio.exec( "seq auto") {
                    log::error(&e.to_string());
                }
            }
            ui.separator();
            let mut test_txt = "on60#127";
            let btnA = ui.button( test_txt );
            if btnA.clicked(){
                if let Err(e) = self.audio.exec( test_txt ) {
                    log::error(&e.to_string());
                }
            }
                    test_txt = "on67";
            let btnA1 = ui.button( test_txt );
            if btnA1.clicked(){
                if let Err(e) = self.audio.exec( test_txt ) {
                    log::error(&e.to_string());
                }
            }
                    test_txt = "on71#1";
            let btnA2 = ui.button( test_txt );
            if btnA2.clicked(){
                if let Err(e) = self.audio.exec( test_txt ) {
                    log::error(&e.to_string());
                }
            }
                    test_txt = "off60";
            let btnB = ui.button( test_txt );
            if btnB.clicked(){
                if let Err(e) = self.audio.exec( test_txt ) {
                    log::error(&e.to_string());
                }
            }
        });

        if self.needsRepaint {
            self.needsRepaint = false;
            ui.ctx().request_repaint();
        }
    }

    fn applySetup(&mut self, setup: &str, data: Option<&[u8]> ) {
        if let Err(e) = self.audio.config(setup, data ) {
            log::error(&e.to_string());
        }
    }
}

