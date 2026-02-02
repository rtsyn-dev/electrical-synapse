use rtsyn_plugin::prelude::*;
use serde_json::Value;
use std::ffi::c_void;

#[repr(C)]
struct ElectricalSynapseCore(c_void);

extern "C" {
    fn electrical_synapse_new() -> *mut ElectricalSynapseCore;
    fn electrical_synapse_free(state: *mut ElectricalSynapseCore);
    fn electrical_synapse_set_config(
        state: *mut ElectricalSynapseCore,
        key: *const u8,
        len: usize,
        value: f64,
    );
    fn electrical_synapse_set_input(
        state: *mut ElectricalSynapseCore,
        key: *const u8,
        len: usize,
        value: f64,
    );
    fn electrical_synapse_process(state: *mut ElectricalSynapseCore);
    fn electrical_synapse_get_output(
        state: *const ElectricalSynapseCore,
        key: *const u8,
        len: usize,
    ) -> f64;
    fn electrical_synapse_get_internal(
        state: *const ElectricalSynapseCore,
        key: *const u8,
        len: usize,
    ) -> f64;
}

struct ElectricalSynapse {
    state: *mut ElectricalSynapseCore,
}

impl Default for ElectricalSynapse {
    fn default() -> Self {
        let state = unsafe { electrical_synapse_new() };
        Self { state }
    }
}

impl Drop for ElectricalSynapse {
    fn drop(&mut self) {
        if !self.state.is_null() {
            unsafe { electrical_synapse_free(self.state) };
            self.state = std::ptr::null_mut();
        }
    }
}

impl PluginDescriptor for ElectricalSynapse {
    fn name() -> &'static str {
        "Electrical Synapse"
    }

    fn kind() -> &'static str {
        "electrical_synapse"
    }

    fn plugin_type() -> PluginType {
        PluginType::Computational
    }

    fn inputs() -> &'static [&'static str] {
        &["v_pre", "v_post"]
    }

    fn outputs() -> &'static [&'static str] {
        &["i_gap"]
    }

    fn internal_variables() -> &'static [&'static str] {
        &["g_gap"]
    }

    fn behavior() -> PluginBehavior {
        PluginBehavior {
            loads_started: false,
            ..PluginBehavior::default()
        }
    }

    fn default_vars() -> Vec<(&'static str, Value)> {
        vec![("g_gap", 0.1.into())]
    }
}

impl PluginRuntime for ElectricalSynapse {
    fn set_config_value(&mut self, key: &str, value: &Value) {
        if self.state.is_null() {
            return;
        }
        if let Some(v) = value.as_f64() {
            unsafe { electrical_synapse_set_config(self.state, key.as_ptr(), key.len(), v) };
        }
    }

    fn set_input_value(&mut self, key: &str, value: f64) {
        if self.state.is_null() {
            return;
        }
        unsafe { electrical_synapse_set_input(self.state, key.as_ptr(), key.len(), value) };
    }

    fn process_tick(&mut self, _tick: u64, _period_seconds: f64) {
        if self.state.is_null() {
            return;
        }
        unsafe { electrical_synapse_process(self.state) };
    }

    fn get_output_value(&self, key: &str) -> f64 {
        if self.state.is_null() {
            return 0.0;
        }
        unsafe { electrical_synapse_get_output(self.state, key.as_ptr(), key.len()) }
    }

    fn get_internal_value(&self, key: &str) -> Option<f64> {
        if self.state.is_null() {
            return None;
        }
        if key == "g_gap" {
            return Some(unsafe { electrical_synapse_get_internal(self.state, key.as_ptr(), key.len()) });
        }
        None
    }
}

rtsyn_plugin::export_plugin!(ElectricalSynapse);
