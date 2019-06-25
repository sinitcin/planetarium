extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog, ButtonsType, DialogFlags, MessageType, Window, WindowPosition};

pub fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::new(application);
    window.set_title("Planetarium");
    window.set_position(WindowPosition::Center);
    window.set_size_request(800, 600);

    window.set_application(Some(application));
    window.show_all();
}