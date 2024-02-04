use egui::Color32;

use audio_server::AudioServer;
use raalog::log;


static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../../SoundFonts/Organ Chorus.SF2");
static TEST_XXX: &str = r#"
    autoexec = [
        'stop',
        { setup = 'NoSynth' },
        'start',
    ]
    [Sequence]
    notes = [ 
             [1  , 'on',  90, 80  ],
        0.5, [1  , 'off', 90, 80  ],
             [1  , 'on',  91, 80  ],
        0.5, [1  , 'off', 91, 80  ],
             [1  , 'on',  92, 80  ],
        0.5, [1  , 'off', 92, 80  ],
             [1  , 'on',  91, 80  ],
        0.5, [1  , 'off', 91, 80  ],
        1.0, [1  , 'off', 92, 80  ],
    ]
    transpose = 7
    speed = 2.0
"#;

static TEST_SEQ: &str = r#"
    [Sequence]
    notes = [ 
             [1  , 'on',  90, 80  ],
        0.5, [1  , 'off', 90, 80  ],
             [1  , 'on',  91, 80  ],
        0.5, [1  , 'off', 91, 80  ],
             [1  , 'on',  92, 80  ],
        0.5, [1  , 'off', 92, 80  ],
             [1  , 'on',  91, 80  ],
        0.5, [1  , 'off', 91, 80  ],
        1.0, [1  , 'off', 92, 80  ],
    ]
    transpose = 7
    speed = 2.0
"#;


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
                let cfg = audio_server::Config::CoreConfigFromStr( TEST_XXX, Vec::new() );
                if let Err(e) = self.audio.load_config(&cfg) {
                    log::error(&e.to_string());
                }
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
                self.applySetup( TEST_SEQ, None );
                self.doExec( "play once" );
            }
            let btnO0 = ui.button( "[+]" );
            if btnO0.clicked(){
                self.applySetup( TEST_SEQ, None );
                self.doExec( "play loop" );
            }
            let mut test_txt;
                    test_txt = "seq auto";
            let btnO1 = ui.button( "[@]" );
            if btnO1.clicked(){
                self.doExec( test_txt );
            }
            ui.separator();
                    test_txt = "[1  , 'on',  60, 127 ]";
            let btnA = ui.button( test_txt );
            if btnA.clicked(){
                self.doExec( test_txt );
            }
                    test_txt = "[1  , 'on',  67,  64 ]";
            let btnA1 = ui.button( test_txt );
            if btnA1.clicked(){
                self.doExec( test_txt );
            }
                    test_txt = "[1  , 'on',  71,   1 ]";
            let btnA2 = ui.button( test_txt );
            if btnA2.clicked(){
                self.doExec( test_txt );
            }
                    test_txt = "[1  , 'off', 60,   1 ]";
            let btnB = ui.button( test_txt );
            if btnB.clicked(){
                self.doExec( test_txt );
            }
        });

        if self.needsRepaint {
            self.needsRepaint = false;
            ui.ctx().request_repaint();
        }
    }

    fn doExec(&mut self, cmd: &str ) {
        if let Err(e) = self.audio.exec( cmd ) {
            log::error(&e.to_string());
        }
    }
    fn applySetup(&mut self, setup: &str, data: Option<&[u8]> ) {
        if let Err(e) = self.audio.config(setup, data ) {
            log::error(&e.to_string());
        }
    }
}

