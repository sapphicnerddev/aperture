#![allow(non_camel_case_types)]
#![allow(unsafe_op_in_unsafe_fn)]

use libloading::Library;

// Type defining the steamapi funcs we need
type SteamAPI_InitFn = unsafe extern "C" fn() -> bool;
type SteamAPI_ShutdownFn = unsafe extern "C" fn();
type SteamAPI_IsSteamRunningFn = unsafe extern "C" fn() -> bool;
type SteamAPI_RunCallbacksFn = unsafe extern "C" fn();

pub struct SteamApi {
    _lib: Library,
    pub init: SteamAPI_InitFn,
    pub shutdown: SteamAPI_ShutdownFn,
    pub is_running: SteamAPI_IsSteamRunningFn,
    pub run_callbacks: SteamAPI_RunCallbacksFn,
}

impl SteamApi {
    pub unsafe fn load() -> Result<Self, libloading::Error> {
        // Per-OS cfg just makes the most sense here
        #[cfg(target_os = "windows")]
        let lib = unsafe { Library::new("steam_api64.dll") }?;

        #[cfg(target_os = "linux")]
        let lib = unsafe { Library::new("libsteam_api.so") }?;

        #[cfg(target_os = "macos")]
        let lib = unsafe { Library::new("libsteam_api.dylib") }?;

        let init = *lib.get::<SteamAPI_InitFn>(b"SteamAPI_Init\0")?;
        let shutdown = *lib.get::<SteamAPI_ShutdownFn>(b"SteamAPI_Shutdown\0")?;
        let is_running = *lib.get::<SteamAPI_IsSteamRunningFn>(b"SteamAPI_IsSteamRunning\0")?;
        let run_callbacks = *lib.get::<SteamAPI_RunCallbacksFn>(b"SteamAPI_RunCallbacks\0")?;

        Ok(SteamApi {
            _lib: lib,
            init,
            shutdown,
            is_running,
            run_callbacks,
        })
    }
}
