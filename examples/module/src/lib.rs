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

    fn on_focus_changed(&mut self, _module: &Module, new_focus: Object, _old_focus: Option<Object>) {
        debug_string!("Focus is now on {:?}", new_focus);
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
