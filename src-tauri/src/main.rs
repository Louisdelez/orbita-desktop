// Orbita Desktop — coquille native (Tauri) qui charge l'app web Orbita (gestion.epicube.ch).
// Fenêtre SANS décorations système + barre de titre custom INJECTÉE dans la page via un script d'init
// (une seule webview → affichage robuste, pas de multi-webview). Le script réserve 38px en haut (#root
// padding-top) et pose une barre fixe avec réduire/agrandir/fermer + déplacement. Il ne s'exécute que
// dans cette app desktop : l'app web dans un navigateur n'est pas modifiée.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

const TITLEBAR_JS: &str = r#"
(function () {
  function mount() {
    if (!document.body || document.getElementById('__ep_tb')) return;
    var css = document.createElement('style');
    css.id = '__ep_tb_css';
    css.textContent =
      '#__ep_tb{position:fixed;top:0;left:0;right:0;height:38px;z-index:2147483647;display:flex;align-items:center;'
      + 'justify-content:space-between;background:linear-gradient(180deg,#141a4a,#0a0d2a);'
      + 'border-bottom:1px solid rgba(255,255,255,.08);font-family:Inter,system-ui,sans-serif;'
      + '-webkit-user-select:none;user-select:none;padding:0 4px 0 14px;box-sizing:border-box}'
      + '#__ep_tb .b{font-weight:800;letter-spacing:.06em;font-size:13px;color:#eef1ff;display:flex;align-items:center;gap:7px;height:100%}'
      + '#__ep_tb .b i{color:#7f5af0;font-style:normal}'
      + '#__ep_tb .b span{background:linear-gradient(90deg,#5865f2,#7f5af0);-webkit-background-clip:text;background-clip:text;color:transparent}'
      + '#__ep_tb .drag{flex:1;height:100%}'
      + '#__ep_tb .c{display:flex;gap:2px}'
      + '#__ep_tb button{width:40px;height:28px;border:0;border-radius:7px;background:transparent;color:#97a0d0;cursor:pointer;display:grid;place-items:center}'
      + '#__ep_tb button:hover{background:rgba(255,255,255,.09);color:#fff}'
      + '#__ep_tb button.x:hover{background:#e13b4c;color:#fff}'
      + '#__ep_tb svg{width:15px;height:15px}'
      + '#root{height:100%!important;box-sizing:border-box!important;padding-top:38px!important}';
    document.head.appendChild(css);

    var bar = document.createElement('div');
    bar.id = '__ep_tb';
    bar.innerHTML =
      '<div class="b" data-drag><i>&#129680;</i> EPICUBE <span>ORBITA</span></div>'
      + '<div class="drag" data-drag></div>'
      + '<div class="c">'
      + '<button id="__ep_min" title="Reduire"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="5" y1="12" x2="19" y2="12"/></svg></button>'
      + '<button id="__ep_max" title="Agrandir"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="5" y="5" width="14" height="14" rx="2"/></svg></button>'
      + '<button id="__ep_close" class="x" title="Fermer"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="6" y1="6" x2="18" y2="18"/><line x1="18" y1="6" x2="6" y2="18"/></svg></button>'
      + '</div>';
    document.body.appendChild(bar);

    var T = window.__TAURI__;
    if (T && T.window && T.window.getCurrentWindow) {
      var w = T.window.getCurrentWindow();
      document.getElementById('__ep_min').onclick = function () { w.minimize(); };
      document.getElementById('__ep_max').onclick = function () { w.toggleMaximize(); };
      document.getElementById('__ep_close').onclick = function () { w.close(); };
      bar.querySelectorAll('[data-drag]').forEach(function (el) {
        el.addEventListener('mousedown', function (e) { if (e.buttons === 1) { w.startDragging(); } });
        el.addEventListener('dblclick', function () { w.toggleMaximize(); });
      });
    }
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', mount);
  else mount();
  // L'app est une SPA : si un re-render enlève la barre, on la remet.
  new MutationObserver(function () { mount(); }).observe(document.documentElement, { childList: true, subtree: true });
})();
"#;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://gestion.epicube.ch".parse().unwrap()),
            )
            .title("Orbita")
            .inner_size(1200.0, 800.0)
            .min_inner_size(480.0, 600.0)
            .decorations(false)
            .center()
            .initialization_script(TITLEBAR_JS)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erreur au lancement d'Orbita Desktop");
}
