use rtsyn_plugin::{PluginApi, PluginString};
use serde_json::Value;
use std::ffi::c_void;

const INPUTS: &[&str] = &["pre", "post"];
const OUTPUTS: &[&str] = &["i_syn"];

#[repr(C)]
struct ElectricalSynapseState {
    pre: f64,
    post: f64,
    g_fast: f64,
}

extern "C" fn create(_id: u64) -> *mut c_void {
    let state = Box::new(ElectricalSynapseState {
        pre: 0.0,
        post: 0.0,
        g_fast: 0.208,
    });
    Box::into_raw(state) as *mut c_void
}

extern "C" fn destroy(handle: *mut c_void) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle as *mut ElectricalSynapseState)); }
    }
}

extern "C" fn meta_json(_handle: *mut c_void) -> PluginString {
    let value = serde_json::json!({
        "name": "Electrical Synapse",
        "default_vars": [
            ["g_fast", 0.208]
        ]
    });
    PluginString::from_string(value.to_string())
}

extern "C" fn inputs_json(_handle: *mut c_void) -> PluginString {
    PluginString::from_string(serde_json::to_string(INPUTS).unwrap_or_default())
}

extern "C" fn outputs_json(_handle: *mut c_void) -> PluginString {
    PluginString::from_string(serde_json::to_string(OUTPUTS).unwrap_or_default())
}

extern "C" fn behavior_json(_handle: *mut c_void) -> PluginString {
    let behavior = serde_json::json!({
        "supports_start_stop": true,
        "supports_restart": true,
        "extendable_inputs": {"type": "none"},
        "loads_started": true
    });
    PluginString::from_string(behavior.to_string())
}

extern "C" fn display_schema_json(_handle: *mut c_void) -> PluginString {
    let schema = serde_json::json!({
        "outputs": ["i_syn"],
        "inputs": [],
        "variables": ["g_fast"]
    });
    PluginString::from_string(schema.to_string())
}

extern "C" fn set_config_json(handle: *mut c_void, data: *const u8, len: usize) {
    if handle.is_null() || data.is_null() {
        return;
    }
    let state = unsafe { &mut *(handle as *mut ElectricalSynapseState) };
    let json_slice = unsafe { std::slice::from_raw_parts(data, len) };
    if let Ok(json_str) = std::str::from_utf8(json_slice) {
        if let Ok(config) = serde_json::from_str::<Value>(json_str) {
            if let Some(g) = config.get("g_fast").and_then(|v| v.as_f64()) {
                state.g_fast = g;
            }
        }
    }
}

extern "C" fn set_input(handle: *mut c_void, name: *const u8, len: usize, value: f64) {
    if handle.is_null() || name.is_null() {
        return;
    }
    let state = unsafe { &mut *(handle as *mut ElectricalSynapseState) };
    let name_slice = unsafe { std::slice::from_raw_parts(name, len) };
    if let Ok(name_str) = std::str::from_utf8(name_slice) {
        match name_str {
            "pre" => state.pre = value,
            "post" => state.post = value,
            _ => {}
        }
    }
}

extern "C" fn process(_handle: *mut c_void, _tick: u64, _period_seconds: f64) {
    // Electrical synapse is computed instantly, no processing needed
}

extern "C" fn get_output(handle: *mut c_void, name: *const u8, len: usize) -> f64 {
    if handle.is_null() || name.is_null() {
        return 0.0;
    }
    let state = unsafe { &*(handle as *const ElectricalSynapseState) };
    let name_slice = unsafe { std::slice::from_raw_parts(name, len) };
    if let Ok(name_str) = std::str::from_utf8(name_slice) {
        if name_str == "i_syn" {
            // Alternative: g_fast * (pre - post) for positive = phase
            return state.g_fast * (state.post - state.pre);
        }
    }
    0.0
}

#[no_mangle]
pub extern "C" fn rtsyn_plugin_api() -> *const PluginApi {
    static API: PluginApi = PluginApi {
        create,
        destroy,
        meta_json,
        inputs_json,
        outputs_json,
        behavior_json: Some(behavior_json),
        display_schema_json: Some(display_schema_json),
        ui_schema_json: None,
        set_config_json,
        set_input,
        process,
        get_output,
    };
    &API
}
