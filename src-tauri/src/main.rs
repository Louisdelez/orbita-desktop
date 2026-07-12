// Orbita Desktop — coquille native (Tauri) qui charge l'application web Orbita (gestion.epicube.ch).
// Version simple et robuste : UNE fenêtre, UNE webview qui charge l'URL distante en plein.
// (L'ancienne tentative de barre de titre custom en multi-webview cassait l'affichage → abandonnée.)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("erreur au lancement d'Orbita Desktop");
}
