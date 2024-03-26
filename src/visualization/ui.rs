use::std::cell::Cell;
use::std::rc::Rc;

use gtk::prelude::*;
use gtk::glib::{self, clone};
use gtk::{Application, ApplicationWindow, Button, Entry, Label,PopoverMenu, PopoverMenuBar};

/// Function to initialize the user interface
pub fn build_ui(app: &Application) {
    let outer_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(2)
        .margin_bottom(2)
        .margin_start(2)
        .margin_end(2)
        .build();

    let button_box = build_test_ui_box();
    outer_box.append(&button_box);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Test app")
        .child(&outer_box)
        .build();

    window.present();
}

fn build_test_ui_box() -> gtk::Box {
    let counter = Rc::new(Cell::new(0));
    
    let inc_button = Button::builder()
        .label("Increase number")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let dec_button = Button::builder()
        .label("Decrease number")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let counter_text = gtk::Label::builder()
        .label("Counter: 0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    dec_button.connect_clicked(clone!(@strong counter, @weak counter_text => move |_| {
        counter.set(counter.get() - 1);
        counter_text.set_text(format!("Counter: {}", counter.get()).as_str());
    }));

    inc_button.connect_clicked(clone!(@weak counter_text => move |_| {
        counter.set(counter.get() + 1);
        counter_text.set_text(format!("Counter: {}", counter.get()).as_str());
    }));

    let button_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    button_box.append(&inc_button);
    button_box.append(&dec_button);
    button_box.append(&counter_text);

    button_box
}

fn build_waveform_ui_box() -> gtk::Box {
    let waveform_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(2)
        .margin_bottom(2)
        .margin_start(2)
        .margin_end(2)
        .build();

    waveform_box
}

fn build_menubar() -> PopoverMenuBar {
    let menubar = PopoverMenuBar::builder()
        .build();

    menubar
}