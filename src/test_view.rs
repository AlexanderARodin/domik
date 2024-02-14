use egui::Color32;

use audio_server::{ AudioServer, Config, Exec };
use raalog::log;


static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../../SoundFonts/Organ Chorus.SF2");
static sf_list:[&[u8];2] = [SF_PIANO,SF_STRINGS];

static TEST_SEQ: &str = r#"
    [workflows]
    play-once = [
        { load = 'Sequence.notes' },
        'play',
    ]
    play-loopA = [
        { speed = { '0.5' = { load = 'Sequence.notes' } } },
        'play-loop',
    ]
    play-loopB = [
        { transpose = { 7 = { load = 'Sequence.notes' } } },
        'play-loop',
    ]
    play-loopC = [
        { transpose = { 7 = { speed = { '0.5' = { load = 'Sequence.notes' } } } } },
        { speed = { 2 = { transpose = { -5 = { load = 'Sequence.notes' } } } } },
        'play-loop',
    ]
    [Sequence]
    notes = [ 
             [1  , 'on',  90, 80  ],
        0.5, [1  , 'off', 90, 80  ],
             [1  , 'on',  91, 50  ],
        0.5, [1  , 'off', 91, 80  ],
             [1  , 'on',  92, 90  ],
        0.5, [1  , 'off', 92, 80  ],
             [1  , 'on',  91, 50  ],
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
                let core_cfg = Config::CoreConfigFromStr( TEST_XXX, sf_list.to_vec() );
                if let Err(e) = self.audio.load_config(&core_cfg) {
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
                    self.doExec( &Exec::CoreExec( "start" ) );
                }else{
                    self.doExec( &Exec::CoreExec( "stop" ) );
                }
            }
        });
        ui.separator();
        ui.separator();
        ui.label("select synthesizer:");
        ui.horizontal( |ui| {
                let btnN = ui.button( "None" );
                if btnN.clicked(){
                    self.doExec( &Exec::CoreExec( "None" ) );
                }
                let btnS = ui.button( "SimpleSynth" );
                if btnS.clicked(){
                    self.doExec( &Exec::CoreExec( "Simple" ) );
                }
                let btnRA = ui.button( "RustySynt - Strings" );
                if btnRA.clicked(){
                    self.doExec( &Exec::CoreExec( "rstString" ) );
                }
                let btnRB = ui.button( "RustySynt - Piano" );
                if btnRB.clicked(){
                    self.doExec( &Exec::CoreExec( "rstPiano" ) );
                }
                let btnRA = ui.button( "Sequencer:Simple" );
                if btnRA.clicked(){
                    self.doExec( &Exec::CoreExec( "sqSimple" ) );
                }
                let btnRA = ui.button( "Sequencer:RustySynt - Strings" );
                if btnRA.clicked(){
                    self.doExec( &Exec::CoreExec( "sqString" ) );
                }
            });
        ui.separator();
        ui.separator();
        ui.label("playing notes:");
        ui.horizontal( |ui| {
            let btnO = ui.button( "[-]" );
            if btnO.clicked(){
                self.applySetup( TEST_SEQ );
                self.doExec( &Exec::OrdinaryExec( "play-once" ));
            }
            let btnO0 = ui.button( "[+]" );
            if btnO0.clicked(){
                self.applySetup( TEST_SEQ );
                self.doExec( &Exec::OrdinaryExec( "play-loopA" ));
            }
            let mut test_txt;
                    test_txt = "seq auto";
            let btnO1 = ui.button( "[@]" );
            if btnO1.clicked(){
                self.applySetup( TEST_SEQ );
                self.doExec( &Exec::OrdinaryExec( "play-loopB" ));
            }
            let btn33 = ui.button( "[!]" );
            if btn33.clicked(){
                self.applySetup( TEST_SEQ );
                self.doExec( &Exec::OrdinaryExec( "play-loopC" ));
            }
            ui.separator();
                    test_txt = "[1  , 'on',  60, 127 ]";
            let btnA = ui.button( test_txt );
            if btnA.clicked(){
                self.doExec( &Exec::DBG( "midi", test_txt ) );
            }
                    test_txt = "[1  , 'on',  67,  64 ]";
            let btnA1 = ui.button( test_txt );
            if btnA1.clicked(){
                self.doExec( &Exec::DBG( "midi", test_txt ) );
            }
                    test_txt = "[1  , 'on',  71,   1 ]";
            let btnA2 = ui.button( test_txt );
            if btnA2.clicked(){
                self.doExec( &Exec::DBG( "midi", test_txt ) );
            }
                    test_txt = "[1  , 'off', 60,   1 ]";
            let btnB = ui.button( test_txt );
            if btnB.clicked(){
                self.doExec( &Exec::DBG( "midi", test_txt ) );
            }
        });

        if self.needsRepaint {
            self.needsRepaint = false;
            ui.ctx().request_repaint();
        }
    }

    fn doExec(&mut self, config: &Exec ) {
        if let Err(e) = self.audio.exec( &config ) {
            log::error(&e.to_string());
        }
    }

    fn applySetup(&mut self, setup: &str ) {
        let cfg = Config::OrdinaryConfigFromStr( &setup );
        if let Err(e) = self.audio.load_config(&cfg) {
            log::error(&e.to_string());
        }
        /*
        if let Err(e) = self.audio.config(setup, data ) {
            log::error(&e.to_string());
        }
        */
    }
}

static TEST_XXX: &str = r#"
        [workflows]
        start = [ 'start' ]
        stop  = [ 'stop'  ]
        autoexec = [
            'stop',
            { AudioSource = { Sequencer = { Rusty = '1' } } },
            'start',
        ]
        None = [
            'stop',
            { AudioSource = 'None' },
            'start',
        ]
        Simple = [
            'stop',
            { AudioSource = 'Simple' },
            'start',
        ]
        rstString = [
            'stop',
            { AudioSource = { Rusty = '1' } },
            'start',
        ]
        rstPiano = [
            'stop',
            { AudioSource = { Rusty = '0' } },
            'start',
        ]
        sqSimple = [
            'stop',
            { AudioSource = { Sequencer = 'Simple' } },
            'start',
        ]
        sqString = [
            'stop',
            { AudioSource = { Sequencer = { Rusty = '1' } } },
            'start',
        ]
        sqPiano = [
            'stop',
            { AudioSource = { Sequencer = { Rusty = '0' } } },
            'start',
        ]
"#;
