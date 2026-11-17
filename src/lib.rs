use mod_api::{
    ClientHost, Hooks, InputKeyEvent, KeyAction, ScreenMouseButtonPressedPreEvent,
    ScreenRenderPostEvent, Text,
};
use std::{
    process::Command,
    sync::Mutex,
    time::{Duration, Instant},
};

const DISCORD_URL: &str = "https://discord.gg/Zq2F3QSDPZ";
const DISCORD_BUTTON: &str = "kyro_discord_button";
const TITLE_SCREEN_ID: &str = "title";
const INTRO_DURATION: Duration = Duration::from_millis(650);

#[derive(Default)]
pub struct KyroMainMenu {
    intro_started: Mutex<Option<Instant>>,
}

impl KyroMainMenu {
    fn intro_progress(&self) -> f32 {
        let mut started = self.intro_started.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let began = *started.get_or_insert(now);
        let t = (now.duration_since(began).as_secs_f32() / INTRO_DURATION.as_secs_f32())
            .clamp(0.0, 1.0);
        // Smoothstep: no abrupt start or stop.
        t * t * (3.0 - 2.0 * t)
    }

    fn draw_title(&self, host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
        if event.screen.id != TITLE_SCREEN_ID {
            return;
        }

        let [width, height] = event.draw.size();
        let p = self.intro_progress();
        let center_x = width * 0.5;
        let center_y = height * 0.36 + (1.0 - p) * 10.0;
        let alpha = (p * 255.0).round() as u32;
        let text = |rgb: u32| (alpha << 24) | rgb;

        // A small title-screen player chip, leaving Minecraft's panorama untouched.
        event.draw.fill([20.0, 18.0, 190.0, 30.0], 0x9907070B);
        event.draw.text("KYRO CLIENT", [32.0, 27.0], text(0x00FFFFFF), true);

        // Center brand placeholder. Replace this with your logo sprite later if desired.
        let logo_size = 82.0 * (0.94 + 0.06 * p);
        let logo_x = center_x - logo_size * 0.5;
        let logo_y = center_y - logo_size * 0.5;
        event.draw.fill([logo_x, logo_y, logo_size, logo_size], text(0x0012121A));
        event.draw.text("KC", [center_x - 27.0, logo_y + 29.0], text(0x00FFFFFF), true);
        let subtitle = "Kruth Client";
        let subtitle_w = event.draw.text_width(subtitle);
        event.draw.text(
            subtitle,
            [center_x - subtitle_w * 0.5, logo_y + logo_size + 17.0],
            text(0x00FFFFFF),
            true,
        );

        // The Singleplayer and Multiplayer buttons underneath are still vanilla's controls;
        // this overlay deliberately does not steal their clicks or change their actions.
        let button_x = center_x - 74.0;
        let button_y = center_y + 112.0;
        for (index, label) in ["Singleplayer", "Multiplayer"].iter().enumerate() {
            let y = button_y + index as f32 * 38.0;
            event.draw.fill([button_x, y, 148.0, 30.0], 0xB3000000 | (alpha << 16));
            let label_w = event.draw.text_width(label);
            event.draw.text(
                label,
                [center_x - label_w * 0.5, y + 9.0],
                text(0x00FFFFFF),
                false,
            );
        }

        // Compact top-right Discord action.
        let discord = discord_rect(width);
        event.draw.fill(discord, 0xB3000000);
        event.draw.text("Discord", [discord[0] + 11.0, discord[1] + 9.0], text(0x00FFFFFF), false);

        // Hide the vanilla Partner promo while allowing the Minecraft panorama to show through.
        let cover = partner_cover(width, height);
        event.draw.fill(cover, 0xD906080E);

        // Keep the familiar bottom-right copyright label visually clear.
        let copyright = "Copyright Mojang Studios";
        let copyright_w = event.draw.text_width(copyright);
        event.draw.text(
            copyright,
            [width - copyright_w - 22.0, height - 23.0],
            text(0x00FFFFFF),
            true,
        );

        let _ = host; // Host is read-only in this render hook.
    }
}

fn discord_rect(width: f32) -> [f32; 4] {
    [width - 166.0, 18.0, 76.0, 30.0]
}

fn partner_cover(width: f32, height: f32) -> [f32; 4] {
    [width - 194.0, height - 110.0, 176.0, 78.0]
}

fn open_discord() {
    // Fixed HTTPS invite only; pass it as an argument and never through a shell.
    #[cfg(target_os = "windows")]
    let result = Command::new("rundll32.exe")
        .args(["url.dll,FileProtocolHandler", DISCORD_URL])
        .spawn();

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(DISCORD_URL).spawn();

    #[cfg(all(unix, not(target_os = "macos")))]
    let result = Command::new("xdg-open").arg(DISCORD_URL).spawn();

    if let Err(error) = result {
        log::warn!("KYRO: could not open Discord invite in the system browser: {error}");
    }
}

#[mod_api::horizon_mod]
impl Hooks for KyroMainMenu {
    fn screen_render_post(&self, host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
        self.draw_title(host, event);
    }

    fn screen_mouse_button_pressed_pre(
        &self,
        _host: &dyn ClientHost,
        event: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) {
        if event.screen.id != TITLE_SCREEN_ID || event.button != 0 {
            return;
        }
        let [x, y, w, h] = discord_rect(event.draw.size()[0]);
        let [mouse_x, mouse_y] = event.screen.mouse;
        if (x..=x + w).contains(&mouse_x) && (y..=y + h).contains(&mouse_y) {
            open_discord();
            event.canceled = true;
        }
    }

    fn input_key(&self, _host: &mut dyn ClientHost, event: &mut InputKeyEvent<'_>) {
        // Right Shift is intentionally not intercepted globally: vanilla screen and gameplay
        // input remain unchanged. The Discord action is available by clicking its title button.
        let _ = (event.action == KeyAction::Press, event.key.as_str());
    }
}
