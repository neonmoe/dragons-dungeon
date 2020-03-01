use log::{Level, Metadata, Record};

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "{:8}{} {}",
                format!("[{}]", record.level()),
                record
                    .module_path()
                    .map(|s| format!("[{}]", s))
                    .unwrap_or(String::new()),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn user_facing_error(message: &str, window: Option<&sdl2::video::Window>) {
    log::error!("{}", message);
    show_dialog(message, window);
}

fn show_dialog(message: &str, window: Option<&sdl2::video::Window>) {
    use sdl2::messagebox::*;
    show_simple_message_box(MessageBoxFlag::ERROR, crate::TITLE, message, window).ok();
    // This is ok'd because there's nothing we can do about not being
    // able to show dialogs.
}
