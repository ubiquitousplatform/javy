use std::slice;

use anyhow::{anyhow, Result};
// use std::io::{Read, Write};
use std::ffi::CStr;

use javy::{Runtime, quickjs::JSValue};

use crate::{APIConfig, JSApiSet};

pub(super) struct UbiqFn;

// This block references imported modules from the host environment
#[link(wasm_import_module = "ubiquitous_functions")]
extern "C" {
    fn invoke_json(ptr: i32, size: i32) -> i32; // fn set_output(ptr: i32, size: i32);
    fn invoke_msgpack(ptr: i32, size: i32) -> i32; // fn set_output(ptr: i32, size: i32);
}


// This is an exported rust function that the host environment can call directly
#[no_mangle]
pub extern "C" fn ubiquitous_functions_guest_malloc(requested_size: i32) -> i32 {
    // Print the requested size
    println!("RUST: malloc called with size {}", requested_size);
    // Allocate a buffer of the requested size and then forget it so that it doesn't get cleaned up and then return the pointer.
    let buf = vec![0u8; requested_size as usize];
    let ptr = buf.as_ptr();
    // TODO: does this leak memory? maybe we should store the buffer in a global variable and then free it when the response is received?
    // leak the buffer so that it doesn't get cleaned up
    //Box::leak(buf);

    std::mem::forget(buf);
    ptr as i32
}


impl JSApiSet for UbiqFn {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        // If it doesn't already exist, create and register the UbiqFn object
        let mut ubiq_fn = global.get_property("Ubiquitous")?;
        if ubiq_fn.is_undefined() {
            ubiq_fn = context.object_value()?;
            global.set_property("Ubiquitous", ubiq_fn)?;
        }

        global.set_property(
            "__ubiquitous_functions_invoke_json",
            context.wrap_callback(|_, _this_arg, args| {
                //println!("RUST: Name: {}", name);
                //println!("RUST: Args: {:?}", args);
                //println!("RUST: This: {:?}", this_arg);
                println!("RUST: Made it across to rust!");

                if args.len() != 1 {
                    return Err(anyhow!("Expecting 1 argument, got {}", args.len()));
                }

                let json_string: String = args[0].try_into()?; // receive the string from JS already converted from js object to JSON string
                println!("RUST: json_string = {:?}", json_string);

                /*
                let [fd, data, offset, length, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };
                let mut fd: Box<dyn Read> = match fd.try_into()? {
                    0 => Box::new(std::io::stdin()),
                    _ => anyhow::bail!("Only stdin is supported"),
                };
                let offset: usize = offset.try_into()?;
                let length: usize = length.try_into()?;
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                let data = data.as_bytes_mut()?;
                let data = &mut data[offset..(offset + length)];
                let n = fd.read(data)?; */
                // let n = "Hello From Rust!!";

                // println!("RUST: Calling log function with JSON...");
                // println!("RUST: Calling log function with JSON...");

                // TODO: pass version of API expected?
                // let log = Log {
                //     level: "info".to_string(),
                //     message: "Hello, world!".to_string(),
                //     trace_parent: "trace_parent".to_string(),
                //     span_id: "span_id".to_string(),
                //     props: serde_json::json!({
                //         "name": "John Doe",
                //         "age": 43,
                //         "address": {
                //             "street": "123 Main Street",
                //             "city": "Anytown",
                //             "state": "CA",
                //             "zip": "12345"
                //         }
                //     }),
                //     id: "test".to_string(),
                // };

                // let json_string = serde_json::to_string(&log)?;

                let size = json_string.len() as i32;
                let ptr = json_string.as_ptr();
                std::mem::forget(ptr);

                println!("RUST: calling invoke_json...");
                unsafe {
                    let resp_ptr = invoke_json(ptr as i32, size);
                println!("RUST: invoke_json returned value of {:?}", resp_ptr);

                // retrieve an i32 value from 4 sequential bytes in memory and store in variable
                let response_size = *(resp_ptr as *const i32);
                println!("RUST: response_size = {:?}", response_size);
                
                // store a utf8 string from memory in a variable at location resp + 4
                
                // Assume that `ptr` is a pointer to the memory location containing the string
                // and `len` is the length of the string
                let slice = unsafe { slice::from_raw_parts((resp_ptr + 4) as *const u8, response_size as usize) };
                let raw_json_string = match std::str::from_utf8(slice) {
                    Ok(s) => s.to_owned(),
                    Err(e) => return Err(e.into()),
                };
                println!("RUST: raw_json_string = {:?}", raw_json_string);


                // Possible optimizations: skip from_utf8 validation
                // use C String? (this require iterating)

                //let response_buffer =  CStr::from_ptr(resp + 4);

                
                //let response_buffer =  CStr::from_ptr(resp);
                
                let response_buffer = "";
                
                println!(
                     "response_buffer as a string = {:?}",
                     response_buffer
                 );

                Ok(response_buffer.into())
            }
                /*println!("RUST: invoke_json called! calling get_response_size...");

                let mem_size = unsafe { get_response_size() };

                println!("RUST: get_response_size called! mem_size = {:?}", mem_size);

                let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
                let ptr = buf.as_mut_ptr();
                std::mem::forget(ptr);

                println!("RUST: calling get_response...");
                let response_buffer = unsafe {
                    get_response(ptr as i32);
                    Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
                };

                println!(
                    "get_response completed! response_buffer = {:?}",
                    response_buffer
                );*/

                /*let response: OkResponse =
                    serde_json::from_slice(&response_buffer).map_err(|e| {
                        eprintln!("RUST: ser: {e}");
                        e
                    })?;

                println!("RUST: response = {:?}", response);*/
                // Probably want to convert it back into a string before returning it?

                //Ok(response_buffer.into())

                //Ok(n.into())

            })?,
        )?;

        context.eval_global("ubiq_fn.js", include_str!("ubiq_fn.js"))?;
        Ok(())
    }
}
