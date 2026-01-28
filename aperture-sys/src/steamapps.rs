//! Interface for ISteamApps
//! (src/steamapps.rs)
//!
//! This implementation uses the *flat* exports from steam_api_flat.h:
//!   SteamAPI_ISteamApps_BIsSubscribed(ISteamApps* self) -> bool
//! etc.
//!

#![allow(non_camel_case_types)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

use core::marker::PhantomData;

// Opaque interface handle (we never dereference it...it’s just a token passed back to Steam).
#[repr(C)]
pub struct ISteamApps {
    _private: [u8; 0],
}

// ---- Core SteamAPI exports (loaded in loader.rs) ----
pub type SteamAPI_InitFn = unsafe extern "C" fn() -> bool;
pub type SteamAPI_ShutdownFn = unsafe extern "C" fn();
pub type SteamAPI_IsSteamRunningFn = unsafe extern "C" fn() -> bool;
pub type SteamAPI_RunCallbacksFn = unsafe extern "C" fn();

// Versioned interface getter export (loaded in loader.rs)
pub type SteamAPI_SteamApps_Fn = unsafe extern "C" fn() -> *mut ISteamApps;

// ---- Flat ISteamApps exports (exported C symbols) ----
pub type ISteamApps_BIsDlcInstalledFn = unsafe extern "C" fn(self_: *mut ISteamApps, app_id: u32) -> bool;
pub type ISteamApps_BIsLowViolenceFn = unsafe extern "C" fn(self_: *mut ISteamApps) -> bool;
pub type ISteamApps_BIsSubscribedFn = unsafe extern "C" fn(self_: *mut ISteamApps) -> bool;
pub type ISteamApps_BIsSubscribedAppFn = unsafe extern "C" fn(self_: *mut ISteamApps, app_id: u32) -> bool;
pub type ISteamApps_BIsSubscribedFromFamilySharingFn = unsafe extern "C" fn(self_: *mut ISteamApps) -> bool;
pub type ISteamApps_BIsSubscribedFromFreeWeekendFn = unsafe extern "C" fn(self_: *mut ISteamApps) -> bool;
pub type ISteamApps_BIsVACBannedFn = unsafe extern "C" fn(self_: *mut ISteamApps) -> bool;
pub type ISteamApps_GetDLCCountFn = unsafe extern "C" fn(self_: *mut ISteamApps) -> u32;

#[derive(Clone, Copy)]
pub struct SteamAppsFns {
    pub bis_dlc_installed: ISteamApps_BIsDlcInstalledFn,
    pub bis_low_violence: ISteamApps_BIsLowViolenceFn,
    pub bis_subscribed: ISteamApps_BIsSubscribedFn,
    pub bis_subscribed_app: ISteamApps_BIsSubscribedAppFn,
    pub bis_subscribed_from_family_sharing: ISteamApps_BIsSubscribedFromFamilySharingFn,
    pub bis_subscribed_from_free_weekend: ISteamApps_BIsSubscribedFromFreeWeekendFn,
    pub bis_vac_banned: ISteamApps_BIsVACBannedFn,
    pub get_dlc_count: ISteamApps_GetDLCCountFn,
}

/// Lightweight wrapper around an `ISteamApps*` plus the loaded flat function pointers.
/// Tied to the lifetime of the loaded function table reference.
pub struct SteamApps<'a> {
    ptr: *mut ISteamApps,
    fns: &'a SteamAppsFns,
    _lt: PhantomData<&'a SteamAppsFns>,
}

impl<'a> SteamApps<'a> {
    pub(crate) unsafe fn from_raw(ptr: *mut ISteamApps, fns: &'a SteamAppsFns) -> Option<Self> {
        (!ptr.is_null()).then(|| Self {
            ptr,
            fns,
            _lt: PhantomData,
        })
    }

    pub fn is_subscribed(&self) -> bool {
        unsafe { (self.fns.bis_subscribed)(self.ptr) }
    }

    pub fn is_low_violence(&self) -> bool {
        unsafe { (self.fns.bis_low_violence)(self.ptr) }
    }

    pub fn is_subscribed_app(&self, app_id: u32) -> bool {
        unsafe { (self.fns.bis_subscribed_app)(self.ptr, app_id) }
    }

    pub fn is_subscribed_from_family_sharing(&self) -> bool {
        unsafe { (self.fns.bis_subscribed_from_family_sharing)(self.ptr) }
    }

    pub fn is_subscribed_from_free_weekend(&self) -> bool {
        unsafe { (self.fns.bis_subscribed_from_free_weekend)(self.ptr) }
    }

    pub fn is_vac_banned(&self) -> bool {
        unsafe { (self.fns.bis_vac_banned)(self.ptr) }
    }

    pub fn get_dlc_count(&self) -> u32 {
        unsafe { (self.fns.get_dlc_count)(self.ptr) }
    }

    pub fn is_dlc_installed(&self, app_id: u32) -> bool {
        unsafe { (self.fns.bis_dlc_installed)(self.ptr, app_id) }
    }
}
