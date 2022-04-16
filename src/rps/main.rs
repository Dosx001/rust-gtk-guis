use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};
use gtk4 as gtk;
use rand::Rng;

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

fn result(ans: i32) -> (i32, &'static str) {
    // 0 => rock; 1 => paper; 2 => scissors
    // 0 => win; 1 => lose; 2 => tie
    let pick: &str;
    match rand::thread_rng().gen_range(0..2) {
        0 => {
            pick = "rock";
            if ans == 1 {
                return (0, pick);
            }
            if ans == 2 {
                return (1, pick);
            }
        }
        1 => {
            pick = "paper";
            if ans == 2 {
                return (0, pick);
            }
            if ans == 0 {
                return (1, pick);
            }
        }
        _ => {
            pick = "scissors";
            if ans == 0 {
                return (0, pick);
            }
            if ans == 1 {
                return (1, pick);
            }
        }
    }
    (2, pick)
}

fn main() {
    let application = Application::new(None, Default::default());
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Rock, Paper, Scissors"));
        window.set_default_size(350, 70);
        let ans_lab = Label::new(Some(""));
        let ans_box = Box::new(gtk::Orientation::Horizontal, 0);
        ans_box.append(&ans_lab);
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
            let lab1c = lab1.clone();
            let lab2c = lab2.clone();
            let lab3c = lab3.clone();
            let winsc = wins.clone();
            let losesc = loses.clone();
            let tiesc = ties.clone();
            let ans_labc = ans_lab.clone();
            btn1.connect_clicked(move |_| {
                let (res, pick) = result(0);
                ans_labc.set_label(pick);
                match res {
                    0 => lab1c.set_label(&format!("Wins: {}", winsc.increment())),
                    1 => lab2c.set_label(&format!("Loses: {}", losesc.increment())),
                    _ => lab3c.set_label(&format!("Ties: {}", tiesc.increment())),
                }
            });
        }
        let btn2 = Button::with_label("Paper");
        {
            let lab1c = lab1.clone();
            let lab2c = lab2.clone();
            let lab3c = lab3.clone();
            let winsc = wins.clone();
            let losesc = loses.clone();
            let tiesc = ties.clone();
            let ans_labc = ans_lab.clone();
            btn2.connect_clicked(move |_| {
                let (res, pick) = result(1);
                ans_labc.set_label(pick);
                match res {
                    0 => lab1c.set_label(&format!("Wins: {}", winsc.increment())),
                    1 => lab2c.set_label(&format!("Loses: {}", losesc.increment())),
                    _ => lab3c.set_label(&format!("Ties: {}", tiesc.increment())),
                }
            });
        }
        let btn3 = Button::with_label("Scissors");
        {
            btn3.connect_clicked(move |_| {
                let (res, pick) = result(2);
                ans_lab.set_label(pick);
                match res {
                    0 => lab1.set_label(&format!("Wins: {}", wins.increment())),
                    1 => lab2.set_label(&format!("Loses: {}", loses.increment())),
                    _ => lab3.set_label(&format!("Ties: {}", ties.increment())),
                }
            });
        }
        let hbox1 = Box::new(gtk::Orientation::Horizontal, 3);
        hbox1.append(&btn1);
        hbox1.append(&btn2);
        hbox1.append(&btn3);
        let vbox = Box::new(gtk::Orientation::Vertical, 3);
        vbox.append(&ans_box);
        vbox.append(&hbox2);
        vbox.append(&hbox1);
        window.set_child(Some(&vbox));
        window.show();
    });
    application.run();
}
