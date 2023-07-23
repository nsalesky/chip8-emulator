use relm4::gtk::prelude::GtkWindowExt;
use relm4::{gtk, ComponentParts};
use relm4::{ComponentSender, SimpleComponent};

pub struct AppModel {}

#[derive(Debug)]
pub enum AppInput {}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Chip8 Emulator"),
            set_default_width: 400,
            set_default_height: 300,
        }
    }

    fn init(
        _init: Self::Init,
        _root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
