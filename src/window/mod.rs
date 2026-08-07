mod imp;

use gio::ListStore;
use glib::Object;
use gtk::{Application, gio, glib, NoSelection, SignalListItemFactory, ListItem};
use gtk::prelude::{Cast, CastNone, ListItemExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::{Cache, SharedCache, UiEvent};
use crate::exobio_card::ExobioCard;
use crate::planet_data_object::PlanetDataObject;

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
    fn exobio_cards(&self) -> gio::ListStore {
        self.imp()
            .exobio_cards
            .borrow()
            .clone()
            .expect("Could not obtain exobio cards")
    }
    fn new_planet(){

    }
    fn setup_exobio(&self){
        let model = ListStore::new::<PlanetDataObject>();
        self.imp().exobio_cards.replace(Some(model));
        let selection_model = NoSelection::new(Some(self.exobio_cards()));
        self.imp().exobiology_list_view.set_model(Some(&selection_model));
    }
    fn setup_exobio_factory(&self) {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let planet_card = ExobioCard::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&planet_card));

        });
        factory.connect_bind(move |_, list_item|
            {
                let planet_object = list_item
                    .downcast_ref::<ListItem>()
                    .expect("Needs to be ListItem")
                    .item()
                    .and_downcast::<PlanetDataObject>()
                    .expect("The item has to be an `PlanetDataObject`.");
                let exobio_card = list_item
                    .downcast_ref::<ListItem>()
                    .expect("Needs to be ExobioCard")
                    .child()
                    .and_downcast::<ExobioCard>()
                    .expect("The item has to be an `ExobioCard`.");
                exobio_card.bind(&planet_object);
            });
        factory.connect_unbind(move |_, list_item| {
            let exobio_card = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<ExobioCard>()
                .expect("The item has to be an `ExobioCard`.");
            exobio_card.unbind();
        });
        self.imp().exobiology_list_view.set_factory(Some(&factory));

    }
    fn setup_callbacks(&self){

    }
}
