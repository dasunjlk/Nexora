//! The Controls page: the machine's quick controls on one page — Wi-Fi,
//! Bluetooth, the mobile hotspot, the power mode, how the displays are
//! arranged and airplane mode — each switchable from the island. The state
//! is `nexora_core::controls::ControlsState`, kept in the app state and
//! filled by `services::controls`, which exists only while this page is on
//! screen (nothing runs at rest).
//!
//! The tiles: a 2×3 grid, a tile per control — its icon, name, the state in
//! words and a switch. Energy saver is a three-way picker (tap cycles Best
//! power efficiency → Balanced → Best performance); Project a four-pill row
//! (PC screen only / Duplicate / Extend / Second screen only, greyed when
//! there is one display); the other four toggle.

use crate::commands::Command;
use crate::gfx::{TextStyle, Weight};
use crate::icons::Icon;
use crate::state::AppState;
use crate::ui::{Ui, wid, wid_n};
use nexora_core::controls::{ControlsCmd, ControlsState, PowerMode, ProjectMode, Radio, TILES, Tile};
use nexora_core::geom::{palette, Point, Rect};

const TILE_H: f32 = 74.0;
const GAP: f32 = 10.0;
const GRID_TOP: f32 = 10.0;
const PICKER_H: f32 = 30.0;

pub fn height(st: &AppState) -> f32 {
    let _ = st;
    GRID_TOP + 3.0 * TILE_H + 2.0 * GAP + PICKER_H + 6.0
}

/// The whole page: the six tiles over the Project picker.
pub fn draw(ui: &mut Ui, st: &mut AppState, area: Rect) {
    let cols = 3usize;
    let w = (area.w - (cols as f32 - 1.0) * GAP) / cols as f32;
    for i in 0..TILES.len() {
        let (col, row) = (i % cols, i / cols);
        let tile = Rect::new(
            area.x + col as f32 * (w + GAP),
            area.y + GRID_TOP + row as f32 * (TILE_H + GAP),
            w,
            TILE_H,
        );
        draw_tile(ui, st, tile, TILES[i]);
    }

    // Project: the four modes, the active one in the accent. One display:
    // the modes but "PC screen only" grey out.
    let py = area.y + GRID_TOP + 3.0 * TILE_H + 2.0 * GAP;
    let label = Rect::new(area.x, py, area.w, 18.0);
    ui.c.text("Project", label, &TextStyle::footnote().color(palette::TEXT_DIM));
    let row = Rect::new(area.x, py + 18.0, area.w, PICKER_H);
    let modes = [ProjectMode::Internal, ProjectMode::Clone, ProjectMode::Extend, ProjectMode::External];
    let one = st.controls.displays <= 1;
    let pw = ((row.w - 3.0 * GAP) / 4.0).max(40.0);
    for (i, m) in modes.iter().enumerate() {
        let r = Rect::new(row.x + i as f32 * (pw + GAP), row.y, pw, PICKER_H);
        let style = if st.controls.project == *m {
            crate::ui::PillStyle::Primary(st.accent)
        } else if one && *m != ProjectMode::Internal {
            crate::ui::PillStyle::Utility
        } else {
            crate::ui::PillStyle::Neutral
        };
        let mut ts = TextStyle::footnote().center();
        if one && *m != ProjectMode::Internal {
            ts = ts.color(palette::TEXT_DIM);
        }
        if ui.pill(wid("controls.project"), r, m.label(), None, style).clicked && !one {
            st.cmd(Command::Controls(ControlsCmd::SetProject(*m)));
        }
    }
    let hint = if one {
        "One display: plug in another to Duplicate or Extend."
    } else {
        "How this screen works with the others."
    };
    ui.c.text(hint, Rect::new(area.x, row.y + PICKER_H + 2.0, area.w, 16.0), &TextStyle::footnote().color(palette::TEXT_DIM));
}

/// One tile: icon, name, the state in words, a switch; the whole tile taps.
fn draw_tile(ui: &mut Ui, st: &mut AppState, r: Rect, tile: Tile) {
    let accent = st.accent;
    let resp = ui.button(wid_n("controls.tile", tile as u64), r);
    if resp.hovered || resp.pressed {
        ui.c.fill_rrect(r, 12.0, palette::SURFACE);
    }
    ui.c.fill_rrect(r, 12.0, palette::SURFACE);

    let icon = match tile {
        Tile::Wifi => Icon::Wifi,
        Tile::Bluetooth => Icon::Bluetooth,
        Tile::Hotspot => Icon::Hotspot,
        Tile::Energy => Icon::Leaf,
        Tile::Project => Icon::Project,
        Tile::Airplane => Icon::Airplane,
    };
    let badge = Point::new(r.x + 13.0 + 9.0, r.y + 11.0 + 9.0);
    ui.c.fill_circle(badge, 18.0, accent.with_alpha(0.18));
    ui.c.icon(icon, Rect::from_center(badge, 16.0, 16.0), accent);

    let sw = Rect::new(r.right() - 12.0 - 30.0, r.y + 11.0, 30.0, 18.0);
    let (on, line) = state_line(st, tile);
    crate::pages::settings::draw_switch(ui.c, sw, on, accent);

    // The name sits right of the badge (the badge is 36 wide, + 10 gap).
    let tx = r.x + 13.0 + 36.0 + 10.0;
    let tw = (sw.x - 12.0 - tx).max(0.0);
    ui.c.text(tile.label(), Rect::new(tx, r.y + 12.0, tw, 18.0), &TextStyle::body().weight(Weight::Semibold));
    // The state spans the tile under the header row.
    let sx = r.x + 13.0;
    let sw_full = r.w - 26.0;
    ui.c.text(&line, Rect::new(sx, r.y + 38.0, sw_full, 17.0), &TextStyle::footnote().color(palette::TEXT_DIM));
    let hint = hint_line(st, tile);
    if !hint.is_empty() {
        ui.c.text(&hint, Rect::new(sx, r.y + 56.0, sw_full, 15.0), &TextStyle::footnote().color(palette::TEXT_FAINT));
    }
}

/// Whether the control is on, and the state in words. Tapping is drawn by
/// the switch's state; what a tap does is `tap`.
fn state_line(st: &AppState, tile: Tile) -> (bool, String) {
    let st = &st.controls;
    match tile {
        Tile::Wifi => match st.wifi {
            Some(on) => (on, if on { "On".into() } else { "Off".into() }),
            None => (false, "…".into()),
        },
        Tile::Bluetooth => match st.bluetooth {
            Some(on) => (on, if on { "On".into() } else { "Off".into() }),
            None => (false, "…".into()),
        },
        Tile::Hotspot => match st.hotspot {
            Some(on) => (
                on,
                if on {
                    if st.hotspot_clients > 0 {
                        format!("On · {} connected", st.hotspot_clients)
                    } else {
                        "On · nobody connected".into()
                    }
                } else {
                    "Off".into()
                },
            ),
            None => (false, "No hotspot hardware".into()),
        },
        Tile::Energy => (st.power == PowerMode::Saver, st.power.label().into()),
        Tile::Project => (st.project != ProjectMode::Internal, st.project.label().into()),
        Tile::Airplane => match st.airplane {
            Some(on) => (on, if on { "On · radios off".into() } else { "Off".into() }),
            None => (false, "…".into()),
        },
    }
}

/// The greyed second line: what tapping does when the OS hasn't answered yet.
fn hint_line(st: &AppState, tile: Tile) -> String {
    let unknown = match tile {
        Tile::Wifi => st.controls.wifi.is_none(),
        Tile::Bluetooth => st.controls.bluetooth.is_none(),
        Tile::Hotspot => st.controls.hotspot.is_none(),
        Tile::Airplane => st.controls.airplane.is_none(),
        _ => false,
    };
    if unknown {
        "Asking Windows…".into()
    } else {
        match tile {
            Tile::Energy => "Tap to change the power mode".into(),
            Tile::Project => "Tap a mode above".into(),
            Tile::Airplane => "Turns Wi-Fi and Bluetooth off".into(),
            _ => String::new(),
        }
    }
}

/// What a tap on `tile` asks for, given the state (`app.rs` turns it into a
/// `Command::Controls` for the service).
pub fn tap(st: &AppState, tile: Tile) -> Option<ControlsCmd> {
    match tile {
        Tile::Wifi => Some(ControlsCmd::ToggleRadio(Radio::Wifi)),
        Tile::Bluetooth => Some(ControlsCmd::ToggleRadio(Radio::Bluetooth)),
        Tile::Hotspot => Some(ControlsCmd::SetHotspot(!st.controls.hotspot.unwrap_or(false))),
        Tile::Energy => Some(ControlsCmd::SetPower(st.controls.power.next())),
        Tile::Airplane => Some(ControlsCmd::SetAirplane(st.controls.airplane != Some(true))),
        Tile::Project => None,
    }
}

/// The page's height is fixed (the grid + the picker); `ControlsState` rides
/// along so the signature matches the other pages'.
pub fn _state_touch(_st: &ControlsState) {}
