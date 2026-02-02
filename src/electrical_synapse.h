#pragma once
#include <stddef.h>

typedef struct electrical_synapse_state electrical_synapse_state_t;

electrical_synapse_state_t *electrical_synapse_new(void);
void electrical_synapse_free(electrical_synapse_state_t *state);
void electrical_synapse_set_config(electrical_synapse_state_t *state,
                                     const char *key, size_t len, double value);
void electrical_synapse_set_input(electrical_synapse_state_t *state,
                                    const char *key, size_t len, double value);
void electrical_synapse_process(electrical_synapse_state_t *state);
double electrical_synapse_get_output(const electrical_synapse_state_t *state,
                                       const char *key, size_t len);
double electrical_synapse_get_internal(
    const electrical_synapse_state_t *state, const char *key, size_t len);
