use glib::clone;
use gtk::{Application, ApplicationWindow, Button, ListBox, TextView, gio, glib};
use gtk::{Entry, prelude::*};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
mod parser;

const APP_ID: &str = "tesnileft.ElitePathfinder-Rust";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("epf-r.gresource").expect("Failed to register resources.");
    let application = Application::builder().application_id(APP_ID).build();

    application.connect_activate(build_ui);

    let current_log = File::open("/mnt/gamestorage/SteamLibrary/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/AppData/Local/Frontier Developments/Elite Dangerous/Journal12543831.cache").unwrap();
    let mut current_log_buf = BufReader::new(current_log);

    thread::spawn(move || {
        loop {
            let mut new_contents = String::new();
            current_log_buf.read_to_string(&mut new_contents).unwrap();
        }
    });

    application.run()
}

fn read_newest_log_file() -> Result<String, std::io::Error> {
    let current_log_result = File::open(
        "/mnt/gamestorage/SteamLibrary/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/AppData/Local/Frontier Developments/Elite Dangerous/Journal12543831.cache",
    );
    match current_log_result {
        Ok(file) => {
            let current_log: File = file;
            let mut current_log_buf = BufReader::new(current_log);
            let mut new_contents = String::new();
            current_log_buf.read_to_string(&mut new_contents).unwrap();
            Ok(new_contents)
        }
        Err(error) => Err(error),
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Elite Pathfinder")
        .default_width(350)
        .default_height(70)
        .build();

    let vertical_column1 = ListBox::builder().build();

    let filepathfield = Entry::builder().build();

    vertical_column1.append(&filepathfield);
    let copybutton = Button::builder()
        .label("Button (cool)")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    copybutton.connect_clicked(move |button| button.set_label(&filepathfield.buffer().text()));
    vertical_column1.append(&copybutton);

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create channel that can hold at most 1 message at a time
    let (sender, receiver) = async_channel::bounded(1);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        gio::spawn_blocking(move || {
            // Deactivate the button until the operation is done
            sender
                .send_blocking(false)
                .expect("The channel needs to be open.");

            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
            // Activate the button again
            sender
                .send_blocking(true)
                .expect("The channel needs to be open.");
        });
    });

    let textbox = TextView::builder().editable(false).can_focus(false).build();
    textbox.buffer().set_text("Sample Text");
    vertical_column1.append(&textbox);
    let readbutton = Button::builder()
        .label("Read Latest Log")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    readbutton.connect_clicked(move |_| match read_newest_log_file() {
        Ok(filecontents) => textbox.buffer().set_text(&filecontents),
        Err(err) => textbox.buffer().set_text(&err.to_string()),
    });
    vertical_column1.append(&readbutton);

    // The main loop executes the asynchronous block
    glib::spawn_future_local(clone!(
        #[weak]
        button,
        async move {
            while let Ok(enable_button) = receiver.recv().await {
                button.set_sensitive(enable_button);
            }
        }
    ));

    vertical_column1.append(&button);

    window.set_child(Some(&vertical_column1));
    window.present();
}
