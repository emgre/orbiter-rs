#include "orbitersdk.h"

// Linking magic stuff that usually happens because ORBITER_MODULE is defined.
// We do it manually in Rust so here's the C version to keep the `dummy` symbol alive.
void dummy();
extern "C" void oapic_dummy() { dummy(); }

extern "C"
{
    // Generic functions
    int oapic_oapiGetOrbiterVersion() { return oapiGetOrbiterVersion(); }
    int oapic_oapiGetModuleVersion() { return oapiGetModuleVersion(); }
    HINSTANCE oapic_oapiGetOrbiterInstance() { return oapiGetOrbiterInstance(); }
    char* oapic_oapiDebugString() { return oapiDebugString(); }

    // Object manipulation
    OBJHANDLE oapic_oapiGetObjectByName(char* name) { return oapiGetObjectByName(name); }
    OBJHANDLE oapic_oapiGetObjectByIndex(int index) { return oapiGetObjectByIndex(index); }
    DWORD oapic_oapiGetObjectCount() { return oapiGetObjectCount(); }
    int oapic_oapiGetObjectType(OBJHANDLE handle) { return oapiGetObjectType(handle); }
    void oapic_oapiGetObjectName(OBJHANDLE hObj, char* name, int n) { return oapiGetObjectName(hObj, name, n); }
}
