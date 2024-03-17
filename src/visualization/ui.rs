use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

/// Function to initialize the user interface
pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Test app")
        .build();

    window.present();
}