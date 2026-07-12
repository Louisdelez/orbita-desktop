// Orbita Desktop — coquille native (Tauri) qui charge l'application web Orbita (gestion.epicube.ch).
// La fenêtre n'a PAS de décorations système : on compose une barre de titre custom (webview locale
// "titlebar") au-dessus du contenu web distant (webview "content"). Les deux webviews cohabitent dans
// une seule fenêtre (multi-webview Tauri) ; le contenu distant reste une navigation top-level → cookies,
// API same-origin et PWA de gestion.epicube.ch intacts.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, Manager, WebviewUrl, WindowEvent};
use tauri::webview::WebviewBuilder;
use tauri::window::WindowBuilder;

const BAR_H: f64 = 38.0;              // hauteur de la barre de titre custom (px logiques)
const W: f64 = 1200.0;                // taille initiale de la fenêtre
const H: f64 = 800.0;
const CONTENT_URL: &str = "https://gestion.epicube.ch";

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = WindowBuilder::new(app, "main")
                .title("Orbita")
                .inner_size(W, H)
                .min_inner_size(480.0, 600.0)
                .decorations(false)   // pas de barre système : on met la nôtre
                .center()
                .build()?;

            // Barre de titre custom (page locale).
            window.add_child(
                WebviewBuilder::new("titlebar", WebviewUrl::App("titlebar.html".into())),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(W, BAR_H),
            )?;

            // Contenu = l'app web Orbita (navigation top-level → même origine que dans un navigateur).
            window.add_child(
                WebviewBuilder::new("content", WebviewUrl::External(CONTENT_URL.parse().unwrap())),
                LogicalPosition::new(0.0, BAR_H),
                LogicalSize::new(W, H - BAR_H),
            )?;

            // Repositionne/redimensionne les webviews quand la fenêtre change de taille.
            let win = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::Resized(size) = event {
                    let scale = win.scale_factor().unwrap_or(1.0);
                    let sz = size.to_logical::<f64>(scale);
                    if let Some(tb) = win.get_webview("titlebar") {
                        let _ = tb.set_size(LogicalSize::new(sz.width, BAR_H));
                    }
                    if let Some(ct) = win.get_webview("content") {
                        let _ = ct.set_position(LogicalPosition::new(0.0, BAR_H));
                        let _ = ct.set_size(LogicalSize::new(sz.width, (sz.height - BAR_H).max(1.0)));
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erreur au lancement d'Orbita Desktop");
}
