//! Aperture: Safe* Rust binding for Steamworks.
//! 
//! This crate provides safe(er) Rust abstractions over the unsafe
//! aperture-sys crate, which handles dynamic loading of the Steamworks SDK.
//! 

use std::fmt;

pub use aperture_sys as sys;

#[derive(Debug)]
pub enum Error {

    /// Failed to load the Steamworks runtime
    Load(libloading::Error),

    /// SteamAPI_Init() returned false
    SteamInitFailed,

    /// Requested interface isn't available
    InterfaceUnavailable(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Load(e) => write!(f, "failed to load steamworks runtime: {e}"),
            Error::SteamInitFailed => write!(f, "SteamAPI_Init failed"),
            Error::InterfaceUnavailable(name) => write!(f, "Steam interface unavailable: {name}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<libloading::Error> for Error {
    fn from(value: libloading::Error) -> Self {
        Error::Load(value)
    }
}

/// Main entry point. Owns the dynamically-loaded Steam API and shuts it down on drop.
///
/// This is intentionally small.
pub struct Client {
    api: sys::loader::SteamApi,
    initialized: bool,
}

impl Client {
    /// Load the runtime library and initialize SteamAPI.
    pub fn init() -> Result<Self, Error> {
        // All the unsafe stays down in sys; here we just orchestrate.
        let api = unsafe { sys::loader::SteamApi::load()? };

        let ok = unsafe { (api.init)() };
        if !ok {
            return Err(Error::SteamInitFailed);
        }

        Ok(Self {
            api,
            initialized: true,
        })
    }

    /// Access ISteamApps (via aperture-sys).
    pub fn apps(&self) -> Result<Apps<'_>, Error> {
        let inner = unsafe { self.api.steamapps() }
            .ok_or(Error::InterfaceUnavailable("ISteamApps"))?;

        Ok(Apps { inner })
    }

    /// Optional: allow users to manually pump callbacks if you later add callback support.
    pub fn run_callbacks(&self) {
        unsafe { (self.api.run_callbacks)() }
    }

    /// Optional: whether Steam is running.
    pub fn is_steam_running(&self) -> bool {
        unsafe { (self.api.is_running)() }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // Make Drop idempotent; only shutdown if we successfully initialized.
        if self.initialized {
            unsafe { (self.api.shutdown)() };
            self.initialized = false;
        }
    }
}

/// Safe-ish wrapper for ISteamApps.
pub struct Apps<'a> {
    inner: sys::steamapps::SteamApps<'a>,
}

impl Apps<'_> {
    pub fn is_subscribed(&self) -> bool {
        self.inner.is_subscribed()
    }

    pub fn is_low_violence(&self) -> bool {
        self.inner.is_low_violence()
    }

    pub fn is_subscribed_app(&self, app_id: u32) -> bool {
        self.inner.is_subscribed_app(app_id)
    }

    pub fn is_subscribed_from_family_sharing(&self) -> bool {
        self.inner.is_subscribed_from_family_sharing()
    }

    pub fn is_subscribed_from_free_weekend(&self) -> bool {
        self.inner.is_subscribed_from_free_weekend()
    }

    pub fn is_vac_banned(&self) -> bool {
        self.inner.is_vac_banned()
    }

    pub fn dlc_count(&self) -> u32 {
        self.inner.get_dlc_count()
    }

    pub fn is_dlc_installed(&self, app_id: u32) -> bool {
        self.inner.is_dlc_installed(app_id)
    }
}