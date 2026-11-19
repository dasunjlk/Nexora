//! The Controls tab's state machine (`nexora_core::controls`): what the page
//! shows and what a tap means, with no OS calls — the Windows side
//! (`services/controls.rs`) reads and sets the real radios, hotspot, power
//! mode and displays, and the page draws whatever this module models.
//!
//! Everything the OS hasn't said yet is `None` (the tile shows "…" and its
//! switch stays where it was). Airplane mode is the page's own concept: ON
//! turns every radio off, remembering which were on; OFF restores exactly
//! those. A radio turned off by hand while the airplane is up stays off when
//! it lifts.

/// The machine's power mode, Windows 11's "Power mode" picker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PowerMode {
    /// Best power efficiency.
    Saver,
    /// Balanced.
    #[default]
    Balanced,
    /// Best performance.
    Performance,
}

impl PowerMode {
    /// The next mode in the picker's order, wrapping.
    pub fn next(self) -> PowerMode {
        match self {
            PowerMode::Saver => PowerMode::Balanced,
            PowerMode::Balanced => PowerMode::Performance,
            PowerMode::Performance => PowerMode::Saver,
        }
    }

    /// The line the tile shows.
    pub fn label(self) -> &'static str {
        match self {
            PowerMode::Saver => "Best power efficiency",
            PowerMode::Balanced => "Balanced",
            PowerMode::Performance => "Best performance",
        }
    }
}

/// How the displays are arranged, Windows 11's Project picker (Win+P).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProjectMode {
    /// PC screen only.
    #[default]
    Internal,
    /// Duplicate.
    Clone,
    /// Extend.
    Extend,
    /// Second screen only.
    External,
}

impl ProjectMode {
    /// The line the picker shows.
    pub fn label(self) -> &'static str {
        match self {
            ProjectMode::Internal => "PC screen only",
            ProjectMode::Clone => "Duplicate",
            ProjectMode::Extend => "Extend",
            ProjectMode::External => "Second screen only",
        }
    }
}

/// The Controls tab's tiles, in the page's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Wifi,
    Bluetooth,
    Hotspot,
    Energy,
    Project,
    Airplane,
}

/// The tiles in their page order.
pub const TILES: [Tile; 6] = [
    Tile::Wifi,
    Tile::Bluetooth,
    Tile::Hotspot,
    Tile::Energy,
    Tile::Project,
    Tile::Airplane,
];

impl Tile {
    /// The tile's name.
    pub fn label(self) -> &'static str {
        match self {
            Tile::Wifi => "Wi-Fi",
            Tile::Bluetooth => "Bluetooth",
            Tile::Hotspot => "Mobile hotspot",
            Tile::Energy => "Energy saver",
            Tile::Project => "Project",
            Tile::Airplane => "Airplane mode",
        }
    }
}

/// The machine's controls, as the OS last reported them and as the person
/// has them. The page holds one; the service fills it and answers the taps.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ControlsState {
    /// Wi-Fi's radio, None until the OS answered.
    pub wifi: Option<bool>,
    /// Bluetooth's radio.
    pub bluetooth: Option<bool>,
    /// The mobile hotspot (None: no adapter or tethering hardware).
    pub hotspot: Option<bool>,
    /// Devices connected to the hotspot, while it's on.
    pub hotspot_clients: u32,
    /// The power mode.
    pub power: PowerMode,
    /// How the displays are arranged.
    pub project: ProjectMode,
    /// How many displays are active (the Project picker says when it's one).
    pub displays: u32,
    /// The airplane state (None: not known yet). What the tile shows; the
    /// radios it turned off and what to restore are the service's business.
    pub airplane: Option<bool>,
}

impl ControlsState {
    /// Whether every radio is off, as far as the OS says. `None` while a
    /// radio's state isn't known.
    pub fn airplane_effective(&self) -> Option<bool> {
        match (self.wifi, self.bluetooth) {
            (Some(w), Some(b)) => Some(!w && !b),
            _ => None,
        }
    }

    /// Apply a radio's new state (the service heard it from the OS, or a tap
    /// turned it). Turning a radio on while the airplane is up ends the
    /// airplane, the way Windows' own quick settings do; turning one off
    /// leaves the airplane as it is.
    pub fn set_radio(&mut self, radio: Radio, on: bool) {
        match radio {
            Radio::Wifi => self.wifi = Some(on),
            Radio::Bluetooth => self.bluetooth = Some(on),
        }
        if on && self.airplane == Some(true) {
            self.airplane = Some(false);
        }
    }
}

/// A radio the Controls tab can switch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Radio {
    Wifi,
    Bluetooth,
}

/// The radio a tile toggles, if it has one.
pub fn radio_of(tile: Tile) -> Option<Radio> {
    match tile {
        Tile::Wifi => Some(Radio::Wifi),
        Tile::Bluetooth => Some(Radio::Bluetooth),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_radio_turned_on_ends_the_airplane() {
        let mut st = ControlsState { wifi: Some(false), bluetooth: Some(false), airplane: Some(true), ..Default::default() };
        st.set_radio(Radio::Wifi, true);
        assert_eq!(st.airplane, Some(false), "a radio back on: the airplane lifts");
        st.airplane = Some(true);
        st.set_radio(Radio::Bluetooth, false);
        assert_eq!(st.airplane, Some(true), "turning a dark radio darker changes nothing");
    }

    #[test]
    fn airplane_is_none_while_a_radio_is_unknown() {
        let st = ControlsState { wifi: Some(false), bluetooth: None, ..Default::default() };
        assert_eq!(st.airplane_effective(), None);
    }

    #[test]
    fn a_radio_turned_off_by_hand_stays_off_when_airplane_lifts() {
        // The service remembers what it turned off; the page only shows the
        // radios as they are.
        let mut st = ControlsState { wifi: Some(false), bluetooth: Some(false), airplane: Some(true), ..Default::default() };
        st.set_radio(Radio::Bluetooth, true);
        assert_eq!(st.bluetooth, Some(true));
        st.set_radio(Radio::Bluetooth, false);
        assert_eq!(st.bluetooth, Some(false));
        st.airplane = st.airplane_effective();
        assert_eq!(st.airplane, Some(true));
    }

    #[test]
    fn power_mode_cycles_through_the_picker() {
        assert_eq!(PowerMode::default(), PowerMode::Balanced);
        assert_eq!(PowerMode::Balanced.next(), PowerMode::Performance);
        assert_eq!(PowerMode::Performance.next(), PowerMode::Saver);
        assert_eq!(PowerMode::Saver.next(), PowerMode::Balanced);
        assert_eq!(PowerMode::Saver.label(), "Best power efficiency");
    }

    #[test]
    fn project_labels_match_windows_pickers() {
        assert_eq!(ProjectMode::Internal.label(), "PC screen only");
        assert_eq!(ProjectMode::Clone.label(), "Duplicate");
        assert_eq!(ProjectMode::Extend.label(), "Extend");
        assert_eq!(ProjectMode::External.label(), "Second screen only");
    }

    #[test]
    fn only_wifi_and_bluetooth_are_radios() {
        for tile in TILES {
            assert_eq!(radio_of(tile).is_some(), matches!(tile, Tile::Wifi | Tile::Bluetooth));
        }
    }
}
