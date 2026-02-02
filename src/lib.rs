use rtsyn_plugin::{
    PluginApi, PluginString,
    Plugin, PluginContext, PluginError,
    PluginId, PluginMeta, Port, PortId,
};
use serde_json::Value;
use std::ffi::c_void;
use std::slice;
use std::str;

// ============================
// Static port declarations
// ============================

const INPUTS: &[&str] = &["pre", "post"];
const OUTPUTS: &[&str] = &["i_syn"];

// ============================
// Core plugin implementation
// ============================

pub struct ElectricalSynapsePlugin {
    id: PluginId,
    meta: PluginMeta,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    pub pre: f64,
    pub post: f64,
    pub output: f64,
    pub g_fast: f64,
}

impl ElectricalSynapsePlugin {
    pub fn new(id: u64) -> Self {
        Self {
            id: PluginId(id),
            meta: PluginMeta {
                name: "Electrical Synapse".to_string(),
                fixed_vars: Vec::new(),
                default_vars: vec![("g_fast".to_string(), Value::from(0.208))],
            },
            inputs: vec![
                Port { id: PortId("pre".to_string()) },
                Port { id: PortId("post".to_string()) },
            ],
            outputs: vec![
                Port { id: PortId("i_syn".to_string()) },
            ],
            pre: 0.0,
            post: 0.0,
            output: 0.0,
            g_fast: 0.208,
        }
    }
}

impl Plugin for ElectricalSynapsePlugin {
    fn id(&self) -> PluginId {
        self.id
    }

    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn inputs(&self) -> &[Port] {
        &self.inputs
    }

    fn outputs(&self) -> &[Port] {
        &self.outputs
    }

    fn process(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> {
        self.output = self.g_fast * (self.post - self.pre);
        Ok(())
    }
}

// ============================
// FFI state wrapper
// ============================

struct PluginState {
    plugin: ElectricalSynapsePlugin,
    ctx: PluginContext,
}

// ============================
// ABI functions
// ============================

extern "C" fn create(id: u64) -> *mut c_void {
    let state = PluginState {
        plugin: ElectricalSynapsePlugin::new(id),
        ctx: PluginContext::default(),
    };
    Box::into_raw(Box::new(state)) as *mut c_void
}

extern "C" fn destroy(handle: *mut c_void) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle as *mut PluginState)) }
    }
}

extern "C" fn meta_json(_: *mut c_void) -> PluginString {
    PluginString::from_string(
        serde_json::json!({
            "name": "Electrical Synapse",
            "kind": "electrical_synapse"
        })
        .to_string(),
    )
}

extern "C" fn inputs_json(_: *mut c_void) -> PluginString {
    PluginString::from_string(serde_json::to_string(INPUTS).unwrap())
}

extern "C" fn outputs_json(_: *mut c_void) -> PluginString {
    PluginString::from_string(serde_json::to_string(OUTPUTS).unwrap())
}

extern "C" fn set_config_json(
    handle: *mut c_void,
    data: *const u8,
    len: usize,
) {
    if handle.is_null() || data.is_null() {
        return;
    }

    let state = unsafe { &mut *(handle as *mut PluginState) };
    let bytes = unsafe { slice::from_raw_parts(data, len) };

    if let Ok(json) = serde_json::from_slice::<Value>(bytes) {
        if let Some(g_fast) = json.get("g_fast").and_then(|v| v.as_f64()) {
            state.plugin.g_fast = g_fast;
        }
    }
}

extern "C" fn set_input(
    handle: *mut c_void,
    port: *const u8,
    len: usize,
    value: f64,
) {
    if handle.is_null() || port.is_null() {
        return;
    }

    let state = unsafe { &mut *(handle as *mut PluginState) };
    let name = unsafe { slice::from_raw_parts(port, len) };

    match str::from_utf8(name) {
        Ok("pre") => state.plugin.pre = value,
        Ok("post") => state.plugin.post = value,
        _ => {}
    }
}

extern "C" fn process(handle: *mut c_void, tick: u64) {
    if handle.is_null() {
        return;
    }

    let state = unsafe { &mut *(handle as *mut PluginState) };
    state.ctx.tick = tick;

    let _ = state.plugin.process(&mut state.ctx);
}

extern "C" fn get_output(
    handle: *mut c_void,
    port: *const u8,
    len: usize,
) -> f64 {
    if handle.is_null() || port.is_null() {
        return 0.0;
    }

    let state = unsafe { &*(handle as *mut PluginState) };
    let name = unsafe { slice::from_raw_parts(port, len) };

    match str::from_utf8(name) {
        Ok("i_syn") => state.plugin.output,
        _ => 0.0,
    }
}

// ============================
// Plugin API export
// ============================

#[no_mangle]
pub extern "C" fn rtsyn_plugin_api() -> *const PluginApi {
    static API: PluginApi = PluginApi {
        create,
        destroy,
        meta_json,
        inputs_json,
        outputs_json,
        set_config_json,
        set_input,
        process,
        get_output,
    };
    &API
}
