use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use winapi::shared::minwindef::DWORD;

pub mod module;

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

#[link(name = "orbiter_c")]
extern "C" {
    fn oapic_dummy();

    // Generic functions
    fn oapic_oapiGetOrbiterVersion() -> c_int;
    fn oapic_oapiGetModuleVersion() -> c_int;
    fn oapic_oapiGetOrbiterInstance() -> HINSTANCE;
    fn oapic_oapiDebugString() -> *mut c_char;

    // Object manipulation
    fn oapic_oapiGetObjectByName(name: *const c_char) -> OBJHANDLE;
    fn oapic_oapiGetObjectByIndex(index: c_int) -> OBJHANDLE;
    fn oapic_oapiGetObjectCount() -> DWORD;
    fn oapic_oapiGetObjectType(handle: OBJHANDLE) -> c_int;
    fn oapic_oapiGetObjectName(handle: OBJHANDLE, name: *mut c_char, n: c_int);
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

type OBJHANDLE = *mut c_void;

pub enum Object {
    /// A star
    Star(Star),
    /// A planet
    ///
    /// Used for all celestial bodies that are not stars, including moons, comets, etc.
    Planet(Planet),
    /// A vessel
    ///
    /// A spacecraft, a space station, etc.
    Vessel(Vessel),
    /// A surface base
    ///
    /// i.e. a spaceport
    SurfaceBase(SurfaceBase),
}

impl Object {
    fn from(handle: OBJHANDLE) -> Option<Self> {
        match unsafe { oapic_oapiGetObjectType(handle) } {
            0 => None,
            1 => panic!("Object type OBJTP_GENERIC not expected"),
            2 => panic!("Object type OBJTP_CBODY not expected"),
            3 => Some(Self::Star(Star{handle})),
            4 => Some(Self::Planet(Planet{handle})),
            10 => Some(Self::Vessel(Vessel{handle})),
            20 => Some(Self::SurfaceBase(SurfaceBase{handle})),
            _ => panic!("Object type {} not expected"),
        }
    }

    fn handle(&self) -> OBJHANDLE {
        match self {
            Self::Star(star) => star.handle,
            Self::Planet(planet) => planet.handle,
            Self::Vessel(vessel) => vessel.handle,
            Self::SurfaceBase(base) => base.handle,
        }
    }

    /// Retrieves all the objects of the current simulation.
    pub fn all_objects() -> impl Iterator<Item = Object> {
        let count = unsafe { oapic_oapiGetObjectCount() };
        let mut objects = Vec::with_capacity(count as usize);
        for i in 0..count {
            let object = unsafe { oapic_oapiGetObjectByIndex(i as i32) };
            objects.push(object);
        }
        ObjectIterator(objects.into_iter())
    }

    /// Retrieves an object by its name.
    pub fn find_by_name(name: &str) -> Option<Object> {
        let name = CString::new(name).unwrap();
        let handle = unsafe { oapic_oapiGetObjectByName(name.as_ptr()) };
        Object::from(handle)
    }

    /// Returns the object's name.
    ///
    /// The maximum length of the name is 255 characters.
    pub fn name(&self) -> String {
        let mut buffer = vec![0; 256];
        unsafe { oapic_oapiGetObjectName(self.handle(), buffer.as_mut_ptr(), buffer.len() as i32) };
        unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().to_string()
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let object_type = match self {
            Self::Star(_) => "Star",
            Self::Planet(_) => "Planet",
            Self::Vessel(_) => "Vessel",
            Self::SurfaceBase(_) => "SurfaceBase",
        };
        f.write_fmt(format_args!("{}({})", object_type, self.name()))
    }
}

struct ObjectIterator(std::vec::IntoIter<OBJHANDLE>);

impl Iterator for ObjectIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|obj| Object::from(obj)).flatten()
    }
}

pub struct Star {
    handle: OBJHANDLE,
}

pub struct Planet {
    handle: OBJHANDLE,
}

pub struct Vessel {
    handle: OBJHANDLE,
}

pub struct SurfaceBase {
    handle: OBJHANDLE,
}
