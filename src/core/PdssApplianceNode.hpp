#pragma once
#include <string>
#include <cmath>
#include <stdexcept>

namespace bugsappliance {

struct PdssIntent {
    std::string profile;     // e.g. "bedbug_cool_surface_low"
    double intensity01;      // [0,1]
    double duty01;           // [0,1] fraction of cycle ON
    double period_seconds;   // total cycle period
};

struct ApplianceCorridors {
    double rnoise_safe,   rnoise_gold,   rnoise_hard;
    double rthermal_safe, rthermal_gold, rthermal_hard;
    double rvib_safe,     rvib_gold,     rvib_hard;
};

struct ApplianceTelemetry {
    double dBA;            // measured or inferred sound level
    double dT_surface_C;   // surface temperature rise
    double vib_rms;        // vibration proxy
};

struct RiskCoords {
    double rnoise;
    double rthermal;
    double rvib;
};

struct NodeResidual {
    RiskCoords r;
    double Vt;
};

inline double normalize_coord(double x,
                              double safe,
                              double gold,
                              double hard)
{
    if (hard <= gold || gold <= safe) {
        throw std::invalid_argument("Invalid corridor bands");
    }
    if (x <= safe) return 0.0;
    if (x >= hard) return 1.0;
    if (x <= gold) {
        return (x - safe) / (gold - safe) * 0.5;
    }
    return 0.5 + (x - gold) / (hard - gold) * 0.5;
}

inline RiskCoords compute_risk(const ApplianceTelemetry& m,
                               const ApplianceCorridors& c)
{
    RiskCoords r;
    r.rnoise   = normalize_coord(m.dBA,          c.rnoise_safe,
                                 c.rnoise_gold,  c.rnoise_hard);
    r.rthermal = normalize_coord(m.dT_surface_C, c.rthermal_safe,
                                 c.rthermal_gold,c.rthermal_hard);
    r.rvib     = normalize_coord(m.vib_rms,      c.rvib_safe,
                                 c.rvib_gold,    c.rvib_hard);
    return r;
}

inline double lyapunov(const RiskCoords& r,
                       double wnoise,
                       double wthermal,
                       double wvib)
{
    return wnoise*std::pow(r.rnoise,2)
         + wthermal*std::pow(r.rthermal,2)
         + wvib*std::pow(r.rvib,2);
}

struct SafeStepDecision {
    bool ok;
    bool derate;
};

inline SafeStepDecision safestep(const NodeResidual& prev,
                                 const NodeResidual& next)
{
    // Hard corridor: any r_j >= 1 blocks actuation.
    if (next.r.rnoise >= 1.0 || next.r.rthermal >= 1.0 ||
        next.r.rvib   >= 1.0) {
        return {false, true};
    }
    // Lyapunov residual must not increase.
    if (next.Vt > prev.Vt) {
        return {false, true};
    }
    return {true, false};
}

} // namespace bugsappliance
