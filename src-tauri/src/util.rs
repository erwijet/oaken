use serde::de::Error;
use tauri::api::dialog;

pub trait Capitalize {
    fn capitalize(self) -> String;
}

impl<S: Into<String>> Capitalize for S {
    fn capitalize(self) -> String {
        let s: String = self.into();
        let mut c = s.chars();

        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

//

pub fn map_err_into_dialog<E: Error>(err: E) {
    dialog::MessageDialogBuilder::new("Error", err.to_string()).show(|_| {});
}

pub trait PresentError<T> {
    fn present_err(self) -> Result<T, ()>;
}

impl<T, E: Error> PresentError<T> for Result<T, E> {
    fn present_err(self) -> Result<T, ()> {
        self.map_err(map_err_into_dialog)
    }
}
