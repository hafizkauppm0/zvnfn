use rfd::MessageDialog;

pub fn show_message(title: &str, description: &str) {
    MessageDialog::new().set_title(title).set_description(description).show();
}