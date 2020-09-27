#include "orbitersdk.h"

extern "C" struct RustModuleCallbacks
{
    void (*clbkSimulationStart)(void* ctx, oapi::Module::RenderMode);
    void (*clbkSimulationEnd)(void* ctx);
    void (*clbkPreStep)(void* ctx, double simt, double simdt, double mjd);
    void (*clbkPostStep)(void* ctx, double simt, double simdt, double mjd);
    void (*clbkTimeJump)(void* ctx, double simt, double simdt, double mjd);
    void (*clbkFocusChanged)(void* ctx, OBJHANDLE new_focus, OBJHANDLE old_focus);
    void (*clbkTimeAccChanged)(void* ctx, double new_warp, double old_warp);
    void (*clbkNewVessel)(void* ctx, OBJHANDLE hVessel);
    void (*clbkDeleteVessel)(void* ctx, OBJHANDLE hVessel);
    void (*clbkVesselJump)(void* ctx, OBJHANDLE hVessel);
    void (*clbkPause)(void* ctx, bool pause);
    bool (*clbkProcessMouse)(void* ctx, UINT event, DWORD state, DWORD x, DWORD y);
    bool (*clbkProcessKeyboardImmediate)(void* ctx, char kstate[256], bool simRunning);
    bool (*clbkProcessKeyboardBuffered)(void* ctx, DWORD key, char kstate[256], bool simRunning);
    bool (*clbkDestroy)(void* ctx);
};

class RustModule final : public oapi::Module
{
private:
    RustModuleCallbacks cb;
    void* ctx;

public:
    RustModule(RustModuleCallbacks cb, void* ctx, HINSTANCE hDLL) :
        oapi::Module(hDLL),
        cb(cb),
        ctx(ctx)
    {}

    ~RustModule()
    {
        cb.clbkDestroy(this->ctx);
    }

    void clbkSimulationStart(RenderMode mode) final
    {
        cb.clbkSimulationStart(this->ctx, mode);
    }

    void clbkSimulationEnd() final
    {
        cb.clbkSimulationEnd(this->ctx);
    }

    void clbkPreStep(double simt, double simdt, double mjd) final
    {
        cb.clbkPreStep(this->ctx, simt, simdt, mjd);
    }

    void clbkPostStep(double simt, double simdt, double mjd) final
    {
        cb.clbkPostStep(this->ctx, simt, simdt, mjd);
    }

    void clbkTimeJump(double simt, double simdt, double mjd) final
    {
        cb.clbkTimeJump(this->ctx, simt, simdt, mjd);
    }

    void clbkFocusChanged(OBJHANDLE new_focus, OBJHANDLE old_focus) final
    {
        cb.clbkFocusChanged(this->ctx, new_focus, old_focus);
    }

    void clbkTimeAccChanged(double new_warp, double old_warp) final
    {
        cb.clbkTimeAccChanged(this->ctx, new_warp, old_warp);
    }

    void clbkNewVessel(OBJHANDLE hVessel) final
    {
        cb.clbkNewVessel(this->ctx, hVessel);
    }

    void clbkDeleteVessel(OBJHANDLE hVessel) final
    {
        cb.clbkDeleteVessel(this->ctx, hVessel);
    }

    void clbkVesselJump(OBJHANDLE hVessel) final
    {
        cb.clbkVesselJump(this->ctx, hVessel);
    }

    void clbkPause(bool pause) final
    {
        cb.clbkPause(this->ctx, pause);
    }

    bool clbkProcessMouse(UINT event, DWORD state, DWORD x, DWORD y) final
    {
        return cb.clbkProcessMouse(this->ctx, event, state, x, y);
    }

    bool clbkProcessKeyboardImmediate(char kstate[256], bool simRunning) final
    {
        return cb.clbkProcessKeyboardImmediate(this->ctx, kstate, simRunning);
    }

    bool clbkProcessKeyboardBuffered(DWORD key, char kstate[256], bool simRunning) final
    {
        return cb.clbkProcessKeyboardBuffered(this->ctx, key, kstate, simRunning);
    }
};

extern "C"
{
    RustModule* oapic_module_new(RustModuleCallbacks cb, void* ctx, HINSTANCE hDLL)
    {
        auto module = new RustModule(cb, ctx, hDLL);
        oapiRegisterModule(module);
        return module;
    }

    int oapic_module_version(RustModule* module) { return module->Version(); }
    HINSTANCE oapic_module_get_module(RustModule* module) { return module->GetModule(); }
    double oapic_module_get_sim_time(RustModule* module) { return module->GetSimTime(); }
    double oapic_module_get_sim_step(RustModule* module) { return module->GetSimStep(); }
    double oapic_module_get_sim_mjd(RustModule* module) { return module->GetSimMJD(); }
}
