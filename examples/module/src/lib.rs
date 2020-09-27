use orbiter::*;
use orbiter::module::*;

struct TestModule;

impl ModuleCallbacks for TestModule {
    fn on_simulation_start(&mut self, _module: &Module, _render_mode: RenderMode) {
        println!("Orbiter version: {}", orbiter_version());
        println!("Module version: {}", module_version());
        for obj in Object::all_objects() {
            println!("{:?}", obj);
        }

        println!("{:?}", Object::find_by_name("Neptune"));
    }

    fn on_pre_step(&mut self, _module: &Module, _simt: f64, _simdt: f64, _mjd: f64) {
        let earth = Object::find_by_name("Earth").unwrap();
        debug_string!("Relative pos: {}", Vessel::focus_vessel().unwrap().relative_position(&earth).norm());
    }

    fn on_focus_changed(&mut self, _module: &Module, new_focus: Vessel, _old_focus: Option<Vessel>) {
        debug_string!("Focus is now on {}. Size: {}m. Mass: {}kg", new_focus.name(), new_focus.size(), new_focus.mass());
    }

    fn on_pause(&mut self, module: &Module, pause: bool) {
        debug_string!("Simulation is paused: {}. Current time: {}", pause, module.sim_time());
    }
}

init!(
    fn init(instance) {
        println!("Registering module...");
        let test_module = TestModule;
        instance.register_module(test_module);
        println!("Done!");
    }

    fn exit(_instance) {
        println!("Goodbye from Rust!");
    }
);
