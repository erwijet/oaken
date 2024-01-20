use std::ops::Deref;

use tauri::api::dialog;

use crate::shared::pool::get_pool;

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

pub fn map_err_into_dialog<E: std::error::Error>(err: E) {
    dialog::MessageDialogBuilder::new("Error", err.to_string()).show(|_| {});
}

pub trait PresentError<T> {
    fn present_err(self) -> Result<T, ()>;
}

impl<T, E: std::error::Error> PresentError<T> for Result<T, E> {
    fn present_err(self) -> Result<T, ()> {
        self.map_err(map_err_into_dialog)
    }
}

#[derive(sqlx::FromRow)]
pub struct LastInsertRowId {
    #[sqlx(rename = "last_insert_rowid()")]
    pub id: i32,
}

impl Deref for LastInsertRowId {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl LastInsertRowId {
    pub async fn get() -> Self {
        let pool = get_pool();
        sqlx::query_as("SELECT last_insert_rowid();")
            .fetch_one(pool.deref())
            .await
            .unwrap()
    }
}

#[macro_export]
macro_rules! sql_args {
    [$($item:expr),+ $(,)?] => {{
        use ::sqlx::Arguments;
        let mut args = ::sqlx::sqlite::SqliteArguments::default();
        $(args.add($item);)+

        args
    }}
}

#[macro_export]
macro_rules! inline_async {
    ($($expr:expr);+ $(;)?) => {
        ::tokio::task::block_in_place(|| {
            ::tauri::async_runtime::block_on(async {
                $($expr);*;
            })
        })
    };
}
