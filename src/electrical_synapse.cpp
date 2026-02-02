#include "electrical_synapse.h"
#include <cmath>
#include <cstdlib>
#include <cstring>

struct electrical_synapse_state {
  double v_pre;
  double v_post;
  double g_gap;
  double i_gap;
};

static int key_eq(const char *key, size_t len, const char *lit) {
  size_t n = std::strlen(lit);
  return (len == n) && (std::strncmp(key, lit, n) == 0);
}

static void electrical_synapse_init(electrical_synapse_state_t *state) {
  state->v_pre = 0.0;
  state->v_post = 0.0;
  state->g_gap = 0.1;
  state->i_gap = 0.0;
}

extern "C" electrical_synapse_state_t *electrical_synapse_new(void) {
  auto *state = static_cast<electrical_synapse_state_t *>(
      std::calloc(1, sizeof(electrical_synapse_state_t)));
  if (state == nullptr) {
    return nullptr;
  }
  electrical_synapse_init(state);
  return state;
}

extern "C" void electrical_synapse_free(electrical_synapse_state_t *state) {
  std::free(state);
}

extern "C" void electrical_synapse_set_config(
    electrical_synapse_state_t *state, const char *key, size_t len,
    double value) {
  if (state == nullptr || key == nullptr || !std::isfinite(value)) {
    return;
  }
  if (key_eq(key, len, "g_gap")) {
    state->g_gap = value;
  }
}

extern "C" void electrical_synapse_set_input(
    electrical_synapse_state_t *state, const char *key, size_t len,
    double value) {
  if (state == nullptr || key == nullptr || !std::isfinite(value)) {
    return;
  }
  if (key_eq(key, len, "v_pre")) {
    state->v_pre = value;
  } else if (key_eq(key, len, "v_post")) {
    state->v_post = value;
  }
}

extern "C" void electrical_synapse_process(
    electrical_synapse_state_t *state) {
  if (state == nullptr) {
    return;
  }
  state->i_gap = state->g_gap * (state->v_pre - state->v_post);
}

extern "C" double electrical_synapse_get_output(
    const electrical_synapse_state_t *state, const char *key, size_t len) {
  if (state == nullptr || key == nullptr) {
    return 0.0;
  }
  if (key_eq(key, len, "i_gap")) {
    return state->i_gap;
  }
  return 0.0;
}

extern "C" double electrical_synapse_get_internal(
    const electrical_synapse_state_t *state, const char *key, size_t len) {
  if (state == nullptr || key == nullptr) {
    return 0.0;
  }
  if (key_eq(key, len, "g_gap")) {
    return state->g_gap;
  }
  return 0.0;
}
