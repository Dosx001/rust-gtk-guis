use gtk::{prelude::*, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::{Application, ApplicationWindow, Box, Label};
use gtk4 as gtk;
use rand::Rng;

mod img_btn;
use img_btn::ImgBtn;

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
    application.connect_startup(|app| {
        let css = gtk::CssProvider::new();
        css.load_from_data(include_bytes!("style.css"));
        gtk::StyleContext::add_provider_for_display(
            &gtk::gdk::Display::default().expect(""),
            &css,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        build_ui(app);
    });
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Rock, Paper, Scissors: Pokemom Ver.")
        .default_width(70)
        .default_height(350)
        .build();
    let pics = [
        "src/rps/assets/Bulbasaur.png",
        "src/rps/assets/Charmander.png",
        "src/rps/assets/Squirtle.png",
    ];
    let pkm = gtk::Image::new();
    pkm.set_pixel_size(100);
    let wins = Arc::new(Counter::new(0));
    let loses = Arc::new(Counter::new(0));
    let ties = Arc::new(Counter::new(0));
    let lab1 = Label::builder()
        .label("Wins: 0")
        .css_name("wins")
        .build();
    let lab2 = Label::builder()
        .label("Loses: 0")
        .css_name("loses")
        .build();
    let lab3 = Label::builder()
        .label("Ties: 0")
        .css_name("ties")
        .build();
    let hbox2 = Box::new(gtk::Orientation::Horizontal, 3);
    hbox2.append(&lab1);
    hbox2.append(&lab2);
    hbox2.append(&lab3);
    let btn1 = ImgBtn::new();
    {
        let lab1c = lab1.clone();
        let lab2c = lab2.clone();
        let lab3c = lab3.clone();
        let winsc = wins.clone();
        let losesc = loses.clone();
        let tiesc = ties.clone();
        let pkmc = pkm.clone();
        btn1.connect_clicked(move |_| {
            let (res, pick) = result(0);
            pkmc.set_from_file(Some(pics[pick]));
            match res {
                0 => lab1c.set_label(&format!("Wins: {}", winsc.increment())),
                1 => lab2c.set_label(&format!("Loses: {}", losesc.increment())),
                _ => lab3c.set_label(&format!("Ties: {}", tiesc.increment())),
            }
        });
    }
    btn1.set_image("src/rps/assets/BulbasaurBack.png");
    let btn2 = ImgBtn::new();
    {
        let lab1c = lab1.clone();
        let lab2c = lab2.clone();
        let lab3c = lab3.clone();
        let winsc = wins.clone();
        let losesc = loses.clone();
        let tiesc = ties.clone();
        let pkmc = pkm.clone();
        btn2.connect_clicked(move |_| {
            let (res, pick) = result(1);
            pkmc.set_from_file(Some(pics[pick]));
            match res {
                0 => lab1c.set_label(&format!("Wins: {}", winsc.increment())),
                1 => lab2c.set_label(&format!("Loses: {}", losesc.increment())),
                _ => lab3c.set_label(&format!("Ties: {}", tiesc.increment())),
            }
        });
    }
    btn2.set_image("src/rps/assets/CharmanderBack.png");
    let btn3 = ImgBtn::new();
    {
        let pkmc = pkm.clone();
        btn3.connect_clicked(move |_| {
            let (res, pick) = result(2);
            pkmc.set_from_file(Some(pics[pick]));
            match res {
                0 => lab1.set_label(&format!("Wins: {}", wins.increment())),
                1 => lab2.set_label(&format!("Loses: {}", loses.increment())),
                _ => lab3.set_label(&format!("Ties: {}", ties.increment())),
            }
        });
    }
    btn3.set_image("src/rps/assets/SquirtleBack.png");
    let hbox1 = Box::new(gtk::Orientation::Horizontal, 3);
    hbox1.append(&btn1);
    hbox1.append(&btn2);
    hbox1.append(&btn3);
    let red = gtk::Image::builder()
        .file("src/rps/assets/Red.png")
        .pixel_size(150)
        .halign(gtk::Align::End)
        .build();
    let vbox = Box::new(gtk::Orientation::Vertical, 3);
    vbox.append(&red);
    vbox.append(&pkm);
    vbox.append(&hbox2);
    vbox.append(&hbox1);
    window.set_child(Some(&vbox));
    application.connect_activate(move |_| {
        window.show();
    });
}

fn result(ans: i32) -> (i32, usize) {
    // 0 => rock; 1 => paper; 2 => scissors
    // 0 => win; 1 => lose; 2 => tie
    let pick = rand::thread_rng().gen_range(0..3);
    match pick {
        0 => {
            if ans == 1 {
                return (0, pick);
            }
            if ans == 2 {
                return (1, pick);
            }
        }
        1 => {
            if ans == 2 {
                return (0, pick);
            }
            if ans == 0 {
                return (1, pick);
            }
        }
        _ => {
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
