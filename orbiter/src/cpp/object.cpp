#include "orbitersdk.h"
#include "types.h"

extern "C"
{
    OBJHANDLE oapic_oapiGetObjectByName(char* name) { return oapiGetObjectByName(name); }
    OBJHANDLE oapic_oapiGetObjectByIndex(int index) { return oapiGetObjectByIndex(index); }
    DWORD oapic_oapiGetObjectCount() { return oapiGetObjectCount(); }
    int oapic_oapiGetObjectType(OBJHANDLE hObj) { return oapiGetObjectType(hObj); }
    void oapic_oapiGetObjectName(OBJHANDLE hObj, char* name, int n) { return oapiGetObjectName(hObj, name, n); }
    double oapic_oapiGetSize(OBJHANDLE hObj) { return oapiGetSize(hObj); }
    double oapic_oapiGetMass(OBJHANDLE hObj) { return oapiGetMass(hObj); }
    void oapic_oapiGetGlobalPos(OBJHANDLE hObj, oapic_VECTOR3* pos) {
        VECTOR3 result;
        oapiGetGlobalPos(hObj, &result);
        convert(result, pos);
    };
    void oapic_oapiGetGlobalVel(OBJHANDLE hObj, oapic_VECTOR3* vel) {
        VECTOR3 result;
        oapiGetGlobalVel(hObj, &result);
        convert(result, vel);
    };
    void oapic_oapiGetRelativePos(OBJHANDLE hObj, OBJHANDLE hRef, oapic_VECTOR3* pos) {
        VECTOR3 result;
        oapiGetRelativePos(hObj, hRef, &result);
        convert(result, pos);
    };
    void oapic_oapiGetRelativeVel(OBJHANDLE hObj, OBJHANDLE hRef, oapic_VECTOR3* vel) {
        VECTOR3 result;
        oapiGetRelativeVel(hObj, hRef, &result);
        convert(result, vel);
    };
}
