use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};
use gtk4 as gtk;

use std::sync::atomic::{AtomicIsize, Ordering};

struct Counter(AtomicIsize);

impl Counter {
    fn new(init: isize) -> Counter {
        Counter(AtomicIsize::new(init))
    }

    fn increment(&self) -> isize {
        let old = self.0.fetch_add(1, Ordering::SeqCst);
        old + 1
    }

    fn decrement(&self) -> isize {
        let old = self.0.fetch_sub(1, Ordering::SeqCst);
        old - 1
    }

    fn get(&self) -> isize {
        self.0.load(Ordering::SeqCst)
    }
}

fn main() {
    let application = Application::new(None, Default::default());
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("First GTK+ Program"));
        window.set_default_size(350, 70);
        let container = Box::new(gtk::Orientation::Vertical, 3);
        use std::sync::Arc;
        let counter = Arc::new(Counter::new(0));
        let label = Label::new(Some(&format!("Starting at {}", counter.get())));
        let inc_btn = Button::with_label("Increment");
        {
            let label_clone = label.clone();
            let counter_clone = counter.clone();
            inc_btn.connect_clicked(move |_| {
                let val = counter_clone.increment();
                label_clone.set_label(&format!("Incremented to {}", val));
            });
        }
        let dec_btn = Button::with_label("Decrement");
        {
            let label_clone = label.clone();
            let counter_clone = counter;
            dec_btn.connect_clicked(move |_| {
                let val = counter_clone.decrement();
                label_clone.set_label(&format!("Decremented to {}", val));
            });
        }
        container.append(&label);
        container.append(&inc_btn);
        container.append(&dec_btn);
        window.set_child(Some(&container));
        window.show();
    });
    application.run();
}
