use arboard::Clipboard;

pub fn copy_to_clipboard(value: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|error| error.to_string())?;

    clipboard
        .set_text(value.to_string())
        .map_err(|error| error.to_string())
}