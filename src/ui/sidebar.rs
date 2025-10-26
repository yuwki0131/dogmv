use gtk4::prelude::*;
use gtk4::{gdk, Button, Paned, ScrolledWindow};
use std::sync::{Arc, Mutex};

/// Setup CSS for toggle button to remove borders completely
pub fn setup_toggle_button_css() {
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        r#"
        .flat-toggle {
            border: none;
            background: none;
            box-shadow: none;
            padding: 4px;
        }
        .flat-toggle:hover {
            border: none;
            background: rgba(255, 255, 255, 0.1);
            box-shadow: none;
        }
        .flat-toggle:active {
            border: none;
            background: rgba(255, 255, 255, 0.2);
            box-shadow: none;
        }
        "#
    );

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Setup toggle button for sidebar visibility
pub fn setup_toggle_button(
    toggle_button: &Button,
    tree_scroll: &ScrolledWindow,
    paned: &Paned,
) {
    let tree_scroll = tree_scroll.clone();
    let toggle_button = toggle_button.clone();
    let paned = paned.clone();

    // Store the original width when sidebar is open
    let original_width = Arc::new(Mutex::new(250)); // Default initial width

    toggle_button.connect_clicked(move |btn| {
        let is_visible = tree_scroll.is_visible();

        if is_visible {
            // Closing sidebar - store current width and minimize
            let current_pos = paned.position();
            if let Ok(mut width) = original_width.lock() {
                *width = current_pos;
            }

            // Set to minimum width (just enough for toggle button, approximately 40-50px)
            paned.set_position(40);
            tree_scroll.set_visible(false);

            btn.set_icon_name("pan-end-symbolic");
            btn.set_tooltip_text(Some("サイドバー展開"));
        } else {
            // Opening sidebar - restore original width
            if let Ok(width) = original_width.lock() {
                paned.set_position(*width);
            }
            tree_scroll.set_visible(true);

            btn.set_icon_name("pan-start-symbolic");
            btn.set_tooltip_text(Some("サイドバー閉じる"));
        }
    });
}
