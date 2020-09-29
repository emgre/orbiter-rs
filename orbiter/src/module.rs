#![allow(non_snake_case)]

use crate::{InstanceHandle, Key, KeyStates, MouseEvent, Vessel};
use crate::{HINSTANCE, OBJHANDLE};
use std::os::raw::{c_char, c_int, c_void};
use winapi::shared::minwindef::{DWORD, UINT};

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

pub trait ModuleCallbacks {
    fn on_simulation_start(&mut self, _module: &mut Module, _render_mode: RenderMode) {}
    fn on_simulation_end(&mut self, _module: &mut Module) {}
    fn on_pre_step(&mut self, _module: &mut Module, _simt: f64, _simdt: f64, _mjd: f64) {}
    fn on_post_step(&mut self, _module: &mut Module, _simt: f64, _simdt: f64, _mjd: f64) {}
    fn on_time_jump(&mut self, _module: &mut Module, _simt: f64, _simdt: f64, _mjd: f64) {}
    fn on_focus_changed(
        &mut self,
        _module: &mut Module,
        _new_focus: Vessel,
        _old_focus: Option<Vessel>,
    ) {
    }
    fn on_time_acc_changed(&mut self, _module: &mut Module, _new_warp: f64, _old_warp: f64) {}
    fn on_new_vessel(&mut self, _module: &mut Module, _vessel: Vessel) {}
    fn on_delete_vessel(&mut self, _module: &mut Module, _vessel: Vessel) {}
    fn on_vessel_jump(&mut self, _module: &mut Module, _vessel: Vessel) {}
    fn on_pause(&mut self, _module: &mut Module, _pause: bool) {}
    fn on_process_mouse(&mut self, _module: &mut Module, _event: MouseEvent) -> bool {
        false
    }
    fn on_process_keyboard_immediate(
        &mut self,
        _module: &mut Module,
        _key_states: &mut KeyStates,
        _sim_running: bool,
    ) -> bool {
        false
    }
    fn on_process_keyboard_buffered(
        &mut self,
        _module: &mut Module,
        _key: Key,
        _key_states: &mut KeyStates,
        _sim_running: bool,
    ) -> bool {
        false
    }
}

pub(crate) struct ModuleAdapter {
    module: Module,
    callbacks: Box<dyn ModuleCallbacks>,
}

impl ModuleAdapter {
    #[allow(clippy::new_ret_no_self)]
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
            (*adapter).module = Module(module);
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

#[repr(C)]
struct RustModuleCallbacks {
    clbkSimulationStart: extern "C" fn(ctx: *mut c_void, render_mode: c_int),
    clbkSimulationEnd: extern "C" fn(ctx: *mut c_void),
    clbkPreStep: extern "C" fn(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64),
    clbkPostStep: extern "C" fn(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64),
    clbkTimeJump: extern "C" fn(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64),
    clbkFocusChanged: extern "C" fn(ctx: *mut c_void, new_focus: OBJHANDLE, old_focus: OBJHANDLE),
    clbkTimeAccChanged: extern "C" fn(ctx: *mut c_void, new_warp: f64, old_warp: f64),
    clbkNewVessel: extern "C" fn(ctx: *mut c_void, vessel: OBJHANDLE),
    clbkDeleteVessel: extern "C" fn(ctx: *mut c_void, vessel: OBJHANDLE),
    clbkVesselJump: extern "C" fn(ctx: *mut c_void, vessel: OBJHANDLE),
    clbkPause: extern "C" fn(ctx: *mut c_void, pause: bool),
    clbkProcessMouse:
        extern "C" fn(ctx: *mut c_void, event: UINT, state: DWORD, x: DWORD, y: DWORD) -> bool,
    clbkProcessKeyboardImmediate:
        extern "C" fn(ctx: *mut c_void, key_states: *mut c_char, sim_running: bool) -> bool,
    clbkProcessKeyboardBuffered: extern "C" fn(
        ctx: *mut c_void,
        key: DWORD,
        key_states: *mut c_char,
        sim_running: bool,
    ) -> bool,
    clbkDestroy: extern "C" fn(ctx: *mut c_void),
}

extern "C" fn clbkSimulationStart(ctx: *mut c_void, render_mode: c_int) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_simulation_start(&mut ctx.module, RenderMode::from(render_mode));
}

extern "C" fn clbkSimulationEnd(ctx: *mut c_void) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_simulation_end(&mut ctx.module);
}

extern "C" fn clbkPreStep(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_pre_step(&mut ctx.module, simt, simdt, mjd);
}

extern "C" fn clbkPostStep(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_post_step(&mut ctx.module, simt, simdt, mjd);
}

extern "C" fn clbkTimeJump(ctx: *mut c_void, simt: f64, simdt: f64, mjd: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_time_jump(&mut ctx.module, simt, simdt, mjd);
}

extern "C" fn clbkFocusChanged(ctx: *mut c_void, new_focus: OBJHANDLE, old_focus: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_focus_changed(
        &mut ctx.module,
        Vessel::from_obj(new_focus).unwrap(),
        Vessel::from_obj(old_focus),
    );
}

extern "C" fn clbkTimeAccChanged(ctx: *mut c_void, new_warp: f64, old_warp: f64) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_time_acc_changed(&mut ctx.module, new_warp, old_warp);
}

extern "C" fn clbkNewVessel(ctx: *mut c_void, vessel: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_new_vessel(&mut ctx.module, Vessel::from_obj(vessel).unwrap());
}

extern "C" fn clbkDeleteVessel(ctx: *mut c_void, vessel: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_delete_vessel(&mut ctx.module, Vessel::from_obj(vessel).unwrap());
}

extern "C" fn clbkVesselJump(ctx: *mut c_void, vessel: OBJHANDLE) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks
        .on_vessel_jump(&mut ctx.module, Vessel::from_obj(vessel).unwrap());
}

extern "C" fn clbkPause(ctx: *mut c_void, pause: bool) {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_pause(&mut ctx.module, pause);
}

extern "C" fn clbkProcessMouse(
    ctx: *mut c_void,
    event: UINT,
    state: DWORD,
    x: DWORD,
    y: DWORD,
) -> bool {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    let mouse_event = MouseEvent::from(event, state, x, y);
    ctx.callbacks.on_process_mouse(&mut ctx.module, mouse_event)
}

extern "C" fn clbkProcessKeyboardImmediate(
    ctx: *mut c_void,
    key_states: *mut c_char,
    sim_running: bool,
) -> bool {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_process_keyboard_immediate(
        &mut ctx.module,
        &mut KeyStates::from(key_states),
        sim_running,
    )
}

extern "C" fn clbkProcessKeyboardBuffered(
    ctx: *mut c_void,
    key: DWORD,
    key_states: *mut c_char,
    sim_running: bool,
) -> bool {
    let ctx = unsafe { &mut *(ctx as *mut ModuleAdapter) };
    ctx.callbacks.on_process_keyboard_buffered(
        &mut ctx.module,
        Key::from(key as u8),
        &mut KeyStates::from(key_states),
        sim_running,
    )
}

extern "C" fn clbkDestroy(ctx: *mut c_void) {
    unsafe {
        Box::from_raw(ctx as *mut ModuleAdapter);
    }
    println!("Module destroyed");
}
