use gtk::{Application, ApplicationWindow, Button, ListBox, glib};
use gtk::{Entry, prelude::*};
mod parser;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(build_ui);

    application.run()
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

    let vertical_column1 = ListBox::builder().build();

    let filepathfield = Entry::builder().build();

    vertical_column1.append(&filepathfield);
    let button = Button::builder()
        .label("Button (cool)")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    button.connect_clicked(move |button| button.set_label(&filepathfield.buffer().text()));
    vertical_column1.append(&button);
    window.set_child(Some(&vertical_column1));
    window.present();
}
