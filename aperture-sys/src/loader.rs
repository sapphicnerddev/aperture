#![allow(non_camel_case_types)]
#![allow(unsafe_op_in_unsafe_fn)]

use libloading::{Library, Symbol};

use crate::steamapps::{
    ISteamApps, SteamApps, SteamAppsFns, SteamAPI_SteamApps_Fn, SteamAPI_InitFn,
    SteamAPI_IsSteamRunningFn, SteamAPI_RunCallbacksFn, SteamAPI_ShutdownFn,
};

pub struct SteamApi {
    _lib: Library,

    pub init: SteamAPI_InitFn,
    pub shutdown: SteamAPI_ShutdownFn,
    pub is_running: SteamAPI_IsSteamRunningFn,
    pub run_callbacks: SteamAPI_RunCallbacksFn,

    /// Exported interface getter (versioned). In SDK 1.63 the default accessor maps to v008.
    pub steamapps_getter_name: &'static str,
    pub steamapps_getter: SteamAPI_SteamApps_Fn,

    /// Flat exports for ISteamApps methods.
    pub apps: SteamAppsFns,
}

impl SteamApi {
    pub unsafe fn load() -> Result<Self, libloading::Error> {
        // Per-OS cfg just makes the most sense here
        #[cfg(target_os = "windows")]
        let lib = Library::new("steam_api64.dll")?;

        #[cfg(target_os = "linux")]
        let lib = Library::new("libsteam_api.so")?;

        #[cfg(target_os = "macos")]
        let lib = Library::new("libsteam_api.dylib")?;

        // Core
        let init = *lib.get::<SteamAPI_InitFn>(b"SteamAPI_Init\0")?;
        let shutdown = *lib.get::<SteamAPI_ShutdownFn>(b"SteamAPI_Shutdown\0")?;
        let is_running = *lib.get::<SteamAPI_IsSteamRunningFn>(b"SteamAPI_IsSteamRunning\0")?;
        let run_callbacks = *lib.get::<SteamAPI_RunCallbacksFn>(b"SteamAPI_RunCallbacks\0")?;

        // Helper: try to load a symbol by name (null-terminated)
        unsafe fn get_sym<T>(lib: &Library, name: &'static [u8]) -> Result<T, libloading::Error>
        where
            T: Copy,
        {
            let s: Symbol<T> = lib.get(name)?;
            Ok(*s)
        }

        // v008 is common for Steamworks SDK 1.63
        let (steamapps_getter_name, steamapps_getter): (&'static str, SteamAPI_SteamApps_Fn) = {
            // NOTE: all names must be null-terminated
            let candidates: [(&'static str, &'static [u8]); 3] = [
                ("SteamAPI_SteamApps_v010", b"SteamAPI_SteamApps_v010\0"),
                ("SteamAPI_SteamApps_v009", b"SteamAPI_SteamApps_v009\0"),
                ("SteamAPI_SteamApps_v008", b"SteamAPI_SteamApps_v008\0"),
            ];

            let mut last_err: Option<libloading::Error> = None;

            let mut found: Option<(&'static str, SteamAPI_SteamApps_Fn)> = None;
            for (label, sym) in candidates {
                match get_sym::<SteamAPI_SteamApps_Fn>(&lib, sym) {
                    Ok(f) => {
                        found = Some((label, f));
                        break;
                    }
                    Err(e) => last_err = Some(e),
                }
            }

            match found {
                Some(v) => v,
                None => {
                    // Fall back to the last error so the caller sees a useful libloading error.
                    return Err(last_err.expect("no SteamApps candidates tried"));
                }
            }
        };

        // Flat ISteamApps exports (these are exported C symbols)
        let apps = SteamAppsFns {
            bis_dlc_installed: get_sym(&lib, b"SteamAPI_ISteamApps_BIsDlcInstalled\0")?,
            bis_low_violence: get_sym(&lib, b"SteamAPI_ISteamApps_BIsLowViolence\0")?,
            bis_subscribed: get_sym(&lib, b"SteamAPI_ISteamApps_BIsSubscribed\0")?,
            bis_subscribed_app: get_sym(&lib, b"SteamAPI_ISteamApps_BIsSubscribedApp\0")?,
            bis_subscribed_from_family_sharing: get_sym(
                &lib,
                b"SteamAPI_ISteamApps_BIsSubscribedFromFamilySharing\0",
            )?,
            bis_subscribed_from_free_weekend: get_sym(
                &lib,
                b"SteamAPI_ISteamApps_BIsSubscribedFromFreeWeekend\0",
            )?,
            bis_vac_banned: get_sym(&lib, b"SteamAPI_ISteamApps_BIsVACBanned\0")?,
            get_dlc_count: get_sym(&lib, b"SteamAPI_ISteamApps_GetDLCCount\0")?,
        };

        Ok(SteamApi {
            _lib: lib,
            init,
            shutdown,
            is_running,
            run_callbacks,
            steamapps_getter_name,
            steamapps_getter,
            apps,
        })
    }

    /// Get a SteamApps wrapper. Must call this only after SteamAPI_Init() succeeds.
    pub unsafe fn steamapps(&self) -> Option<SteamApps<'_>> {
        let ptr: *mut ISteamApps = (self.steamapps_getter)();
        SteamApps::from_raw(ptr, &self.apps)
    }
}
