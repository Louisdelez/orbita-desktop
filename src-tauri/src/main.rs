// Orbita Desktop — coquille native (Tauri) qui charge l'application web Orbita (gestion.epicube.ch).
// L'app web reste la seule source de vérité ; ce binaire n'est qu'une fenêtre native qui l'affiche.
// La fenêtre + son URL distante sont déclarées dans tauri.conf.json (app.windows[0].url).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("erreur au lancement d'Orbita Desktop");
}
