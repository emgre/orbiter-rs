#include "orbitersdk.h"

// Linking magic stuff that usually happens because ORBITER_MODULE is defined.
// We do it manually in Rust so here's the C version to keep the `dummy` symbol alive.
void dummy();
extern "C" void oapic_dummy() { dummy(); }

extern "C"
{
    int oapic_oapiGetOrbiterVersion() { return oapiGetOrbiterVersion(); }
    int oapic_oapiGetModuleVersion() { return oapiGetModuleVersion(); }
    HINSTANCE oapic_oapiGetOrbiterInstance() { return oapiGetOrbiterInstance(); }
    char* oapic_oapiDebugString() { return oapiDebugString(); }
}
