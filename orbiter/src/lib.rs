use std::ffi::CString;
use std::os::raw::{c_char, c_int};

pub mod module;
mod object;
mod vessel;

/// Defines the required functions to make your DLL available to Orbiter.
///
/// This macro takes two functions in parameter. The `init` function gets called
/// when Orbiter loads your DLL. It provides a [`InstanceHandle`] that you can
/// use to register a module, a MFD, etc. The `exit` function get called when
/// Orbiter unloads your DLL. It should contain cleanup code, such as releasing
/// resources and unregistering MFDs, etc.
///
/// **You must call this macro once at the root of your crate.**
///
/// # Examples
///
/// ```
/// init!(
///     fn init(instance) {
///         println!("Hello from Rust!");
///         // Register a module, etc.
///     }
///
///     fn exit(_instance) {
///         println!("Goodbye from Rust!");
///         // Do the required cleanup
///     }
///);
/// ```
///
/// [`InstanceHandle`]: ./struct.InstanceHandle.html
#[macro_export]
macro_rules! init {
    ( fn init($module_init_ident:ident) $body_init:block fn exit($module_exit_ident:ident) $body_exit:block) => {
        // This is a required symbol that returns the date of build
        #[no_mangle]
        pub extern "C" fn ModuleDate() -> *const std::os::raw::c_char {
            orbiter::get_module_date()
        }

        // This is called when the module is loaded by Orbiter
        #[no_mangle]
        pub unsafe extern "C" fn InitModule(module: orbiter::HINSTANCE)
        {
            let mut $module_init_ident = orbiter::InstanceHandle::from(module);
            $body_init
        }

        // This is called before the module is unloaded by Orbiter
        #[no_mangle]
        pub extern "C" fn ExitModule(module: orbiter::HINSTANCE)
        {
            let mut $module_exit_ident = orbiter::InstanceHandle::from(module);
            $body_exit
        }
    }
}

lazy_static::lazy_static! {
    static ref BUILD_DATE: CString = CString::new(env!("ORBITER_DATE")).unwrap();
}

#[doc(hidden)]
pub fn get_module_date() -> *const std::os::raw::c_char {
    // This is to ensure that the `dummy` symbol of OrbiterSDK is kept.
    // It is used to generate the GetModuleVersion symbol
    unsafe { oapic_dummy(); }
    BUILD_DATE.as_ptr()
}

/// Instance handle from the Windows API
pub use winapi::shared::minwindef::HINSTANCE;

#[derive(Copy, Clone, PartialEq)]
pub struct InstanceHandle(HINSTANCE);

impl InstanceHandle {
    #[doc(hidden)]
    pub fn from(ptr: HINSTANCE) -> Self {
        Self(ptr)
    }

    pub fn into_raw(self) -> HINSTANCE {
        self.0
    }

    pub fn register_module<M: module::ModuleCallbacks + 'static>(&mut self, module: M) {
        module::ModuleAdapter::new(self, module)
    }
}

/// Display a string in the lower left corner of the viewport.
///
/// This macro uses the exact same parameters as the [`format!`] macro of the
/// standard library.
///
/// Due to how Orbiter handles this string, its length is limited to 255 characters.
/// The Rust code truncates the formatted string to 255 characters to make sure that
/// no buffer overflow occurs.
///
/// **This function should only be used for debugging purposes.**
///
/// # Examples
///
/// ```
/// use orbiter::debug_string;
///
/// let some_value = 42;
/// debug_string!("This is my value: {}", 42);
/// ```
///
/// [`format!`]: https://doc.rust-lang.org/std/fmt/index.html
#[macro_export]
macro_rules! debug_string {
    ($($args:tt)+) => {
        orbiter::_debug_string(format!($($args)*));
    }
}

#[doc(hidden)]
pub fn _debug_string(mut text: String) {
    text.truncate(255);
    let encoded = std::ffi::CString::new(text).unwrap();
    let bytes = encoded.as_bytes_with_nul();
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), oapic_oapiDebugString() as *mut _, bytes.len()); }
}

/// Returns the version number of the Orbiter core system.
///
/// Orbiter version numbers are derived from the build date.
/// The version number is constructed as `(year%100)*10000 + month*100 + day`,
/// resulting in a decimal version number of the form `YYMMDD`
pub fn orbiter_version() -> u32 {
    unsafe { oapic_oapiGetOrbiterVersion() as u32 }
}

/// Returns the API version number against which the module was linked.
///
/// Orbiter version numbers are derived from the build date.
/// The version number is constructed as `(year%100)*10000 + month*100 + day`,
/// resulting in a decimal version number of the form `YYMMDD`
pub fn module_version() -> u32 {
    unsafe { oapic_oapiGetModuleVersion() as u32 }
}

/// Returns the instance handle for the running Orbiter application.
///
/// This handle might be useful for interating with the Windows API
/// (e.g. creating a window).
pub fn orbiter_instance() -> HINSTANCE {
    unsafe { oapic_oapiGetOrbiterInstance() }
}

#[link(name = "orbiter_c")]
extern "C" {
    fn oapic_dummy();
    fn oapic_oapiGetOrbiterVersion() -> c_int;
    fn oapic_oapiGetModuleVersion() -> c_int;
    fn oapic_oapiGetOrbiterInstance() -> HINSTANCE;
    fn oapic_oapiDebugString() -> *mut c_char;
}

pub type Vector3 = nalgebra::Vector3<f64>;

#[repr(C)]
struct oapic_VECTOR3 {
    x: f64,
    y: f64,
    z: f64,
}

impl oapic_VECTOR3 {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl From<oapic_VECTOR3> for Vector3 {
    fn from(from: oapic_VECTOR3) -> Self {
        Vector3::new(from.x, from.y, from.z)
    }
}

use crate::object::OBJHANDLE;
pub use crate::object::{Object, ObjectTrait};

pub struct Star {
    handle: OBJHANDLE,
}

pub struct Planet {
    handle: OBJHANDLE,
}

pub use crate::vessel::{Vessel, VesselTrait};

pub struct SurfaceBase {
    handle: OBJHANDLE,
}
