#include "orbitersdk.h"

extern "C"
{
    VESSEL* oapic_oapiGetVesselInterface(OBJHANDLE hVessel) { return oapiGetVesselInterface(hVessel); }
    OBJHANDLE oapic_oapiGetFocusObject() { return oapiGetFocusObject(); }
    OBJHANDLE oapic_oapiSetFocusObject(OBJHANDLE hVessel) { return oapiSetFocusObject(hVessel); }
    double oapic_oapiGetEmptyMass(OBJHANDLE hVessel) { return oapiGetEmptyMass(hVessel); }
    void oapic_oapiSetEmptyMass(OBJHANDLE hVessel, double mass) { oapiSetEmptyMass(hVessel, mass); }
    double oapic_oapiGetFuelMass(OBJHANDLE hVessel) { return oapiGetFuelMass(hVessel); }
    double oapic_oapiGetMaxFuelMass(OBJHANDLE hVessel) { return oapiGetMaxFuelMass(hVessel); }
}

extern "C"
{
    OBJHANDLE oapic_VESSEL_GetHandle(VESSEL* hVessel) { return hVessel->GetHandle(); }
}
