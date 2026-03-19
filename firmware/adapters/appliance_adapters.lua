#!/usr/bin/env lua
--[[
    BugsLife PDSS - Legacy Appliance Adapter Layer
    Module: appliance_adapters.lua
    Version: 0.9.2-ecotribute
    License: MIT + Ecological Safety Covenant
    
    DESCRIPTION:
    This module provides the translation layer between abstract PDSS intents
    (approved by the Rust Safety Kernel) and physical hardware commands for
    legacy appliances (IR, Serial, GPIO, MQTT). 
    
    SAFETY BOUNDARY:
    This script DOES NOT enforce safety corridors. It assumes all intents
    received have already passed the Rust kernel's `safestep()` verification.
    However, it includes local sanity checks to prevent hardware damage.
    
    ECOLOGICAL MISSION:
    - Minimizes energy waste via optimized duty cycling.
    - Prevents harmful emissions (acoustic/thermal) via empirical mapping.
    - Supports legacy hardware reuse to reduce e-waste.
--]]

local pdss_adapter = {}
pdss_adapter._version = "0.9.2"
pdss_adapter._strict_safety = true -- Fail closed on unknown devices

-- ============================================================================
-- CONFIGURATION & CONSTANTS
-- ============================================================================

local MAX_IR_REPEATS = 3        -- Prevent IR flooding
local SERIAL_TIMEOUT_MS = 500   -- Modbus/RS485 timeout
local PWM_FREQUENCY_HZ = 2000   -- Default PWM for fan/heater control
local SAFE_TEMP_MAX_C = 45.0    -- Hard cutoff for thermal actuators
local SAFENoise_MAX_DB = 85.0   -- Hard cutoff for acoustic actuators

-- Risk Coordinate Mapping (Matches Rust VarId enum)
local RISK_COORDS = {
    NOISE      = 0x01,
    LIGHT      = 0x02,
    ODOR       = 0x03,
    THERMAL    = 0x04,
    VIBRATION  = 0x05,
    ULTRASONIC = 0x06,
}

-- ============================================================================
-- APPLIANCE CHARACTERIZATION DATABASE
-- ============================================================================
-- Research Gap: This table must be populated via systematic bench-testing.
-- Each entry maps intensity_pct (0.0-1.0) to physical output metrics.
-- Data Source: Empirical measurements (Anemometer, SPL Meter, Thermal Camera)

local appliance_registry = {
    -- Example: Generic AC Fan (3-speed legacy)
    ["fan_legacy_3speed"] = {
        type = "IR_CONTROLLED",
        protocol = "NEC",
        device_id = 0x10FE,
        modalities = { RISK_COORDS.NOISE, RISK_COORDS.VIBRATION, RISK_COORDS.AIRFLOW },
        characterization = {
            -- [intensity] = { noise_db, airflow_cfm, power_w }
            [0.0] = { noise = 0.0, airflow = 0.0, power = 0.0 },
            [0.33] = { noise = 35.0, airflow = 50.0, power = 15.0 }, -- Low
            [0.66] = { noise = 50.0, airflow = 100.0, power = 30.0 }, -- Med
            [1.0] = { noise = 65.0, airflow = 150.0, power = 50.0 }, -- High
        },
        commands = {
            [0.0] = 0x10FE01FE, -- Power Off
            [0.33] = 0x10FE02FD, -- Speed 1
            [0.66] = 0x10FE03FC, -- Speed 2
            [1.0] = 0x10FE04FB, -- Speed 3
        }
    },
    
    -- Example: Smart Plug (WiFi/MQTT) controlling Space Heater
    ["heater_smartplug"] = {
        type = "MQTT_RELAY",
        topic_base = "home/bedroom/heater",
        modalities = { RISK_COORDS.THERMAL },
        characterization = {
            [0.0] = { temp_rise_c = 0.0, power = 0.0 },
            [0.5] = { temp_rise_c = 2.0, power = 750.0 }, -- Eco Mode
            [1.0] = { temp_rise_c = 5.0, power = 1500.0 }, -- Max
        },
        commands = {
            [0.0] = { payload = "OFF", qos = 1 },
            [0.5] = { payload = "ON_ECO", qos = 1 },
            [1.0] = { payload = "ON_MAX", qos = 1 },
        }
    },

    -- Example: TV Screen (Visual Deterrent)
    ["tv_visual_deterrent"] = {
        type = "HDMI_CEC",
        modalities = { RISK_COORDS.LIGHT },
        characterization = {
            [0.0] = { lux = 0.0, power = 1.0 }, -- Standby
            [0.5] = { lux = 100.0, power = 50.0 }, -- Dim White
            [1.0] = { lux = 300.0, power = 100.0 }, -- Max White
        },
        commands = {
            [0.0] = { cec = "STANDBY" },
            [0.5] = { cec = "ON", pattern = "WHITE_DIM" },
            [1.0] = { cec = "ON", pattern = "WHITE_MAX" },
        }
    }
}

-- ============================================================================
-- INTENT TRANSLATION ENGINE
-- ============================================================================

--- Quantizes continuous intensity (0.0-1.0) to discrete device steps
--- @param intensity number (0.0-1.0)
--- @param steps table Available intensity steps (keys)
--- @return number closest_step
local function quantize_intensity(intensity, steps)
    local closest = steps[1]
    local min_diff = math.abs(intensity - steps[1])
    
    for _, step in ipairs(steps) do
        local diff = math.abs(intensity - step)
        if diff < min_diff then
            min_diff = diff
            closest = step
        end
    end
    return closest
end

--- Translates PDSS Intent to Hardware Command
--- @param appliance_id string Key in appliance_registry
--- @param intent table {profile_id, intensity_pct, duty_cycle, modality}
--- @return table command_struct Ready for execution layer
function pdss_adapter.translate_intent(appliance_id, intent)
    local device = appliance_registry[appliance_id]
    
    if not device then
        if pdss_adapter._strict_safety then
            error("SAFETY_LOCK: Unknown appliance_id " .. tostring(appliance_id))
        end
        return nil
    end
    
    -- Verify modality compatibility (Prevent misuse)
    local modality_match = false
    for _, m in ipairs(device.modalities) do
        if m == intent.modality then
            modality_match = true
            break
        end
    end
    
    if not modality_match then
        warn("ECOTRIBUTE_WARN: Intent modality mismatch for " .. appliance_id)
        -- Fallback to safest option (Off)
        return { action = "OFF", reason = "modality_mismatch" }
    end
    
    -- Quantize intensity to device capabilities
    local available_steps = {}
    for k, _ in pairs(device.characterization) do
        table.insert(available_steps, k)
    end
    table.sort(available_steps)
    
    local effective_intensity = quantize_intensity(intent.intensity_pct, available_steps)
    local command_code = device.commands[effective_intensity]
    
    -- Build Command Structure
    local cmd = {
        device_id = appliance_id,
        action = "EXECUTE",
        protocol = device.type,
        intensity_actual = effective_intensity,
        duty_cycle = intent.duty_cycle,
        payload = command_code,
        estimated_impact = device.characterization[effective_intensity],
        timestamp = os.time(),
    }
    
    -- Energy Optimization: If intensity is negligible, force OFF
    if effective_intensity < 0.05 then
        cmd.action = "OFF"
        cmd.payload = device.commands[0.0]
        cmd.intensity_actual = 0.0
    end
    
    return cmd
end

-- ============================================================================
-- HARDWARE ABSTRACTION LAYER (HAL) SIMULATION
-- ============================================================================
-- In production, these functions bind to C/Rust FFI or OS drivers.

local hal = {
    ir_send = function(code, repeats) 
        -- print(string.format("IR_TX: 0x%X (x%d)", code, repeats)) 
    end,
    mqtt_publish = function(topic, payload, qos)
        -- print(string.format("MQTT_PUB: %s -> %s", topic, payload))
    end,
    gpio_pwm = function(pin, duty, freq)
        -- print(string.format("PWM: Pin%d Duty=%f Freq=%d", pin, duty, freq))
    end,
    serial_write = function(port, data)
        -- print(string.format("UART[%s]: %s", port, data))
    end
}

--- Executes the translated command via appropriate HAL
--- @param cmd table The translated command structure
--- @return boolean success
function pdss_adapter.execute_command(cmd)
    if cmd.action == "OFF" then
        -- Ensure safe shutdown
        if cmd.protocol == "MQTT_RELAY" then
            hal.mqtt_publish(cmd.device_id .. "/cmd", cmd.payload.payload, cmd.payload.qos)
        elseif cmd.protocol == "IR_CONTROLLED" then
            hal.ir_send(cmd.payload, 1)
        end
        return true
    end
    
    if cmd.action == "EXECUTE" then
        if cmd.protocol == "IR_CONTROLLED" then
            hal.ir_send(cmd.payload, MAX_IR_REPEATS)
        elseif cmd.protocol == "MQTT_RELAY" then
            hal.mqtt_publish(cmd.device_id .. "/cmd", cmd.payload.payload, cmd.payload.qos)
        elseif cmd.protocol == "HDMI_CEC" then
            -- Simulate CEC command
            hal.serial_write("CEC_BUS", cmd.payload.cec)
        elseif cmd.protocol == "GPIO_PWM" then
            hal.gpio_pwm(cmd.pin, cmd.intensity_actual, PWM_FREQUENCY_HZ)
        else
            warn("Unknown protocol: " .. tostring(cmd.protocol))
            return false
        end
        return true
    end
    
    return false
end

-- ============================================================================
-- LOCAL SAFETY & ECO-SANITY CHECKS
-- ============================================================================
-- These are secondary checks. Primary safety is in Rust Kernel.
-- These protect hardware integrity and prevent obvious ecological violations.

function pdss_adapter.sanity_check(cmd, env_context)
    -- 1. Thermal Safety (Prevent overheating legacy heaters)
    if cmd.estimated_impact and cmd.estimated_impact.temp_rise_c then
        if env_context and env_context.ambient_temp_c then
            local projected_temp = env_context.ambient_temp_c + cmd.estimated_impact.temp_rise_c
            if projected_temp > SAFE_TEMP_MAX_C then
                warn("ECO_SAFETY: Thermal limit exceeded. Derating.")
                return false, "THERMAL_LIMIT"
            end
        end
    end
    
    -- 2. Noise Safety (Prevent neighborhood disturbance)
    if cmd.estimated_impact and cmd.estimated_impact.noise then
        if cmd.estimated_impact.noise > SAFENoise_MAX_DB then
            warn("ECO_SAFETY: Noise limit exceeded. Blocking.")
            return false, "NOISE_LIMIT"
        end
    end
    
    -- 3. Energy Waste Check (Prevent high power during low efficacy)
    if cmd.intensity_actual > 0.8 and cmd.duty_cycle > 0.9 then
        -- Log for K/E/R tracking
        -- print("HIGH_ENERGY_PROFILE_ACTIVE") 
    end
    
    return true, "OK"
end

-- ============================================================================
-- ADAPTIVE LEARNING HOOKS (ALN Integration)
-- ============================================================================
-- Stores feedback data for the Adaptive Logic Network to refine characterization.

local learning_buffer = {}

function pdss_adapter.record_outcome(intent_id, cmd, actual_sensor_readings)
    -- Store for batch upload to Rust Kernel / Shard
    table.insert(learning_buffer, {
        intent = intent_id,
        commanded = cmd.intensity_actual,
        actual_noise = actual_sensor_readings.noise_db,
        actual_power = actual_sensor_readings.power_w,
        timestamp = os.time()
    })
    
    -- Keep buffer small
    if #learning_buffer > 100 then
        table.remove(learning_buffer, 1)
    end
end

function pdss_adapter.get_learning_data()
    local data = learning_buffer
    learning_buffer = {}
    return data
end

-- ============================================================================
-- MAIN ENTRY POINT (Called by Rust FFI or Event Loop)
-- ============================================================================

--- Primary API for the Safety Kernel to trigger actuation
--- @param appliance_id string
--- @param intent table Validated by Rust Kernel
--- @param env_context table Current sensor readings
--- @return boolean success, string reason
function pdss_adapter.dispatch(appliance_id, intent, env_context)
    -- 1. Translate Intent
    local cmd = pdss_adapter.translate_intent(appliance_id, intent)
    if not cmd then return false, "TRANSLATION_FAILED" end
    
    -- 2. Local Sanity Check (Secondary Safety Layer)
    local safe, reason = pdss_adapter.sanity_check(cmd, env_context)
    if not safe then return false, reason end
    
    -- 3. Execute
    local success = pdss_adapter.execute_command(cmd)
    
    -- 4. Record for Adaptive Learning
    if success then
        pdss_adapter.record_outcome(intent.profile_id, cmd, env_context)
    end
    
    return success, success and "EXECUTED" or "EXECUTION_FAILED"
end

-- ============================================================================
-- UNIT TESTS (Self-Verification)
-- ============================================================================

local function run_self_test()
    print("Running Appliance Adapter Self-Test...")
    
    -- Test 1: Valid Intent
    local intent = {
        profile_id = "bedbug_thermal_low",
        intensity_pct = 0.4,
        duty_cycle = 0.5,
        modality = RISK_COORDS.THERMAL
    }
    
    local cmd = pdss_adapter.translate_intent("heater_smartplug", intent)
    assert(cmd ~= nil, "Test 1 Failed: Valid intent returned nil")
    assert(cmd.intensity_actual == 0.5, "Test 1 Failed: Quantization error")
    
    -- Test 2: Unknown Device (Safety Lock)
    local success, err = pcall(function()
        pdss_adapter.translate_intent("unknown_device_xyz", intent)
    end)
    assert(not success, "Test 2 Failed: Safety lock did not trigger")
    
    -- Test 3: Thermal Limit
    local env = { ambient_temp_c = 44.0 }
    local safe, reason = pdss_adapter.sanity_check(cmd, env)
    assert(not safe, "Test 3 Failed: Thermal limit not caught")
    assert(reason == "THERMAL_LIMIT", "Test 3 Failed: Wrong reason code")
    
    print("All Adapter Tests Passed.")
end

-- Uncomment to run on load in standalone mode
-- run_self_test()

return pdss_adapter
