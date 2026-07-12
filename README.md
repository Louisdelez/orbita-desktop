# Orbita Desktop

Application desktop **Orbita** (Epicube Gestion) — une coquille native (Rust + [Tauri](https://tauri.app) v2)
qui affiche l'application web **[gestion.epicube.ch](https://gestion.epicube.ch)** dans une vraie fenêtre
d'application, hors du navigateur.

L'application web reste la **seule source de vérité** : ce binaire ne fait que la charger. Toute mise à jour
du web est donc immédiate côté desktop, sans rebuild.

## Distribution

Buildée pour **Windows, macOS et Linux** via GitHub Actions (voir `.github/workflows/build.yml`).
Elle est distribuée **depuis le Launcher Epicube** : dans le launcher, sélectionne **Orbita** puis
Installer / Jouer — le launcher télécharge le binaire portable et le lance (même principe que les serveurs
Minecraft). Le launcher lit le manifeste `https://epicube.ch/mods/epicube-orbita.json`.

## Build local

```bash
cargo tauri build            # installeurs de l'OS courant
cargo tauri build --bundles appimage   # Linux : AppImage portable
```

## Structure

- `src-tauri/` — projet Tauri (Rust). La fenêtre + son URL distante sont dans `tauri.conf.json`
  (`app.windows[0].url = https://gestion.epicube.ch`).
- `dist/index.html` — placeholder (la fenêtre charge l'URL distante, pas ce fichier).
