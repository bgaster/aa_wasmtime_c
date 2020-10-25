#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct AAModule;

extern "C" {

/// create audio anywhere module
AAModule *aa_module_new(const char *url, const char *module);

/// deallocate AA module
void aa_module_delete(AAModule *ptr);

/// get JSON string for modules
const char *aa_get_modules(const char *url);

/// get JSON string for GUI descripion
const char *get_gui_description(AAModule *ptr);

/// init AA module
/// not thread safe
void aa_module_init(AAModule *ptr, double sample_rate);

/// set param for node in graph
void set_param_float(AAModule *ptr, unsigned int node, unsigned int index, float param);

/// handle note on
void aa_module_handle_note_on(AAModule *ptr, int note, float velocity);

/// handle note off
void aa_module_handle_note_off(AAModule *ptr, int note, float velocity);

/// number of audio inputs
/// not thread safe
int aa_module_get_number_inputs(AAModule *ptr);

/// number of audio outputs
/// not thread safe
int aa_module_get_number_outputs(AAModule *ptr);

void aa_module_compute_zero_one(AAModule *ptr, int frames, float *outputs);

void aa_module_compute_one_one(AAModule *ptr, int frames, const float *inputs, float *outputs);

void aa_module_compute_one_two_non(AAModule *ptr,
                                   int frames,
                                   const float *input,
                                   float *output0,
                                   float *output1);

void aa_module_compute_two_two_non(AAModule *ptr,
                                   int frames,
                                   const float *input0,
                                   const float *input1,
                                   float *output0,
                                   float *output1);

void aa_module_compute_zero_two_non(AAModule *ptr, int frames, float *output0, float *output1);

} // extern "C"
