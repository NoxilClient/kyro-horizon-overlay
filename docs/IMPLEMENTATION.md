# Mod implementation notes

This project is a native mod overlay, not a fork/rebuild of Horizon's engine. The title screen's vanilla panorama and native Singleplayer/Multiplayer actions remain in place. The Partner promo is covered with a translucent dark patch because the mod API does not expose a remove-partner-widget operation.

The Discord link is fixed to the HTTPS invite in `src/lib.rs` and launched with a platform opener without a shell. Native mods run with the player's permissions and are not sandboxed; inspect and trust the source before building.
