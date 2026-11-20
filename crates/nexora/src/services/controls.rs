//! The Controls tab's service: it reads the machine's quick controls — the
//! Wi-Fi and Bluetooth radios, the mobile hotspot, the power mode and how
//! the displays are arranged — and carries out what the page's tiles ask
//! (`Command::Controls`). Its thread exists only while the page is on
//! screen: the page came or went (`ControlsService::set_open`), a tap
//! arrived (`controls_cmd`), or the app is quitting.
//!
//! It reports with `AppEvent::Controls(ControlsState)` — a full snapshot
//! every time anything it knows changes, so the page only ever draws it.
//! Refreshes: a snapshot when the page opens, again every 3 s while it
//! stays (the radios and the hotspot have no events an unpackaged desktop
//! app can take; the 3 s asks nothing but the radio and tethering states),
//! and right after every command.
//!
//! The self-test's copy (`start_selftest`) never touches the OS: the states
//! are canned and a tap only flips the model, so a screenshot of the page
//! works on any machine — and, the rule, no test ever turns a radio off.

use crate::events::{AppEvent, EventSink};
use nexora_core::controls::{ControlsCmd, ControlsState, PowerMode, ProjectMode, Radio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// How often the page's snapshot refreshes while it's open.
const REFRESH: Duration = Duration::from_secs(3);

struct Queue {
    open: bool,
    on_battery: bool,
    cmds: Vec<ControlsCmd>,
    shutdown: bool,
}

struct Shared {
    q: Mutex<Queue>,
    cv: Condvar,
}

/// The Controls tab's service. `start` for the app, `start_selftest` for the
/// scripted screenshots: same behaviour, no OS calls.
pub struct ControlsService {
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
}

impl ControlsService {
    fn spawn(sink: EventSink, fake: bool) -> ControlsService {
        let shared = Arc::new(Shared {
            q: Mutex::new(Queue { open: false, on_battery: false, cmds: Vec::new(), shutdown: false }),
            cv: Condvar::new(),
        });
        let worker = shared.clone();
        let thread = std::thread::Builder::new()
            .name("nexora-controls".into())
            .spawn(move || run(&worker, &sink, fake))
            .ok();
        ControlsService { shared, thread }
    }

    pub fn start(sink: EventSink) -> ControlsService {
        ControlsService::spawn(sink, false)
    }

    pub fn start_selftest(sink: EventSink) -> ControlsService {
        ControlsService::spawn(sink, true)
    }

    /// The page came or went: the thread wakes once, and with the page gone
    /// it sleeps until it's back (or the app quits).
    pub fn set_open(&self, open: bool) {
        let mut q = self.shared.q.lock().unwrap();
        if q.open != open {
            q.open = open;
            self.shared.cv.notify_all();
        }
    }

    /// Windows is on battery (or not): the power mode the page shows is the
    /// scheme of the source in use.
    pub fn set_on_battery(&self, on_battery: bool) {
        let mut q = self.shared.q.lock().unwrap();
        if q.on_battery != on_battery {
            q.on_battery = on_battery;
            self.shared.cv.notify_all();
        }
    }

    /// A tap on a tile.
    pub fn cmd(&self, c: ControlsCmd) {
        let mut q = self.shared.q.lock().unwrap();
        q.cmds.push(c);
        self.shared.cv.notify_all();
    }
}

impl Drop for ControlsService {
    fn drop(&mut self) {
        self.shared.q.lock().unwrap().shutdown = true;
        self.shared.cv.notify_all();
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn run(shared: &Shared, sink: &EventSink, fake: bool) {
    let mut model = ControlsState::default();
    // What the airplane turned off, to restore when it lifts (the fake mode
    // exercises the same bookkeeping the real one does).
    let mut airplane_saved: Option<(bool, bool)> = None;
    loop {
        let (cmds, open) = {
            let mut g = shared.q.lock().unwrap();
            loop {
                if g.shutdown {
                    return;
                }
                if !g.cmds.is_empty() || g.open {
                    break;
                }
                g = shared.cv.wait(g).unwrap();
            }
            let cmds = std::mem::take(&mut g.cmds);
            (cmds, g.open)
        };
        for c in cmds {
            apply(&mut model, &mut airplane_saved, c, sink, fake);
        }
        if open {
            model = snapshot(model, false, fake);
            sink.send(AppEvent::Controls(Box::new(model.clone())));
            // Sleep the refresh gap, waking early for a tap.
            let until = Instant::now() + REFRESH;
            let mut g = shared.q.lock().unwrap();
            loop {
                let Some(wait) = until.checked_duration_since(Instant::now()) else { break };
                let (guard, _) = shared.cv.wait_timeout(g, wait).unwrap();
                g = guard;
                if g.shutdown || !g.cmds.is_empty() {
                    break;
                }
            }
        }
    }
}

/// One command, on the service's thread. `fake`: flip the model only.
fn apply(model: &mut ControlsState, airplane_saved: &mut Option<(bool, bool)>, c: ControlsCmd, sink: &EventSink, fake: bool) {
    match c {
        ControlsCmd::ToggleRadio(radio) => {
            let now_on = match radio {
                Radio::Wifi => !model.wifi.unwrap_or(true),
                Radio::Bluetooth => !model.bluetooth.unwrap_or(true),
            };
            if fake {
                model.set_radio(radio, now_on);
            } else if set_radio(radio, now_on) {
                model.set_radio(radio, now_on);
                if now_on {
                    *airplane_saved = None;
                }
            }
        }
        ControlsCmd::SetAirplane(on) => {
            if fake {
                if on {
                    *airplane_saved = Some((model.wifi.unwrap_or(true), model.bluetooth.unwrap_or(true)));
                    model.wifi = Some(false);
                    model.bluetooth = Some(false);
                } else if let Some((w, b)) = *airplane_saved {
                    model.wifi = Some(w);
                    model.bluetooth = Some(b);
                }
                model.airplane = Some(on);
            } else if on {
                let was_wifi = model.wifi.unwrap_or(true);
                let was_bt = model.bluetooth.unwrap_or(true);
                let ok = set_radio(Radio::Wifi, false) && set_radio(Radio::Bluetooth, false);
                if ok {
                    *airplane_saved = Some((was_wifi, was_bt));
                    model.wifi = Some(false);
                    model.bluetooth = Some(false);
                    model.airplane = Some(true);
                }
            } else if let Some((w, b)) = *airplane_saved {
                let _ = set_radio(Radio::Wifi, w);
                let _ = set_radio(Radio::Bluetooth, b);
                model.wifi = Some(w);
                model.bluetooth = Some(b);
                model.airplane = Some(false);
                *airplane_saved = None;
            } else {
                model.airplane = Some(false);
            }
        }
        ControlsCmd::SetHotspot(on) => {
            if fake {
                model.hotspot = Some(on);
            } else if set_hotspot(on) {
                model.hotspot = Some(on);
            }
        }
        ControlsCmd::SetPower(mode) => {
            if fake {
                model.power = mode;
            } else if set_power_mode(mode) {
                model.power = mode;
            }
        }
        ControlsCmd::SetProject(mode) => {
            if fake {
                model.project = mode;
            } else if set_project(mode) {
                model.project = mode;
            }
        }
    }
    sink.send(AppEvent::Controls(Box::new(model.clone())));
}

/// What the OS says now, over what the page already knows. `fake`: the
/// canned machine (two radios on, hotspot off, two displays).
fn snapshot(mut model: ControlsState, _on_battery: bool, fake: bool) -> ControlsState {
    if fake {
        if model.wifi.is_none() {
            model.wifi = Some(true);
            model.bluetooth = Some(true);
            model.hotspot = Some(false);
            model.airplane = Some(false);
        }
        model.power = PowerMode::Balanced;
        model.project = ProjectMode::Extend;
        model.displays = 2;
        return model;
    }
    let (wifi, bluetooth) = read_radios();
    model.wifi = Some(wifi);
    model.bluetooth = Some(bluetooth);
    if let Some((on, clients)) = read_hotspot() {
        model.hotspot = Some(on);
        model.hotspot_clients = clients;
    }
    model.power = read_power_mode();
    let (project, displays) = read_project();
    model.project = project;
    model.displays = displays;
    model.airplane = model.airplane_effective();
    model
}

// ---------------------------------------------------------------- the OS

/// COM on this thread (the WinRT calls want an apartment; the thread is
/// MTA for the whole run).
#[cfg(windows)]
fn com() {
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok() };
}

/// The two radios' states, `(wifi, bluetooth)`; a missing radio reads off.
#[cfg(windows)]
fn read_radios() -> (bool, bool) {
    com();
    use windows::Devices::Radios::{Radio as WinRadio, RadioKind, RadioState};
    let mut out = (false, false);
    let ok = (|| -> windows::core::Result<()> {
        let radios = WinRadio::GetRadiosAsync()?.get()?;
        for i in 0..radios.Size()? {
            let r = radios.GetAt(i)?;
            let on = r.State()? == RadioState::On;
            match r.Kind()? {
                RadioKind::WiFi => out.0 = on,
                RadioKind::Bluetooth => out.1 = on,
                _ => {}
            }
        }
        Ok(())
    })();
    if let Err(e) = ok {
        crate::pwarn!("controls: the radios didn't read ({})", e);
    }
    out
}

/// Sets one radio's state; true when the OS said it took.
#[cfg(windows)]
fn set_radio(radio: Radio, on: bool) -> bool {
    com();
    use windows::Devices::Radios::{Radio as WinRadio, RadioKind, RadioState};
    let kind = match radio {
        Radio::Wifi => RadioKind::WiFi,
        Radio::Bluetooth => RadioKind::Bluetooth,
    };
    let want = if on { RadioState::On } else { RadioState::Off };
    let what = if radio == Radio::Wifi { "Wi-Fi" } else { "Bluetooth" };
    let done = (|| -> windows::core::Result<bool> {
        let radios = WinRadio::GetRadiosAsync()?.get()?;
        for i in 0..radios.Size()? {
            let r = radios.GetAt(i)?;
            if r.Kind()? == kind {
                let access = r.SetStateAsync(want)?.get()?;
                return Ok(access == windows::Devices::Radios::RadioAccessStatus::Allowed);
            }
        }
        Ok(false)
    })();
    match done {
        Ok(true) => {
            crate::pinfo!("controls: {} {}", what, if on { "on" } else { "off" });
            true
        }
        Ok(false) => {
            crate::pwarn!("controls: the {} radio didn't take (denied)", what);
            false
        }
        Err(e) => {
            crate::pwarn!("controls: the {} radio didn't set ({})", what, e);
            false
        }
    }
}

/// The hotspot's `(on, clients)`, or None without the hardware/profile.
#[cfg(windows)]
fn read_hotspot() -> Option<(bool, u32)> {
    com();
    use windows::Networking::Connectivity::NetworkInformation;
    use windows::Networking::NetworkOperators::{NetworkOperatorTetheringManager, TetheringOperationalState};
    let profile = NetworkInformation::GetInternetConnectionProfile().ok()?;
    let mgr = NetworkOperatorTetheringManager::CreateFromConnectionProfile(&profile).ok()?;
    let on = mgr.TetheringOperationalState().ok()? == TetheringOperationalState::On;
    let clients = mgr.ClientCount().ok()?;
    Some((on, clients))
}

/// Starts or stops the hotspot; true when the OS said it took.
#[cfg(windows)]
fn set_hotspot(on: bool) -> bool {
    com();
    use windows::Networking::Connectivity::NetworkInformation;
    use windows::Networking::NetworkOperators::{NetworkOperatorTetheringManager, TetheringOperationalState};
    let Some(profile) = NetworkInformation::GetInternetConnectionProfile().ok() else {
        crate::pwarn!("controls: no connection profile, so no hotspot");
        return false;
    };
    let Ok(mgr) = NetworkOperatorTetheringManager::CreateFromConnectionProfile(&profile) else {
        crate::pwarn!("controls: the hotspot didn't open");
        return false;
    };
    let now_on = mgr.TetheringOperationalState().map(|s| s == TetheringOperationalState::On).unwrap_or(false);
    if now_on == on {
        return true;
    }
    let what = if on { "start" } else { "stop" };
    let done = if on { mgr.StartTetheringAsync() } else { mgr.StopTetheringAsync() };
    match done {
        Ok(op) => match op.get() {
            Ok(_) => {
                crate::pinfo!("controls: the hotspot {}", if on { "started" } else { "stopped" });
                true
            }
            Err(e) => {
                crate::pwarn!("controls: the hotspot didn't {} ({})", what, e);
                false
            }
        },
        Err(e) => {
            crate::pwarn!("controls: the hotspot didn't {} ({})", what, e);
            false
        }
    }
}

/// The three overlay-scheme GUIDs Windows 11's Power mode picker sets
/// (`PowerSetActiveOverlayScheme`, docs/Power Management).
#[cfg(windows)]
mod scheme {
    use windows::core::GUID;
    pub const SAVER: GUID = GUID::from_u128(0x961cc777_ac5b_4cf6_a5a1_54f2c19dbee5);
    pub const BALANCED: GUID = GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
    pub const PERFORMANCE: GUID = GUID::from_u128(0xded574b5_45a0_4f42_8736_01173c5a3549);

    pub fn as_u128(g: &GUID) -> u128 {
        u128::from(*g)
    }
}

/// The active power mode, from the Power key's overlay values (read-only, no
/// admin needed): the scheme of the source in use.
#[cfg(windows)]
fn read_power_mode() -> PowerMode {
    let key = "SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes";
    let guid = read_reg_str(key, "ActiveOverlayAcScheme").unwrap_or_default();
    if let Some(g) = parse_guid(&guid) {
        let n = scheme::as_u128(&g);
        if n == scheme::as_u128(&scheme::SAVER) {
            return PowerMode::Saver;
        }
        if n == scheme::as_u128(&scheme::PERFORMANCE) {
            return PowerMode::Performance;
        }
    }
    PowerMode::Balanced
}

/// A GUID string ("961cc777-ac5b-...") into a windows GUID, or None.
#[cfg(windows)]
fn parse_guid(s: &str) -> Option<windows::core::GUID> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    Some(windows::core::GUID::from(s))
}

/// One REG_SZ out of the registry, read-only.
#[cfg(windows)]
fn read_reg_str(key: &str, value: &str) -> Option<String> {
    use windows::Win32::System::Registry::{RegGetValueW, HKEY, HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ};
    use windows::core::{HSTRING, PCWSTR};
    let key_w: Vec<u16> = HSTRING::from(key).as_wide().iter().copied().chain([0]).collect();
    let value_w: Vec<u16> = HSTRING::from(value).as_wide().iter().copied().chain([0]).collect();
    let mut buf = [0u16; 160];
    let mut size = (buf.len() * 2) as u32;
    let mut kind = windows::Win32::System::Registry::REG_VALUE_TYPE(0);
    let ok = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(key_w.as_ptr()),
            PCWSTR(value_w.as_ptr()),
            RRF_RT_REG_SZ,
            Some(&mut kind as *mut _ as *mut windows::Win32::System::Registry::REG_VALUE_TYPE),
            Some(buf.as_mut_ptr() as *mut core::ffi::c_void),
            Some(&mut size),
        )
    };
    ok.ok().ok()?;
    let len = (size as usize / 2).min(buf.len());
    Some(String::from_utf16_lossy(&buf[..len]).trim_end_matches('\0').trim_end().to_string())
}

/// Sets the power mode with `PowerSetActiveOverlayScheme`; true when it took.
/// Sets the power mode with `PowerSetActiveOverlayScheme`, looked up at run
/// time from powrprof (Windows 10 1709+; the overlay scheme the Settings
/// app's Power mode picker writes). False when the lookup fails (Wine, an
/// old Windows).
#[cfg(windows)]
fn set_power_mode(mode: PowerMode) -> bool {
    use windows::core::{s, w, PCSTR};
    use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    let guid = match mode {
        PowerMode::Saver => scheme::SAVER,
        PowerMode::Balanced => scheme::BALANCED,
        PowerMode::Performance => scheme::PERFORMANCE,
    };
    let took = (|| -> Option<bool> {
        let powrprof = unsafe { GetModuleHandleW(w!("powrprof.dll")).ok()? };
        let f = unsafe { GetProcAddress(powrprof, s!("PowerSetActiveOverlayScheme"))? };
        let f: unsafe extern "system" fn(*const windows::core::GUID) -> i32 = unsafe { std::mem::transmute(f) };
        let r = unsafe { f(&guid) };
        if r == 0 {
            crate::pinfo!("controls: power mode {:?}", mode);
            Some(true)
        } else {
            crate::pwarn!("controls: the power mode didn't set (code {})", r);
            Some(false)
        }
    })();
    took.unwrap_or_else(|| {
        crate::pwarn!("controls: powrprof's PowerSetActiveOverlayScheme not found");
        false
    })
}

/// How the displays are arranged now, from the active database paths: one
/// path is one screen, two sharing a source are a duplicate, two disjoint
/// sources are an extend.
#[cfg(windows)]
fn read_project() -> (ProjectMode, u32) {
    use windows::Win32::Devices::Display::{GetDisplayConfigBufferSizes, QueryDisplayConfig, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, QDC_DATABASE_CURRENT};
    let mut paths = [DISPLAYCONFIG_PATH_INFO::default(); 8];
    let mut modes = [DISPLAYCONFIG_MODE_INFO::default(); 16];
    let mut n_paths = paths.len() as u32;
    let mut n_modes = modes.len() as u32;
    unsafe {
        if GetDisplayConfigBufferSizes(QDC_DATABASE_CURRENT, &mut n_paths, &mut n_modes).is_err() {
            return (ProjectMode::Internal, 1);
        }
        if QueryDisplayConfig(QDC_DATABASE_CURRENT, &mut n_paths, paths.as_mut_ptr(), &mut n_modes, modes.as_mut_ptr(), None).is_err() {
            return (ProjectMode::Internal, 1);
        }
    }
    let active = n_paths as usize;
    let displays = active.max(1) as u32;
    let mode = match active {
        1 => ProjectMode::Internal,
        0 => ProjectMode::Internal,
        _ => {
            let clone = (1..active).any(|i| {
                paths[i].sourceInfo.adapterId == paths[0].sourceInfo.adapterId && paths[i].sourceInfo.id == paths[0].sourceInfo.id
            });
            if clone {
                ProjectMode::Clone
            } else {
                ProjectMode::Extend
            }
        }
    };
    (mode, displays)
}

/// `SetDisplayConfig` with the topology parameter: the windows crate's
/// wrapper omits it, so the raw user32 export is declared here.
#[cfg(windows)]
use windows::Win32::Devices::Display::{
    DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_TOPOLOGY_ID, SET_DISPLAY_CONFIG_FLAGS,
};

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn SetDisplayConfig(
        numpatharrayelements: u32,
        patharray: *const DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: u32,
        modeinfoarray: *const DISPLAYCONFIG_MODE_INFO,
        topologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID,
        flags: SET_DISPLAY_CONFIG_FLAGS,
    ) -> i32;
}

#[cfg(windows)]
const SDC_TOPOLOGY_SUPPLIED: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(1u32);
#[cfg(windows)]
const SDC_APPLY: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(128u32);

#[cfg(windows)]
const DISPLAYCONFIG_TOPOLOGY_INTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(1i32);
#[cfg(windows)]
const DISPLAYCONFIG_TOPOLOGY_CLONE: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(2i32);
#[cfg(windows)]
const DISPLAYCONFIG_TOPOLOGY_EXTEND: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(4i32);
#[cfg(windows)]
const DISPLAYCONFIG_TOPOLOGY_EXTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(8i32);

/// Sets the display topology the way Win+P's four modes do
/// (`SDC_TOPOLOGY_SUPPLIED` with the topology id, no mode changes).
#[cfg(windows)]
fn set_project(mode: ProjectMode) -> bool {
    let mut topology: DISPLAYCONFIG_TOPOLOGY_ID = match mode {
        ProjectMode::Internal => DISPLAYCONFIG_TOPOLOGY_INTERNAL,
        ProjectMode::Clone => DISPLAYCONFIG_TOPOLOGY_CLONE,
        ProjectMode::Extend => DISPLAYCONFIG_TOPOLOGY_EXTEND,
        ProjectMode::External => DISPLAYCONFIG_TOPOLOGY_EXTERNAL,
    };
    let r = unsafe {
        SetDisplayConfig(0, std::ptr::null(), 0, std::ptr::null(), &mut topology, SDC_TOPOLOGY_SUPPLIED | SDC_APPLY)
    };
    let took = r == 0;
    if took {
        crate::pinfo!("controls: project {:?}", mode);
    } else {
        crate::pwarn!("controls: the display topology didn't set (code {})", r);
    }
    took
}

#[cfg(not(windows))]
fn read_radios() -> (bool, bool) {
    (true, false)
}

#[cfg(not(windows))]
fn set_radio(_radio: Radio, _on: bool) -> bool {
    false
}

#[cfg(not(windows))]
fn read_hotspot() -> Option<(bool, u32)> {
    None
}

#[cfg(not(windows))]
fn set_hotspot(_on: bool) -> bool {
    false
}

#[cfg(not(windows))]
fn read_power_mode() -> PowerMode {
    PowerMode::Balanced
}

#[cfg(not(windows))]
fn set_power_mode(_mode: PowerMode) -> bool {
    false
}

#[cfg(not(windows))]
fn read_project() -> (ProjectMode, u32) {
    (ProjectMode::Internal, 1)
}

#[cfg(not(windows))]
fn set_project(_mode: ProjectMode) -> bool {
    false
}
