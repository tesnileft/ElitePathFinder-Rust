mod imp;

use glib::Object;
use gtk::{Application, gio, glib};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::{Cache, SharedCache, UiEvent};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(
        app: &Application,
        receiver: async_channel::Receiver<UiEvent>,
        cache: SharedCache
    ) -> Self {
        // Create new window
        let window: Self = Object::builder().property("application", app).build();
        let window_clone = window.clone();
        // async thread for processing ui events
        glib::spawn_future_local(async move {
            while let Ok(event) = receiver.recv().await {
                window_clone.update_ui(event);
            }
        });
        window
    }
    fn update_ui(&self, event: UiEvent) {
        let imp = self.imp();
        match event {
            UiEvent::SetCurrentSystem { system_name: name } => { imp.topbar.set_systemname(&name)}
            other => {}
        }
    }
}
