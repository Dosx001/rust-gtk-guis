use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label,};

use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

struct Counter(AtomicIsize);

impl Counter {
    fn new(init: isize) -> Counter {
        Counter(AtomicIsize::new(init))
    }

    fn increment(&self) -> isize {
        let old = self.0.fetch_add(1, Ordering::SeqCst);
        old + 1
    }

    fn get(&self) -> isize {
        self.0.load(Ordering::SeqCst)
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
        let lab1 = Label::new(Some(&format!("Wins: {}", wins.get())));
        let lab2 = Label::new(Some(&format!("Loses: {}", loses.get())));
        let lab3 = Label::new(Some(&format!("Ties: {}", ties.get())));
        let hbox2 = Box::new(gtk::Orientation::Horizontal, 3);
        hbox2.append(&lab1);
        hbox2.append(&lab2);
        hbox2.append(&lab3);
        let btn1 = Button::with_label("Rock");
        {
            let label_clone = lab1;
            let counter_clone = wins;
            btn1.connect_clicked(move |_| {
                let val = counter_clone.increment();
                label_clone.set_label(&format!("Wins: {}", val));
            });
        }
        let btn2 = Button::with_label("Paper");
        {
            let label_clone = lab2;
            let counter_clone = ties;
            btn2.connect_clicked(move |_| {
                let val = counter_clone.increment();
                label_clone.set_label(&format!("Wins: {}", val));
            });
        }
        let btn3 = Button::with_label("Scissors");
        {
            let label_clone = lab3;
            let counter_clone = loses;
            btn3.connect_clicked(move |_| {
                let val = counter_clone.increment();
                label_clone.set_label(&format!("Wins: {}", val));
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
