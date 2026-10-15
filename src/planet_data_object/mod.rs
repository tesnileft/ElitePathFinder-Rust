mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct PlanetDataObject(ObjectSubclass<imp::PlanetDataObject>);
}

///One species line on an exobio card
#[derive(Clone, Default)]
pub struct SpeciesRow {
    pub name: String,
    pub value: u64,
    pub variants: Vec<String>,
    ///Player has scanned (and confirmed) this species
    pub confirmed: bool,
    ///Player ruled this species out / doesn't care about it
    pub absent: bool,
}

impl PlanetDataObject {
    pub fn new(body_name: String, rows: Vec<SpeciesRow>) -> Self {
        let obj: Self = Object::builder()
            .property("body-name", body_name)
            .build();
        *obj.imp().rows.borrow_mut() = rows;
        obj.refresh_summary();
        obj
    }

    pub fn rows(&self) -> Vec<SpeciesRow> {
        self.imp().rows.borrow().clone()
    }

    ///Copies scanned/greyed state from an older card for the same body, so a refresh doesn't lose the player's ticks
    pub fn carry_over_state(&self, old: &PlanetDataObject) {
        let old_rows = old.rows();
        for row in self.imp().rows.borrow_mut().iter_mut() {
            if let Some(prev) = old_rows.iter().find(|r| r.name == row.name) {
                row.confirmed = prev.confirmed;
                row.absent = prev.absent;
            }
        }
        self.refresh_summary();
    }

    pub fn set_row_scanned(&self, index: usize, scanned: bool) {
        if let Some(row) = self.imp().rows.borrow_mut().get_mut(index) {
            row.confirmed = scanned;
        }
        self.refresh_summary();
    }

    pub fn set_row_greyed(&self, index: usize, greyed: bool) {
        if let Some(row) = self.imp().rows.borrow_mut().get_mut(index) {
            row.absent = greyed;
        }
        self.refresh_summary();
    }

    ///Updates the header text. Greyed-out species don't count towards the total
    fn refresh_summary(&self) {
        let (count, total) = {
            let rows = self.imp().rows.borrow();
            let active = rows.iter().filter(|r| !r.absent);
            (active.clone().count(), active.map(|r| r.value).sum::<u64>())
        };
        self.set_property("species-text", format!("{} species", count));
        self.set_property("total-value", format!("up to ¢{}", format_credits(total)));
    }
}

pub fn format_credits(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[derive(Default)]
pub struct PlanetData {
    pub body_name: String,
    pub species_text: String,
    pub total_value: String,
}