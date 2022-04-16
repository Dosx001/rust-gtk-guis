use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};
use gtk4 as gtk;

use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

struct Counter(AtomicIsize);

impl Counter {
    fn new(init: isize) -> Counter {
        Counter(AtomicIsize::new(init))
    }

    fn increment(&self) -> isize {
        let value = self.0.fetch_add(1, Ordering::SeqCst);
        value + 1
    }
}

fn main() {
    let application = Application::new(None, Default::default());
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Rock, Paper, Scissors"));
        window.set_default_size(350, 70);
        let wins = Arc::new(Counter::new(0));
        let loses = Arc::new(Counter::new(0));
        let ties = Arc::new(Counter::new(0));
        let lab1 = Label::new(Some("Wins: 0"));
        let lab2 = Label::new(Some("Loses: 0"));
        let lab3 = Label::new(Some("Ties: 0"));
        let hbox2 = Box::new(gtk::Orientation::Horizontal, 3);
        hbox2.append(&lab1);
        hbox2.append(&lab2);
        hbox2.append(&lab3);
        let btn1 = Button::with_label("Rock");
        {
            btn1.connect_clicked(move |_| {
                lab1.set_label(&format!("Wins: {}", wins.increment()));
            });
        }
        let btn2 = Button::with_label("Paper");
        {
            btn2.connect_clicked(move |_| {
                lab2.set_label(&format!("Loses: {}", ties.increment()));
            });
        }
        let btn3 = Button::with_label("Scissors");
        {
            btn3.connect_clicked(move |_| {
                lab3.set_label(&format!("Ties: {}", loses.increment()));
            });
        }
        let hbox1 = Box::new(gtk::Orientation::Horizontal, 3);
        hbox1.append(&btn1);
        hbox1.append(&btn2);
        hbox1.append(&btn3);
        let vbox = Box::new(gtk::Orientation::Vertical, 3);
        vbox.append(&hbox2);
        vbox.append(&hbox1);
        window.set_child(Some(&vbox));
        window.show();
    });
    application.run();
}
