#![allow(non_snake_case)]

use std::os::raw::{c_void, c_int, c_char};
use crate::{InstanceHandle, Vessel};
use crate::{HINSTANCE, OBJHANDLE};

#[repr(C)]
struct RustModuleCallbacks {
    clbkSimulationStart: extern "C" fn(ctx: *mut c_void, render_mode: c_int),
    clbkSimulationEnd: extern "C" fn(ctx: *mut c_void),
    clbkPreStep: extern "C" fn(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64),
    clbkPostStep: extern "C" fn(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64),
    clbkTimeJump: extern "C" fn(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64),
    clbkFocusChanged: extern "C" fn(ctx: *mut c_void, new_focus: OBJHANDLE , old_focus: OBJHANDLE),
    clbkTimeAccChanged: extern "C" fn(ctx: *mut c_void, new_warp: f64, old_warp: f64),
    clbkNewVessel: extern "C" fn(ctx: *mut c_void, vessel: OBJHANDLE),
    clbkDeleteVessel: extern "C" fn(ctx: *mut c_void, vessel: OBJHANDLE),
    clbkVesselJump: extern "C" fn(ctx: *mut c_void, vessel: OBJHANDLE),
    clbkPause: extern "C" fn(ctx: *mut c_void, pause: bool),
    clbkProcessMouse: extern "C" fn(ctx: *mut c_void, event: MouseEvent, state: KeyboardState, x: u32, y: u32) -> bool,
    clbkProcessKeyboardImmediate: extern "C" fn(ctx: *mut c_void, key_states: KeyStates, sim_running: bool) -> bool,
    clbkProcessKeyboardBuffered: extern "C" fn(ctx: *mut c_void, key: Key, key_states: KeyStates, sim_running: bool) -> bool,
    clbkDestroy: extern "C" fn(ctx: *mut c_void),
}

extern "C" fn clbkSimulationStart(ctx: *mut c_void, render_mode: c_int) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_simulation_start(&ctx.module, RenderMode::from(render_mode));
}

extern "C" fn clbkSimulationEnd(ctx: *mut c_void) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_simulation_end(&ctx.module, );
}

extern "C" fn clbkPreStep(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_pre_step(&ctx.module, simt, simdt, mjd);
}

extern "C" fn clbkPostStep(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_post_step(&ctx.module, simt, simdt, mjd);
}

extern "C" fn clbkTimeJump(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_time_jump(&ctx.module, simt, simdt, mjd);
}

extern "C" fn clbkFocusChanged(ctx: *mut c_void, new_focus: OBJHANDLE , old_focus: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_focus_changed(&ctx.module, Vessel::from_obj(new_focus).unwrap(), Vessel::from_obj(old_focus));
}

extern "C" fn clbkTimeAccChanged(ctx: *mut c_void, new_warp: f64, old_warp: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_time_acc_changed(&ctx.module, new_warp, old_warp);
}

extern "C" fn clbkNewVessel(ctx: *mut c_void, vessel: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_new_vessel(&ctx.module, Vessel::from_obj(vessel).unwrap());
}

extern "C" fn clbkDeleteVessel(ctx: *mut c_void, vessel: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_delete_vessel(&ctx.module, Vessel::from_obj(vessel).unwrap());
}

extern "C" fn clbkVesselJump(ctx: *mut c_void, vessel: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_vessel_jump(&ctx.module, Vessel::from_obj(vessel).unwrap());
}

extern "C" fn clbkPause(ctx: *mut c_void, pause: bool) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_pause(&ctx.module, pause);
}

extern "C" fn clbkProcessMouse(ctx: *mut c_void, event: MouseEvent, state: KeyboardState, x: u32, y: u32) -> bool {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_process_mouse(&ctx.module, event, state, x, y)
}

extern "C" fn clbkProcessKeyboardImmediate(ctx: *mut c_void, key_states: KeyStates, sim_running: bool) -> bool {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_process_keyboard_immediate(&ctx.module, key_states, sim_running)
}

extern "C" fn clbkProcessKeyboardBuffered(ctx: *mut c_void, key: Key, key_states: KeyStates, sim_running: bool) -> bool {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_process_keyboard_buffered(&ctx.module, key, key_states, sim_running)
}

extern "C" fn clbkDestroy(ctx: *mut c_void) {
    unsafe { Box::from_raw(ctx as *mut ModuleAdapter); }
    println!("Module destroyed");
}

type RustModule = *mut c_void;

#[link(name = "orbiter_c")]
extern "C" {
    fn oapic_module_new(cb: RustModuleCallbacks, ctx: *mut c_void, hDLL: HINSTANCE) -> RustModule;
    fn oapic_module_version(module: RustModule) -> c_int;
    fn oapic_module_get_module(module: RustModule) -> HINSTANCE;
    fn oapic_module_get_sim_time(module: RustModule) -> f64;
    fn oapic_module_get_sim_step(module: RustModule) -> f64;
    fn oapic_module_get_sim_mjd(module: RustModule) -> f64;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum RenderMode {
    /// No graphics support
    None,
    /// Fullscreen mode
    Fullscreen,
    /// Window mode
    Window,
}

impl RenderMode {
    pub(crate) fn from(value: c_int) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Fullscreen,
            2 => Self::Window,
            _ => panic!("Unknown RenderMode: {}", value),
        }
    }
}

// TODO: wrap this in an enum
pub type MouseEvent = u32;
// TODO: wrap this in an enum
pub type KeyboardState = u32;
// TODO: wrap this in a struct
pub type KeyStates = *const c_char;
// TODO: wrap this in an enum
pub type Key = u32;

pub trait ModuleCallbacks {
    fn on_simulation_start(&mut self, _module: &Module, _render_mode: RenderMode) {}
    fn on_simulation_end(&mut self, _module: &Module, ) {}
    fn on_pre_step(&mut self, _module: &Module, _simt: f64, _simdt: f64, _mjd: f64) {}
    fn on_post_step(&mut self, _module: &Module, _simt: f64, _simdt: f64, _mjd: f64) {}
    fn on_time_jump(&mut self, _module: &Module, _simt: f64, _simdt: f64, _mjd: f64) {}
    fn on_focus_changed(&mut self, _module: &Module, _new_focus: Vessel, _old_focus: Option<Vessel>) {}
    fn on_time_acc_changed(&mut self, _module: &Module, _new_warp: f64, _old_warp: f64) {}
    fn on_new_vessel(&mut self, _module: &Module, _vessel: Vessel) {}
    fn on_delete_vessel(&mut self, _module: &Module, _vessel: Vessel) {}
    fn on_vessel_jump(&mut self, _module: &Module, _vessel: Vessel) {}
    fn on_pause(&mut self, _module: &Module, _pause: bool) {}
    fn on_process_mouse(&mut self, _module: &Module, _event: MouseEvent, _state: KeyboardState, _x: u32, _y: u32) -> bool { false }
    fn on_process_keyboard_immediate(&mut self, _module: &Module, _key_states: KeyStates, _sim_running: bool) -> bool { false }
    fn on_process_keyboard_buffered(&mut self, _module: &Module, _key: Key, _key_states: KeyStates, _sim_running: bool) -> bool { false }
}

pub(crate) struct ModuleAdapter {
    module: Module,
    callbacks: Box<dyn ModuleCallbacks>,
}

impl ModuleAdapter {
    pub(crate) fn new<M: ModuleCallbacks + 'static>(handle: &InstanceHandle, module: M) {
        let c_callbacks = RustModuleCallbacks {
            clbkSimulationStart,
            clbkSimulationEnd,
            clbkPreStep,
            clbkPostStep,
            clbkTimeJump,
            clbkFocusChanged,
            clbkTimeAccChanged,
            clbkNewVessel,
            clbkDeleteVessel,
            clbkVesselJump,
            clbkPause,
            clbkProcessMouse,
            clbkProcessKeyboardImmediate,
            clbkProcessKeyboardBuffered,
            clbkDestroy,
        };
        let callbacks = Box::new(module);
        let adapter = Box::into_raw(Box::new(ModuleAdapter {
            module: Module(std::ptr::null_mut()),
            callbacks,
        }));

        unsafe {
            let module = oapic_module_new(c_callbacks, adapter as *mut _, handle.into_raw());
            (&mut *adapter).module = Module(module);
        };
    }
}

pub struct Module(RustModule);

impl Module {
    pub fn version(&self) -> u32 {
        unsafe { oapic_module_version(self.0) as u32 }
    }

    pub fn module(&self) -> HINSTANCE {
        unsafe { oapic_module_get_module(self.0) }
    }

    pub fn sim_time(&self) -> f64 {
        unsafe { oapic_module_get_sim_time(self.0) }
    }

    pub fn sim_step(&self) -> f64 {
        unsafe { oapic_module_get_sim_step(self.0) }
    }

    pub fn sim_mjd(&self) -> f64 {
        unsafe { oapic_module_get_sim_mjd(self.0) }
    }
}
