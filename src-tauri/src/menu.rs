use tauri::{Context, utils::assets::EmbeddedAssets, Menu, Submenu, AboutMetadata, MenuItem, CustomMenuItem};

pub fn build_menu(ctx: &Context<EmbeddedAssets>) -> Menu {
    let mut menu = Menu::new();
    let mut meta = AboutMetadata::default();

    meta.version = Some(ctx.package_info().version.to_string());

    if cfg!(target_os = "macos") {
        menu = menu.add_submenu(Submenu::new(
            "Oaken",
            Menu::new()
                .add_native_item(MenuItem::About("Oaken".to_string(), meta))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    menu = menu.add_submenu(Submenu::new("File", {
        let mut menu = Menu::new();

        menu = menu.add_item(CustomMenuItem::new("create_config", "Create Config File"));
        menu = menu.add_item(CustomMenuItem::new("load_config", "Load Config File"));

        menu = menu.add_native_item(MenuItem::Separator);

        menu = menu.add_native_item(MenuItem::Quit);

        menu
    }));

    menu = menu.add_submenu(Submenu::new("Edit", {
        let mut menu = Menu::new();
        menu = menu.add_native_item(MenuItem::Undo);
        menu = menu.add_native_item(MenuItem::Redo);
        menu = menu.add_native_item(MenuItem::Separator);
        menu = menu.add_native_item(MenuItem::Cut);
        menu = menu.add_native_item(MenuItem::Copy);
        menu = menu.add_native_item(MenuItem::Paste);

        if cfg!(not(target_os = "macos")) {
            menu = menu.add_native_item(MenuItem::Separator)
        }

        menu = menu.add_native_item(MenuItem::SelectAll);

        menu
    }));

    menu = menu.add_submenu(Submenu::new("Window", {
        let mut menu = Menu::new();

        menu = menu.add_native_item(MenuItem::Minimize);
        menu = menu.add_native_item(MenuItem::Zoom);

        if cfg!(not(target_os = "macos")) {
            menu = menu.add_native_item(MenuItem::CloseWindow);
        }

        menu
    }));

    menu
}