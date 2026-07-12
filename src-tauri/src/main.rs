// Orbita Desktop — coquille native (Tauri) qui charge l'app web Orbita (gestion.epicube.ch).
// Fenêtre SANS décorations système + barre de fenêtre custom INJECTÉE dans la page (webview unique →
// affichage robuste). Intégration PROPRE : une barre fine (32px) avec seulement les boutons fenêtre
// (pas de branding en double avec la topbar de l'app), et surtout `#root` devient le BLOC CONTENEUR de
// tous les `position:fixed` de l'app (via transform) → toute l'app (topbar + panneaux fixes + fonds)
// se décale de 32px de façon cohérente, alignée, sans chevauchement ni coupure. Ne tourne QUE dans
// l'app desktop : l'app web dans un navigateur n'est pas modifiée.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

const TITLEBAR_JS: &str = r#"
(function () {
  var H = 32; // hauteur de la barre de fenêtre (px)
  var SVG_MIN = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="5" y1="12" x2="19" y2="12"/></svg>';
  var SVG_MAX = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="5" y="5" width="14" height="14" rx="2"/></svg>';
  var SVG_X   = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="6" y1="6" x2="18" y2="18"/><line x1="18" y1="6" x2="6" y2="18"/></svg>';

  function ensureStyle() {
    if (document.getElementById('__ep_style')) return;
    var st = document.createElement('style');
    st.id = '__ep_style';
    st.textContent =
      // barre de fenêtre : fine, couleur du fond de l'app, boutons à droite, reste = zone de déplacement
      '#__ep_tb{position:fixed;top:0;left:0;right:0;height:' + H + 'px;z-index:2147483647;display:flex;'
      + 'align-items:stretch;background:#0a0d3a;-webkit-user-select:none;user-select:none;}'
      + '#__ep_tb .__ep_drag{flex:1;height:100%}'
      + '#__ep_tb .__ep_ctrls{display:flex;align-items:stretch}'
      + '#__ep_tb button{width:46px;height:' + H + 'px;border:0;background:transparent;color:#9aa0d4;cursor:pointer;'
      + 'display:grid;place-items:center;transition:background .12s,color .12s}'
      + '#__ep_tb button:hover{background:rgba(255,255,255,.08);color:#eef0ff}'
      + '#__ep_tb button.__ep_close:hover{background:#e13b4c;color:#fff}'
      + '#__ep_tb svg{width:14px;height:14px}'
      // DÉCALAGE PROPRE : #root occupe la zone SOUS la barre et devient le bloc conteneur de tous les
      // position:fixed de l'app (le transform crée un containing block) → panneaux/overlays/fond se
      // repositionnent relativement à cette zone, gardant leur alignement avec la topbar de l'app.
      + 'html,body{height:100%!important;margin:0!important;overflow:hidden!important}'
      + '#root{position:fixed!important;top:' + H + 'px!important;left:0!important;right:0!important;bottom:0!important;'
      + 'height:auto!important;width:auto!important;contain:layout!important;overflow:hidden!important}';
    document.head.appendChild(st);
  }

  function ensureBar() {
    if (document.getElementById('__ep_tb')) return;
    if (!document.body) return;
    var bar = document.createElement('div');
    bar.id = '__ep_tb';
    bar.innerHTML =
      '<div class="__ep_drag"></div>'
      + '<div class="__ep_ctrls">'
      + '<button class="__ep_min" title="Reduire" aria-label="Reduire">' + SVG_MIN + '</button>'
      + '<button class="__ep_max" title="Agrandir" aria-label="Agrandir">' + SVG_MAX + '</button>'
      + '<button class="__ep_close" title="Fermer" aria-label="Fermer">' + SVG_X + '</button>'
      + '</div>';
    document.body.appendChild(bar);

    var T = window.__TAURI__;
    if (T && T.window && T.window.getCurrentWindow) {
      var w = T.window.getCurrentWindow();
      bar.querySelector('.__ep_min').onclick = function () { w.minimize(); };
      bar.querySelector('.__ep_max').onclick = function () { w.toggleMaximize(); };
      bar.querySelector('.__ep_close').onclick = function () { w.close(); };
      var drag = bar.querySelector('.__ep_drag');
      drag.addEventListener('mousedown', function (e) { if (e.buttons === 1) { w.startDragging(); } });
      drag.addEventListener('dblclick', function () { w.toggleMaximize(); });
    }
  }

  function mount() { ensureStyle(); ensureBar(); }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', mount);
  else mount();
  // L'app est une SPA : si un re-render retire la barre/le style, on les remet.
  new MutationObserver(mount).observe(document.documentElement, { childList: true, subtree: true });
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
