use rtsyn_plugin::prelude::*;
use serde_json::Value;

struct ElectricalSynapse {
    v_pre: f64,
    v_post: f64,
    g_gap: f64,
    i_gap: f64,
}

impl Default for ElectricalSynapse {
    fn default() -> Self {
        Self {
            v_pre: 0.0,
            v_post: 0.0,
            g_gap: 0.1,
            i_gap: 0.0,
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

    fn default_vars() -> Vec<(&'static str, Value)> {
        vec![("g_gap", 0.1.into())]
    }

    fn behavior() -> PluginBehavior {
        PluginBehavior {
            loads_started: false,
            ..PluginBehavior::default()
        }
    }
}

impl PluginRuntime for ElectricalSynapse {
    fn set_config_value(&mut self, key: &str, value: &Value) {
        if key == "g_gap" {
            if let Some(v) = value.as_f64() {
                self.g_gap = if v.is_finite() { v } else { self.g_gap };
            }
        }
    }

    fn set_input_value(&mut self, key: &str, value: f64) {
        let v = if value.is_finite() { value } else { 0.0 };
        match key {
            "v_pre" => self.v_pre = v,
            "v_post" => self.v_post = v,
            _ => {}
        }
    }

    fn process_tick(&mut self, _tick: u64, _period_seconds: f64) {
        self.i_gap = self.g_gap * (self.v_pre - self.v_post);
    }

    fn get_output_value(&self, key: &str) -> f64 {
        match key {
            "i_gap" => self.i_gap,
            _ => 0.0,
        }
    }

    fn get_internal_value(&self, key: &str) -> Option<f64> {
        match key {
            "g_gap" => Some(self.g_gap),
            _ => None,
        }
    }
}

rtsyn_plugin::export_plugin!(ElectricalSynapse);
