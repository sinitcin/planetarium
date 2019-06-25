extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog, ButtonsType, DialogFlags, MessageType, Window};


mod tycho2;
use tycho2::*;

mod ui;
use ui::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let application = gtk::Application::new(
        Some("com.github.planetarium"), 
        Default::default(),
    ).expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());                       
}
