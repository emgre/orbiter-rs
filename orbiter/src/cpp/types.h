#pragma once

#include "orbitersdk.h"

extern "C"
{
    typedef struct oapic_VECTOR3
    {
        double x;
        double y;
        double z;
    } oapic_VECTOR3;
}

inline void convert(VECTOR3& from, oapic_VECTOR3* to)
{
    to->x = from.x;
    to->y = from.y;
    to->z = from.z;
}
