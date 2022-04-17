use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;

use gtk::CompositeTemplate;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "img_btn.ui")]
pub struct ImgBtn {
    #[template_child]
    pub image: TemplateChild<gtk::Image>,
}

#[glib::object_subclass]
impl ObjectSubclass for ImgBtn {
    const NAME: &'static str = "ImgBtn";
    type Type = super::ImgBtn;
    type ParentType = gtk::Button;

    fn class_init(cls: &mut Self::Class) {
        cls.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ImgBtn {}
impl WidgetImpl for ImgBtn {}
impl ButtonImpl for ImgBtn {}
