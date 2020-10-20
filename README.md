# About

This repo is part of a larger project called Audio Anywhere (AA). Audio Anywhere a 
framework for work-ing with audio plugins that are compiled once and run anywhere.
At the heart of Audio Anywhere is an audio engine whose Digital Signal Processing (DSP) components are written in Faust and deployed with WebAssembly. 

Details about the project can be found on the [project's homepage](https://muses-dmi.github.io/projects/).

## Introduction

C bindings for [AA Wasmtime](https://github.com/bgaster/aa_wasmtime).

## Building

It is implemented using stable [Rust](https://www.rust-lang.org/).

To install Rust go you need simply to install [Rustup](https://rustup.rs/) and 
if you already have Rust installed, then you can update with the command rustup update.

You can build with the following cargo command:

```bash
cargo build --release
```
This produces:

 * dist/aa_wasmtime_c.h
 * target/release/libaa_wasmtime_c.a

## API

###  Create an Audio Anywhere module instance

Returns a pointer to an _AAModule_, if successful, otherwise null.

Arguments

* _url_ - URL for Audio Anywhere server
* _module_ - Module to load

```c++
AAModule *aa_module_new(
  const char *url, 
  const char *module);
```

### Delete memory for previously allocated AA module

Arguments

* _ptr_ - Previously allocated AA module

```c++
void aa_module_delete(AAModule *ptr);
```

```c++
const char *aa_get_modules(const char *url);
```

### Get GUI description JSON

Returns JSON for optional GUI description assocated with loaded
module, otherwise null.

Arguments

* _ptr_ - A previously allocated AA module.

```c++
const char *get_gui_description(AAModule *ptr);
```

### Initalize module

Arguments

* _ptr_ - A previously allocated AA module.
* _sample\_rate_ - Audio sample rate. If sample rate changes, module must be re-initialized.

```c++
void aa_module_init(AAModule *ptr, double sample_rate);
```

### Set parameter

Arguments

* _ptr_ - A previously allocated AA module.
* _node_ - Node index within audio graph
* _index_ - Parameter index within node
* _param_ - Parameter value

```c++
void set_param_float(
  AAModule *ptr, 
  unsigned int node, 
  unsigned int index, 
  float param);
```

### Handle note on message

Currently, timing is at the buffer level and not per sample. (This will be changed in the near future.)

Arguments 

* _ptr_ - A previously allocated AA module.
* _note_ - MIDI note
* _velocity_ - Velocity of note press, between 0.0 - 1.0

```c++
void aa_module_handle_note_on(
  AAModule *ptr, 
  int note, 
  float velocity);
```

### Handle note off message

Currently, timing is at the buffer level and not per sample. 
(This will be changed in the near future.)

Arguments 

* _ptr_ - A previously allocated AA module.
* _note_ - MIDI note
* _velocity_ - Velocity of note press, between 0.0 - 1.0

```c++
void aa_module_handle_note_off(
  AAModule *ptr, 
  int note, 
  float velocity);
```

### Number of audio inputs

Returns the number of audio inputs for module.

Arguments 

* _ptr_ - A previously allocated AA module.

```c++
int aa_module_get_number_inputs(AAModule *ptr);
```

### Number of audio outputs

Returns the number of audio outputs for module.

Arguments 

* _ptr_ - A previously allocated AA module.

```c++
int aa_module_get_number_outputs(AAModule *ptr);
```

### Compute audio for zero in and one out

Arguments 

* _ptr_ - A previously allocated AA module.
* _frames_ - Number of samples to produce
* _outputs_ - Pointer to audio output buffer

```c++
void aa_module_compute_zero_one(
  AAModule *ptr, 
  int frames, 
  float *outputs);
```

### Compute audio for one in and two out

Arguments 

* _ptr_ - A previously allocated AA module.
* _frames_ - Number of samples to produce
* _input_ - Pointer to audio input buffer
* _output0_ - Pointer to audio output buffer 1
* _output0_ - Pointer to audio output buffer 2

```c++
void aa_module_compute_one_two_non(AAModule *ptr,
                                   int frames,
                                   const float *input,
                                   float *output0,
                                   float *output1);
```

### Compute audio for zero in and two out

Arguments 

* _ptr_ - A previously allocated AA module.
* _frames_ - Number of samples to produce
* _output0_ - Pointer to audio output buffer 1
* _output0_ - Pointer to audio output buffer 2

```c++
void aa_module_compute_zero_two_non(AAModule *ptr, int frames, float *output0, float *output1);
```

### Compute audio for one in and one out

Arguments 

* _ptr_ - A previously allocated AA module.
* _frames_ - Number of samples to produce
* _inputs_ - Pointer to audio input buffer
* _output_ - Pointer to audio output buffer

```c++
void aa_module_compute_one_one(AAModule *ptr, int frames, const float *input, float *output);
```

# License
© 2020 [Benedict R. Gaster (cuberoo_)](https://bgaster.github.io/)

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
