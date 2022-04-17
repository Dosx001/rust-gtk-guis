mod imp;

use gtk::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;

glib::wrapper! {
    pub struct ImgBtn(ObjectSubclass<imp::ImgBtn>)
        @extends gtk::Widget, gtk::Button;
}

impl Default for ImgBtn {
    fn default() -> Self {
        Self::new()
    }
}

impl ImgBtn {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ImgBtn")
    }

    pub fn set_image(&self, path: &str) {
        let imp = self.imp();
        imp.image.set_from_file(Some(path));
    }
}
