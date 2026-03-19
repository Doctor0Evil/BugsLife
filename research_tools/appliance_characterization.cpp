/**
 * @file appliance_characterization.cpp
 * @brief BugsLife PDSS - Systematic Appliance Emission Mapping Framework
 * @version 0.9.2-ecotribute
 * @license MIT + Ecological Safety Covenant
 * 
 * @description
 * This C++ framework automates the bench-testing required to populate the
 * `appliance_registry` (Lua) and `CorridorBands` (Rust). It drives appliances
 * through controlled sequences while measuring multi-modal emissions (noise,
 * thermal, airflow, power) to establish empirical safety corridors.
 * 
 * @safety_critical
 * - Includes hardware emergency stop (E-Stop) logic.
 * - Enforces human safety limits (noise/thermal) even during testing.
 * - Outputs data signed with test-session ID for auditability.
 * 
 * @research_goal
 * Close the "Appliance Characterization" gap by generating high-fidelity
 * maps of intensity_pct -> physical_output for legacy hardware.
 */

#include <iostream>
#include <vector>
#include <chrono>
#include <thread>
#include <atomic>
#include <mutex>
#include <fstream>
#include <sstream>
#include <cmath>
#include <cstring>
#include <algorithm>
#include <map>
#include <functional>

// ============================================================================
// CONSTANTS & SAFETY LIMITS (Matches Rust Kernel Hard Limits)
// ============================================================================

constexpr double SAFE_NOISE_MAX_DB = 85.0;       // OSHA/ISO limit
constexpr double SAFE_TEMP_MAX_C = 45.0;         // Surface touch limit
constexpr double SAFE_POWER_MAX_W = 2000.0;      // Circuit breaker margin
constexpr double TEST_DURATION_SEC = 60.0;       // Max continuous test per step
constexpr double EMERGENCY_STOP_THRESHOLD = 1.2; // 120% of hard limit triggers E-Stop

// Risk Coordinate IDs (Must match Rust VarId enum)
enum class VarId : uint8_t {
    RNOISE_HUMAN = 0x01,
    RLIGHT_EYE = 0x02,
    RODOR_TOX = 0x03,
    RTHERMAL_BODY = 0x04,
    RSTRUCT_VIB = 0x05,
    RFREQ_ULTRASONIC = 0x06,
    REM_FIELD = 0x07,
    RAIRFLOW = 0x08,
    RMOISTURE = 0x09,
    RMULTIMODAL = 0x0A
};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

struct SensorReading {
    uint64_t timestamp_ms;
    double noise_db;
    double temperature_c;
    double airflow_cfm;
    double power_w;
    double vibration_g;
    double humidity_pct;
    
    SensorReading() : timestamp_ms(0), noise_db(0), temperature_c(0), 
                      airflow_cfm(0), power_w(0), vibration_g(0), humidity_pct(0) {}
};

struct ApplianceStepResult {
    double intensity_pct;
    double duty_cycle;
    std::vector<SensorReading> samples;
    SensorReading avg;
    SensorReading max;
    SensorReading min;
    bool safety_violation;
    std::string violation_reason;
    
    ApplianceStepResult() : intensity_pct(0), duty_cycle(0), safety_violation(false) {}
};

struct ApplianceProfile {
    std::string appliance_id;
    std::string appliance_type;
    std::string manufacturer;
    std::string model_number;
    std::vector<ApplianceStepResult> test_results;
    std::string test_session_id;
    uint64_t test_timestamp;
    double ker_knowledge_score; // Post-test K score estimate
};

// ============================================================================
// HARDWARE ABSTRACTION LAYER (HAL) - Mocked for Portability
// ============================================================================

class SensorHub {
public:
    virtual ~SensorHub() = default;
    virtual SensorReading read_current() = 0;
    virtual bool is_human_present() = 0; // PIR/Mic array check
};

class MockSensorHub : public SensorHub {
public:
    SensorReading read_current() override {
        SensorReading r;
        r.timestamp_ms = get_timestamp_ms();
        // Simulate sensor noise
        r.noise_db = 30.0 + (rand() % 10); 
        r.temperature_c = 22.0 + (rand() % 5);
        r.power_w = 0.0; // Updated by ActuatorController
        return r;
    }
    bool is_human_present() override { return false; } // Override for lab safety
private:
    uint64_t get_timestamp_ms() {
        return std::chrono::duration_cast<std::chrono::milliseconds>(
            std::chrono::system_clock::now().time_since_epoch()).count();
    }
};

class ActuatorController {
public:
    virtual ~ActuatorController() = default;
    virtual void set_intensity(double pct) = 0;
    virtual void set_duty_cycle(double pct) = 0;
    virtual void emergency_stop() = 0;
    virtual double get_power_draw() = 0;
};

class MockActuatorController : public ActuatorController {
public:
    void set_intensity(double pct) override { 
        std::cout << "[ACTUATOR] Setting intensity to " << (pct * 100) << "%" << std::endl; 
    }
    void set_duty_cycle(double pct) override { 
        std::cout << "[ACTUATOR] Setting duty cycle to " << (pct * 100) << "%" << std::endl; 
    }
    void emergency_stop() override { 
        std::cout << "[ACTUATOR] EMERGENCY STOP TRIGGERED" << std::endl; 
    }
    double get_power_draw() override { return 0.0; } // Mocked
};

// ============================================================================
// SAFETY WATCHDOG THREAD
// ============================================================================

class SafetyWatchdog {
private:
    std::atomic<bool>& stop_flag;
    std::atomic<bool>& emergency_stop_flag;
    SensorHub& sensors;
    ActuatorController& actuators;
    std::thread watchdog_thread;
    
    void monitor_loop() {
        while (!stop_flag.load()) {
            SensorReading r = sensors.read_current();
            
            // Check Hard Limits
            if (r.noise_db > SAFE_NOISE_MAX_DB * EMERGENCY_STOP_THRESHOLD) {
                std::cerr << "[WATCHDOG] CRITICAL: Noise limit exceeded (" << r.noise_db << " dB)" << std::endl;
                emergency_stop_flag.store(true);
            }
            
            if (r.temperature_c > SAFE_TEMP_MAX_C * EMERGENCY_STOP_THRESHOLD) {
                std::cerr << "[WATCHDOG] CRITICAL: Thermal limit exceeded (" << r.temperature_c << " C)" << std::endl;
                emergency_stop_flag.store(true);
            }
            
            // Check Human Presence
            if (sensors.is_human_present()) {
                std::cerr << "[WATCHDOG] CRITICAL: Human detected in test zone." << std::endl;
                emergency_stop_flag.store(true);
            }
            
            if (emergency_stop_flag.load()) {
                actuators.emergency_stop();
                std::this_thread::sleep_for(std::chrono::milliseconds(100));
            } else {
                std::this_thread::sleep_for(std::chrono::milliseconds(50));
            }
        }
    }

public:
    SafetyWatchdog(std::atomic<bool>& stop, std::atomic<bool>& e_stop, 
                   SensorHub& sens, ActuatorController& act)
        : stop_flag(stop), emergency_stop_flag(e_stop), sensors(sens), actuators(act) {
        watchdog_thread = std::thread(&SafetyWatchdog::monitor_loop, this);
    }
    
    ~SafetyWatchdog() {
        stop_flag.store(true);
        if (watchdog_thread.joinable()) watchdog_thread.join();
    }
};

// ============================================================================
// CHARACTERIZATION ENGINE
// ============================================================================

class CharacterizationEngine {
private:
    SensorHub& sensors;
    ActuatorController& actuators;
    std::atomic<bool>& emergency_stop_flag;
    std::mutex log_mutex;
    
    SensorReading compute_statistics(const std::vector<SensorReading>& samples, bool is_max) {
        SensorResult result;
        if (samples.empty()) return SensorReading();
        
        SensorReading stat;
        // Simplified stat computation for brevity
        for (const auto& s : samples) {
            stat.noise_db += s.noise_db;
            stat.temperature_c += s.temperature_c;
            stat.power_w += s.power_w;
        }
        double count = static_cast<double>(samples.size());
        stat.noise_db /= count;
        stat.temperature_c /= count;
        stat.power_w /= count;
        
        if (is_max) {
            // Find max in original samples
            for (const auto& s : samples) {
                if (s.noise_db > stat.noise_db) stat.noise_db = s.noise_db;
                if (s.temperature_c > stat.temperature_c) stat.temperature_c = s.temperature_c;
            }
        }
        return stat;
    }

public:
    CharacterizationEngine(SensorHub& s, ActuatorController& a, std::atomic<bool>& e_stop)
        : sensors(s), actuators(a), emergency_stop_flag(e_stop) {}
    
    ApplianceStepResult run_intensity_step(double intensity, double duty_cycle, double duration_sec) {
        ApplianceStepResult result;
        result.intensity_pct = intensity;
        result.duty_cycle = duty_cycle;
        
        std::cout << "[TEST] Starting step: Intensity=" << intensity << " Duty=" << duty_cycle << std::endl;
        
        // Set Actuator
        actuators.set_intensity(intensity);
        actuators.set_duty_cycle(duty_cycle);
        
        // Stabilization period (5 seconds)
        std::this_thread::sleep_for(std::chrono::seconds(5));
        
        // Sampling period
        auto start = std::chrono::steady_clock::now();
        while (std::chrono::steady_clock::now() - start < std::chrono::milliseconds(static_cast<int>(duration_sec * 1000))) {
            if (emergency_stop_flag.load()) {
                result.safety_violation = true;
                result.violation_reason = "EMERGENCY_STOP_TRIGGERED";
                return result;
            }
            
            SensorReading r = sensors.read_current();
            r.power_w = actuators.get_power_draw(); // Update power
            result.samples.push_back(r);
            
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
        }
        
        // Compute Stats
        result.avg = compute_statistics(result.samples, false);
        result.max = compute_statistics(result.samples, true);
        
        // Check Step Limits
        if (result.max.noise_db > SAFE_NOISE_MAX_DB) {
            result.safety_violation = true;
            result.violation_reason = "NOISE_LIMIT_EXCEEDED";
        }
        if (result.max.temperature_c > SAFE_TEMP_MAX_C) {
            result.safety_violation = true;
            result.violation_reason = "THERMAL_LIMIT_EXCEEDED";
        }
        
        return result;
    }
    
    ApplianceProfile run_full_characterization(const std::string& app_id) {
        ApplianceProfile profile;
        profile.appliance_id = app_id;
        profile.test_session_id = generate_session_id();
        profile.test_timestamp = get_timestamp_ms();
        
        std::vector<double> intensity_steps = {0.0, 0.25, 0.5, 0.75, 1.0};
        
        for (double intensity : intensity_steps) {
            if (emergency_stop_flag.load()) break;
            
            ApplianceStepResult step = run_intensity_step(intensity, 1.0, 10.0); // 10 sec per step
            profile.test_results.push_back(step);
            
            if (step.safety_violation) {
                std::cerr << "[TEST] Violation at intensity " << intensity << ". Stopping characterization." << std::endl;
                break; // Stop testing this device
            }
            
            // Cool-down period between steps
            std::this_thread::sleep_for(std::chrono::seconds(5));
        }
        
        // Compute Knowledge Score (K) based on data quality
        profile.ker_knowledge_score = compute_knowledge_score(profile);
        
        return profile;
    }
    
private:
    std::string generate_session_id() {
        return "SESS_" + std::to_string(get_timestamp_ms());
    }
    
    uint64_t get_timestamp_ms() {
        return std::chrono::duration_cast<std::chrono::milliseconds>(
            std::chrono::system_clock::now().time_since_epoch()).count();
    }
    
    double compute_knowledge_score(const ApplianceProfile& p) {
        // K = 0.5 + (0.5 * (valid_steps / total_steps))
        if (p.test_results.empty()) return 0.0;
        int valid_steps = 0;
        for (const auto& r : p.test_results) {
            if (!r.safety_violation) valid_steps++;
        }
        return 0.5 + (0.5 * (static_cast<double>(valid_steps) / 5.0));
    }
};

// ============================================================================
// DATA EXPORTER (JSON/CSV for Lua/Rust)
// ============================================================================

class DataExporter {
public:
    static void export_lua_registry(const ApplianceProfile& p, const std::string& filename) {
        std::ofstream file(filename);
        if (!file.is_open()) return;
        
        file << "-- Auto-generated by appliance_characterization.cpp" << std::endl;
        file << "-- Session: " << p.test_session_id << std::endl;
        file << "appliance_registry[\"" << p.appliance_id << "\"] = {" << std::endl;
        file << "    type = \"BENCH_TESTED\"," << std::endl;
        file << "    characterization = {" << std::endl;
        
        for (const auto& step : p.test_results) {
            file << "        [" << step.intensity_pct << "] = {" << std::endl;
            file << "            noise = " << step.avg.noise_db << "," << std::endl;
            file << "            temp = " << step.avg.temperature_c << "," << std::endl;
            file << "            power = " << step.avg.power_w << std::endl;
            file << "        }," << std::endl;
        }
        
        file << "    }," << std::endl;
        file << "    ker_knowledge = " << p.ker_knowledge_score << std::endl;
        file << "}" << std::endl;
        
        file.close();
        std::cout << "[EXPORT] Lua registry written to " << filename << std::endl;
    }
    
    static void export_rust_corridors(const ApplianceProfile& p, const std::string& filename) {
        std::ofstream file(filename);
        if (!file.is_open()) return;
        
        file << "-- Rust Corridor Bands CSV Format" << std::endl;
        file << "varid,safe_limit,gold_limit,hard_limit,weight,mandatory" << std::endl;
        
        // Derive limits from max observed + safety margin
        double max_noise = 0.0;
        for (const auto& step : p.test_results) {
            if (step.max.noise_db > max_noise) max_noise = step.max.noise_db;
        }
        
        // Normalized 0-1 scale for Rust kernel
        double norm_max = max_noise / SAFE_NOISE_MAX_DB;
        
        file << "0x01," << (norm_max * 0.5) << "," << (norm_max * 0.8) << "," << (norm_max * 1.0) << ",1.0,true" << std::endl;
        
        file.close();
        std::cout << "[EXPORT] Rust corridors written to " << filename << std::endl;
    }
};

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

int main(int argc, char* argv[]) {
    std::cout << "=== BugsLife PDSS Appliance Characterization Tool ===" << std::endl;
    std::cout << "Version: 0.9.2-ecotribute" << std::endl;
    
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <appliance_id>" << std::endl;
        return 1;
    }
    
    std::string appliance_id = argv[1];
    std::atomic<bool> stop_flag(false);
    std::atomic<bool> emergency_stop_flag(false);
    
    // Initialize Hardware Abstraction
    MockSensorHub sensors;
    MockActuatorController actuators;
    
    // Start Safety Watchdog
    SafetyWatchdog watchdog(stop_flag, emergency_stop_flag, sensors, actuators);
    
    // Initialize Engine
    CharacterizationEngine engine(sensors, actuators, emergency_stop_flag);
    
    std::cout << "[INIT] Starting characterization for " << appliance_id << std::endl;
    std::cout << "[SAFE] Watchdog active. Limits: Noise=" << SAFE_NOISE_MAX_DB << "dB, Temp=" << SAFE_TEMP_MAX_C << "C" << std::endl;
    
    // Run Test
    ApplianceProfile profile = engine.run_full_characterization(appliance_id);
    
    // Export Data
    DataExporter::export_lua_registry(profile, "output_" + appliance_id + ".lua");
    DataExporter::export_rust_corridors(profile, "output_" + appliance_id + "_corridors.csv");
    
    std::cout << "[COMPLETE] Knowledge Score (K): " << profile.ker_knowledge_score << std::endl;
    
    if (emergency_stop_flag.load()) {
        std::cerr << "[WARN] Test ended due to emergency stop." << std::endl;
        return 2;
    }
    
    return 0;
}
