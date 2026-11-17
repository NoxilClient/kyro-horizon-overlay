# KYRO Horizon main-menu overlay

A client-side native Rust mod starter for Horizon. It keeps Minecraft's existing title-screen background and vanilla Singleplayer/Multiplayer controls, adds a centered KYRO identity and a Discord button, and covers the stock Partner tile. The Discord action opens the fixed invite URL in the system browser.

## Create the pinned native project

The checked-in `horizon-mod.toml` pins this starter to Horizon commit `c07bfa76e71e8db9e84a7e61832a8fb898718e81`. Generate/fetch its SDK before editing or building:

```sh
cargo horizon-mod new kyro-overlay --template native --horizon c07bfa76e71e8db9e84a7e61832a8fb898718e81
```

If the tool says that build has no SDK, use a current Horizon build instead and let `cargo horizon-mod new` generate the matching manifest and SDK pin; keep this repository's `src/lib.rs` and copy it into the generated project. Native mods run with your account's permissions and are not sandboxed. Review the code before building.

## Build and install

```sh
cd kyro-overlay
horizon-launcher native-build . --instance "My Instance" --trust-native-code
```

Open the instance and go to the title screen. The vanilla title buttons retain their original hitboxes/actions. Click **Discord** in the top-right overlay to open `https://discord.gg/Zq2F3QSDPZ`.

## Artwork

The overlay uses text and flat fills for its placeholder brand/buttons, so it works without images. Add your PNGs under `assets/kyro/textures/gui/` and register/load them through the pinned SDK's sprite API before switching the placeholder drawing to sprites. The game background remains Minecraft's own panorama; there is no replacement background asset.

## Note on the Partner tile

The title-screen API does not expose a dedicated "remove partner tile" operation. The overlay paints a dark translucent cover over its usual bottom-right location, keeping the panorama visible beneath it. If Minecraft changes the title layout, adjust `partner_cover()` in `src/lib.rs`.
