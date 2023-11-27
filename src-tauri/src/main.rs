// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rspc::Router;
use std::{borrow::BorrowMut, sync::Arc};

struct OakenState {
    items: Vec<String>,
}

impl OakenState {
    fn new() -> Self {
        OakenState { items: vec![] }
    }

    fn reset(&mut self) {
        self.items.clear()
    }
}

fn router() -> Arc<Router<OakenState>> {
    let router = Router::new()
        .config(rspc::Config::new().export_ts_bindings("../src/bindings.d.ts"))
        .query("me", |t| {
            t(|_ctx: OakenState, name: String| format!("heya, {name}"))
        })
        .query("getItems", |t| t(|ctx: OakenState, _: ()| ctx.items))
        .mutation("addItem", |t| {
            t(|mut ctx, item: String| {
                ctx.items.push(item);
            })
        })
        .mutation("resetItems", |t| t(|mut ctx, _: ()| ctx.items.clear()))
        .build();

    router.arced()
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .plugin(rspc::integrations::tauri::plugin(router(), || {
            OakenState::new()
        }))
        .run(context)
        .expect("error while running application");
}
