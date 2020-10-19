// Desc:
//   C Wrapper around Audio Anywhere Rust Wasm interface
// 

extern crate aa_wasmtime;

extern crate crossbeam_channel;
use crossbeam_channel as cb;

use libc::{c_char, c_int, c_float, c_double, c_uint};
use std::ffi::{CStr, CString};

mod utils;
mod bundle;

use crate::utils::*;
use crate::bundle::*;

#[derive(Debug, Clone, Copy)]
enum Command {
    Param(u32, u32, f32),
    NoteOn(i32, f32),
    NoteOff(i32, f32),
}

//#[repr(C)]
pub struct AAModule {
    sender: cb::Sender<Command>, 
    receiver: cb::Receiver<Command>,
    /// wasmtime AA unit, representation of an AA module
    //aaunit: *mut aa_wasmtime::AAUnit,
    aaunit: aa_wasmtime::AAUnit,
    gui_description: Option<String>,
}

#[no_mangle]
/// create audio anywhere module
pub extern "C" fn aa_module_new(url: *const c_char, module: *const c_char) -> *mut AAModule {
    
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };
    let module = unsafe { CStr::from_ptr(module).to_str().unwrap() };

    let bundle_url = [url, module].join("/");
    let json = match get_string(&bundle_url) {
        Ok(json) => json,
        Err(_) => {
            return std::ptr::null_mut();
        }
    };

    let bundle = match Bundle::from_json(&json) {
        Ok(bundle) => bundle,
        Err(_) => {
            return std::ptr::null_mut();
        }
    };

    let mut wasm_bytes = Vec::new();

    for wasm_url in bundle.wasm_url.iter() {
        let bytes = match get_vec(&[url, &wasm_url].join("")) {
            Ok(bytes) => bytes,
            Err(_) => {
                return std::ptr::null_mut();
            }
        };
        wasm_bytes.push(bytes);
    }

    let aaunit = match aa_wasmtime::AAUnit::new(wasm_bytes) {
        Ok(aaunit) => aaunit,
        Err(_) => {
            return std::ptr::null_mut();
        }
    };

    let (sender, receiver) = cb::unbounded();
    // load GUI description JSON, if present
    let gui_description = if let Some(gui_desc) = bundle.gui_description {
        let description_url = [url, &gui_desc].join("/");
        if let Ok(json) = get_string(&description_url) {
            Some(json)
        }
        else {
            None
        }
    }
    else {
        None
    };

    Box::into_raw(Box::new(AAModule { 
        sender,
        receiver,
        aaunit,
        gui_description,
     }))
}

#[no_mangle]
/// deallocate AA module
pub extern "C" fn aa_module_delete(ptr: *mut AAModule) {
    if !ptr.is_null() {
        unsafe {
            Box::from_raw(ptr);
        }    
    }
}

#[no_mangle]
/// get JSON string for modules
pub extern "C" fn aa_get_modules(url: *const c_char) -> *const c_char {
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };

    let modules_url = [url, "modules.json"].join("/");
    if let Ok(json) = get_string(&modules_url) {
        let s = CString::new(json).unwrap();
        let p = s.as_ptr();
        std::mem::forget(s);
        p
    }
    else {
        std::ptr::null() 
    }
}

#[no_mangle]
/// get JSON string for GUI descripion
pub extern "C" fn get_gui_description(ptr: *mut AAModule) -> *const c_char {
    let module = to_module(ptr);
    let ptr = if let Some(gui) = &module.gui_description {
        let s = CString::new(gui.clone()).unwrap();
        let p = s.as_ptr();
        std::mem::forget(s);
        p
    }
    else {
        std::ptr::null() 
    };
    ptr
}

fn to_module<'a>(ptr: *mut AAModule) -> &'a mut AAModule {
    unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    }
}

#[no_mangle]
/// init AA module
/// not thread safe
pub extern "C" fn aa_module_init(ptr: *mut AAModule, sample_rate: c_double) {
    let module = to_module(ptr);
    let _ = module.aaunit.init(sample_rate);
}

#[no_mangle]
/// set param for node in graph
pub extern "C" fn set_param_float(ptr: *mut AAModule, node: c_uint, index: c_uint, param: c_float) {
    let module = to_module(ptr);
    let _ = module.sender.send(Command::Param(node, index, param));
}

#[no_mangle]
/// handle note on 
pub extern "C" fn aa_module_handle_note_on(ptr: *mut AAModule, note: c_int, velocity: c_float) {
    let module = to_module(ptr);
    let _ = module.sender.send(Command::NoteOn(note, velocity));
}

#[no_mangle]
/// handle note off
pub extern "C" fn aa_module_handle_note_off(ptr: *mut AAModule, note: c_int, velocity: c_float) {
    let module = to_module(ptr);
    let _ = module.sender.send(Command::NoteOff(note, velocity));
}

#[no_mangle]
/// number of audio inputs
/// not thread safe
pub extern "C" fn aa_module_get_number_inputs(ptr: *mut AAModule) -> c_int {
    let module = to_module(ptr);
    match module.aaunit.get_number_inputs() {
        Ok(v) => v,
        _     => 0
    }
}

#[no_mangle]
/// number of audio outputs
/// not thread safe
pub extern "C" fn aa_module_get_number_outputs(ptr: *mut AAModule) -> c_int {
    let module = to_module(ptr);
    match module.aaunit.get_number_outputs() {
        Ok(v) => v,
        _     => 0
    }
}

fn handle_commands(module: &mut AAModule) {
    for command in module.receiver.try_recv() {
        match command {
            Command::Param(node, index, param) => {
                let _ = module.aaunit.set_param_float(node, index, param);
            },
            Command::NoteOn(note, velocity) => {
                let _ = module.aaunit.handle_note_on(note, velocity);
            },
            Command::NoteOff(note, velocity) => {
                let _ = module.aaunit.handle_note_off(note, velocity);
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn aa_module_compute_zero_one(ptr: *mut AAModule, frames: c_int, outputs: * mut c_float) {
    //let aaunit = to_aaunit(ptr);
    let module = to_module(ptr);

    handle_commands(module);
    
    let outputs = unsafe {
        assert!(!outputs.is_null());
        &mut *outputs
    };

    let outputs = unsafe { 
        std::slice::from_raw_parts_mut(outputs, frames as usize)
    };

    let _ = module.aaunit.compute_zero_one(frames as usize, outputs);
}