use crate::OBJHANDLE;
use crate::ObjectTrait;

#[doc(hidden)]
pub enum _VESSEL {}
pub type VESSEL = *mut _VESSEL;

pub trait VesselTrait: ObjectTrait {
    #[doc(hidden)]
    fn vessel_handle(&self) -> VESSEL;

    /// Switches the input focus to a different vessel object.
    ///
    /// Returns the handle to the vessel losing focus, or `None` if focus did not change.
    fn set_focus_vessel(&self) -> Option<Vessel> {
        let handle = unsafe { oapic_oapiSetFocusObject(self.handle()) };
        Vessel::from_obj(handle)
    }

    /// Returns the empty mass (in kg) of the vessel, excluding fuel.
    ///
    /// Do not rely on a constant empty mass. Structural changes (e.g. discarding a
    /// rocket stage) will affect the empty mass.
    fn empty_mass(&self) -> f64 {
        unsafe { oapic_oapiGetEmptyMass(self.handle()) }
    }

    /// Set the empty mass (in kg) of the vessel, excluding fuel.
    ///
    /// Use this function to register structural mass changes, for example as a
    /// result of jettisoning a fuel tank.
    fn set_empty_mass(&mut self, mass: f64) {
        unsafe { oapic_oapiSetEmptyMass(self.handle(), mass) };
    }

    /// Returns current fuel mass (in kg) of the first propellant resource of a vessel.
    #[deprecated]
    fn fuel_mass(&self) -> f64 {
        unsafe { oapic_oapiGetFuelMass(self.handle()) }
    }

    /// Returns maximum fuel capacity (in kg) of the first propellant resource of a vessel.
    #[deprecated]
    fn max_fuel_mass(&self) -> f64 {
        unsafe { oapic_oapiGetMaxFuelMass(self.handle()) }
    }
}

impl<T: VesselTrait> ObjectTrait for T {
    fn handle(&self) -> OBJHANDLE {
        unsafe { oapic_VESSEL_GetHandle(self.vessel_handle()) }
    }
}

pub struct Vessel {
    handle: VESSEL,
}

impl VesselTrait for Vessel {
    fn vessel_handle(&self) -> VESSEL {
        self.handle
    }
}

impl Vessel {
    pub(crate) fn from_obj(obj: OBJHANDLE) -> Option<Vessel> {
        if obj.is_null() {
            return None
        }

        let handle = unsafe { oapic_oapiGetVesselInterface(obj) };
        if !handle.is_null() {
            Some(Vessel { handle })
        } else {
            None
        }
    }

    /// Returns the current focus vessel.
    ///
    /// The focus object is the user-controlled vessel which receives keyboard
    /// and joystick input.
    ///
    /// This method returns a vessel if and only if a simulation session is in progress.
    pub fn focus_vessel() -> Option<Vessel> {
        let handle = unsafe { oapic_oapiGetFocusObject() };
        Vessel::from_obj(handle)
    }
}

#[link(name = "orbiter_c")]
extern "C" {
    fn oapic_oapiGetVesselInterface(obj: OBJHANDLE) -> VESSEL;
    fn oapic_oapiGetFocusObject() -> OBJHANDLE;
    fn oapic_oapiSetFocusObject(vessel: OBJHANDLE) -> OBJHANDLE;
    fn oapic_oapiGetEmptyMass(vessel: OBJHANDLE) -> f64;
    fn oapic_oapiSetEmptyMass(vessel: OBJHANDLE, mass: f64);
    fn oapic_oapiGetFuelMass(vessel: OBJHANDLE) -> f64;
    fn oapic_oapiGetMaxFuelMass(vessel: OBJHANDLE) -> f64;

    fn oapic_VESSEL_GetHandle(vessel: VESSEL) -> OBJHANDLE;
}
