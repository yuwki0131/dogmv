use crate::models::FileItem;
use crate::ui::preview::display_markdown;
use gtk4::prelude::*;
use gtk4::{gio, Box as GtkBox, Label, ListView, Orientation, ScrolledWindow, SignalListItemFactory, SingleSelection, TreeListModel, TreeListRow};
use log::{info, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use webkit6::WebView;

/// Creates a tree view for browsing directory structure
pub fn create_tree_view(root_dir: &Path) -> (ScrolledWindow, SingleSelection) {
    let scroll = ScrolledWindow::new();
    scroll.set_vexpand(true);
    scroll.set_hexpand(true);

    // Load root directory items
    let root_items = load_directory_items(root_dir);

    // Create root model
    let root_model = gio::ListStore::new::<FileItem>();
    for item in root_items {
        root_model.append(&item);
    }

    // Create TreeListModel with expand function
    let tree_model = TreeListModel::new(
        root_model.clone(),
        false, // passthrough
        false,  // don't autoexpand
        |item| {
            let file_item = item.downcast_ref::<FileItem>()?;
            if file_item.is_dir() && !file_item.is_symlink() {
                let path = file_item.path_buf();
                let children = load_directory_items(&path);
                if children.is_empty() {
                    return None;
                }
                let child_model = gio::ListStore::new::<FileItem>();
                for child in children {
                    child_model.append(&child);
                }
                Some(child_model.upcast())
            } else {
                None
            }
        },
    );

    // Create selection model
    let selection_model = SingleSelection::new(Some(tree_model.clone()));

    // Create ListView
    let list_view = ListView::new(Some(selection_model.clone()), None::<SignalListItemFactory>);

    // Create factory for rendering list items
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let row = GtkBox::new(Orientation::Horizontal, 6);
        row.set_margin_start(6);
        row.set_margin_end(6);
        row.set_margin_top(3);
        row.set_margin_bottom(3);

        let icon = Label::new(None);
        let label = Label::new(None);
        label.set_xalign(0.0);
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

        row.append(&icon);
        row.append(&label);

        list_item.set_child(Some(&row));
    });

    factory.connect_bind(move |_, list_item| {
        let tree_list_row = list_item
            .item()
            .and_downcast::<TreeListRow>()
            .expect("Item must be TreeListRow");

        let file_item = tree_list_row
            .item()
            .and_downcast::<FileItem>()
            .expect("TreeListRow item must be FileItem");

        let row_widget = list_item.child().and_downcast::<GtkBox>().unwrap();
        let icon_label = row_widget.first_child().unwrap().downcast::<Label>().unwrap();
        let name_label = icon_label.next_sibling().unwrap().downcast::<Label>().unwrap();

        // Set indentation based on depth
        let depth = tree_list_row.depth();
        row_widget.set_margin_start((depth * 16 + 6) as i32);

        // Set icon based on file type
        let icon_text = if file_item.is_symlink() {
            "🔗" // Symlink
        } else if file_item.is_dir() {
            if tree_list_row.is_expanded() {
                "📂" // Open folder
            } else {
                "📁" // Closed folder
            }
        } else {
            "📄" // File
        };
        icon_label.set_text(icon_text);

        // Set file name with tooltip
        let name = file_item.name();
        name_label.set_text(&name);
        name_label.set_tooltip_text(Some(&name));

        // Add (symlink) suffix for symlinks
        if file_item.is_symlink() {
            name_label.set_text(&format!("{} (symlink)", name));
        }
    });

    list_view.set_factory(Some(&factory));

    scroll.set_child(Some(&list_view));
    (scroll, selection_model)
}

/// Load directory items and sort them (directories first, then alphabetically)
pub fn load_directory_items(dir_path: &Path) -> Vec<FileItem> {
    let mut items = Vec::new();

    let read_dir = match fs::read_dir(dir_path) {
        Ok(rd) => rd,
        Err(e) => {
            warn!("Failed to read directory {}: {}", dir_path.display(), e);
            return items;
        }
    };

    for entry in read_dir.flatten() {
        let path = entry.path();

        // Skip hidden files (starting with .)
        if let Some(name) = path.file_name() {
            if name.to_string_lossy().starts_with('.') {
                continue;
            }
        }

        if let Some(item) = FileItem::new(&path) {
            items.push(item);
        }
    }

    // Sort: directories first, then alphabetically by name
    items.sort_by(|a, b| {
        match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name().to_lowercase().cmp(&b.name().to_lowercase()),
        }
    });

    items
}

/// Setup file selection handler for tree view
pub fn setup_file_selection_handler(
    selection_model: &SingleSelection,
    webview: &WebView,
    current_file: Arc<Mutex<Option<PathBuf>>>,
) {
    let webview = webview.clone();
    let current_file = current_file.clone();

    selection_model.connect_selected_item_notify(move |model| {
        if let Some(selected_item) = model.selected_item() {
            if let Some(tree_list_row) = selected_item.downcast_ref::<TreeListRow>() {
                if let Some(file_item) = tree_list_row.item().and_downcast::<FileItem>() {
                    let path = file_item.path_buf();

                    // Only open files, not directories or symlinks
                    if !file_item.is_dir() && !file_item.is_symlink() {
                        info!("File selected: {}", path.display());

                        // Update current file in app state
                        if let Ok(mut current) = current_file.lock() {
                            *current = Some(path.clone());
                        }

                        // Display markdown
                        display_markdown(&webview, &path);
                    } else if file_item.is_dir() && !file_item.is_symlink() {
                        // Toggle directory expansion
                        tree_list_row.set_expanded(!tree_list_row.is_expanded());
                    }
                }
            }
        }
    });
}
