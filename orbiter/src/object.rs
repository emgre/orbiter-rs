use crate::{oapic_VECTOR3, Vector3};
use crate::{Planet, Star, SurfaceBase, Vessel};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use winapi::shared::minwindef::DWORD;

#[doc(hidden)]
pub enum _OBJHANDLE {}
pub type OBJHANDLE = *mut _OBJHANDLE;

/// Common methods for all objects
pub trait ObjectTrait {
    #[doc(hidden)]
    fn handle(&self) -> OBJHANDLE;

    /// Returns the object's name.
    ///
    /// The maximum length of the name is 255 characters.
    fn name(&self) -> String {
        let mut buffer = vec![0; 256];
        unsafe { oapic_oapiGetObjectName(self.handle(), buffer.as_mut_ptr(), buffer.len() as i32) };
        unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .to_string()
    }

    /// Returns the size (mean radius) of the object in meters.
    fn size(&self) -> f64 {
        unsafe { oapic_oapiGetSize(self.handle()) }
    }

    /// Returns the mass of an object in kilograms.
    ///
    /// For vessels, this is the total mass, including the current fuel mass.
    fn mass(&self) -> f64 {
        unsafe { oapic_oapiGetMass(self.handle()) }
    }

    /// Returns the position of an object in the global reference frame.
    ///
    /// The global reference frame is the heliocentric ecliptic system at ecliptic
    /// and equinox of J2000. The units are meters.
    fn global_pos(&self) -> Vector3 {
        let mut pos = oapic_VECTOR3::new();
        unsafe { oapic_oapiGetGlobalPos(self.handle(), &mut pos) };
        pos.into()
    }

    /// Returns the velocity of an object in the global reference frame.
    ///
    /// The global reference frame is the heliocentric ecliptic system at ecliptic
    /// and equinox of J2000. The units are meters per second.
    fn global_velocity(&self) -> Vector3 {
        let mut vel = oapic_VECTOR3::new();
        unsafe { oapic_oapiGetGlobalVel(self.handle(), &mut vel) };
        vel.into()
    }

    /// Returns the distance vector from the reference object to the current object
    /// in the ecliptic reference frame.
    ///
    /// The results are with regards to the ecliptic frame at equinox and ecliptic of J2000.
    /// The units are meters.
    fn relative_position(&self, reference: &dyn ObjectTrait) -> Vector3 {
        let mut pos = oapic_VECTOR3::new();
        unsafe { oapic_oapiGetRelativePos(self.handle(), reference.handle(), &mut pos) };
        pos.into()
    }

    /// Returns the velocity difference of the current object relative to the reference object
    /// in the ecliptic reference frame.
    ///
    /// The results are with regards to the ecliptic frame at equinox and ecliptic of J2000.
    /// The units are meters per second.
    fn relative_velocity(&self, reference: &dyn ObjectTrait) -> Vector3 {
        let mut vel = oapic_VECTOR3::new();
        unsafe { oapic_oapiGetRelativeVel(self.handle(), reference.handle(), &mut vel) };
        vel.into()
    }
}

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

impl ObjectTrait for Object {
    fn handle(&self) -> OBJHANDLE {
        match self {
            Self::Star(star) => star.handle,
            Self::Planet(planet) => planet.handle,
            Self::Vessel(vessel) => vessel.handle(),
            Self::SurfaceBase(base) => base.handle,
        }
    }
}

impl Object {
    fn from(handle: OBJHANDLE) -> Option<Self> {
        if handle.is_null() {
            return None;
        }

        match unsafe { oapic_oapiGetObjectType(handle) } {
            0 => None,
            1 => panic!("Object type OBJTP_GENERIC not expected"),
            2 => panic!("Object type OBJTP_CBODY not expected"),
            3 => Some(Self::Star(Star { handle })),
            4 => Some(Self::Planet(Planet { handle })),
            10 => Some(Self::Vessel(Vessel::from_obj(handle).unwrap())),
            20 => Some(Self::SurfaceBase(SurfaceBase { handle })),
            value => panic!("Object type {} not expected", value),
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
        self.0.next().map(Object::from).flatten()
    }
}

#[link(name = "orbiter_c")]
extern "C" {
    fn oapic_oapiGetObjectByName(name: *const c_char) -> OBJHANDLE;
    fn oapic_oapiGetObjectByIndex(index: c_int) -> OBJHANDLE;
    fn oapic_oapiGetObjectCount() -> DWORD;
    fn oapic_oapiGetObjectType(handle: OBJHANDLE) -> c_int;
    fn oapic_oapiGetObjectName(handle: OBJHANDLE, name: *mut c_char, n: c_int);
    fn oapic_oapiGetSize(handle: OBJHANDLE) -> f64;
    fn oapic_oapiGetMass(handle: OBJHANDLE) -> f64;
    fn oapic_oapiGetGlobalPos(handle: OBJHANDLE, pos: *mut oapic_VECTOR3);
    fn oapic_oapiGetGlobalVel(handle: OBJHANDLE, vel: *mut oapic_VECTOR3);
    fn oapic_oapiGetRelativePos(handle: OBJHANDLE, reference: OBJHANDLE, pos: *mut oapic_VECTOR3);
    fn oapic_oapiGetRelativeVel(handle: OBJHANDLE, reference: OBJHANDLE, vel: *mut oapic_VECTOR3);
}
