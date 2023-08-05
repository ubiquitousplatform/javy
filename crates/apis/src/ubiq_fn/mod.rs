use anyhow::{anyhow, Result};
// use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct UbiqFn;

#[link(wasm_import_module = "ubiquitous_functions")]
extern "C" {
    fn get_response_size() -> i32; // fn get_input_size() -> i32;
    fn get_response(ptr: i32); // fn get_input(ptr: i32);
    fn invoke_json(ptr: i32, size: i32); // fn set_output(ptr: i32, size: i32);
    fn invoke_msgpack(ptr: i32, size: i32); // fn set_output(ptr: i32, size: i32);
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
                //println!("Name: {}", name);
                //println!("Args: {:?}", args);
                //println!("This: {:?}", this_arg);
                println!("Made it across to rust!");

                if args.len() != 1 {
                    return Err(anyhow!("Expecting 1 argument, got {}", args.len()));
                }

                let json_string: String = args[0].try_into()?; // receive the string from JS already converted from js object to JSON string
                println!("json_string = {:?}", json_string);

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

                // println!("Calling log function with JSON...");
                // println!("Calling log function with JSON...");

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

                println!("calling invoke_json...");
                unsafe {
                    invoke_json(ptr as i32, size);
                }

                println!("invoke_json called! calling get_response_size...");

                let mem_size = unsafe { get_response_size() };

                println!("get_response_size called! mem_size = {:?}", mem_size);

                let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
                let ptr = buf.as_mut_ptr();
                std::mem::forget(ptr);

                println!("calling get_response...");
                let response_buffer = unsafe {
                    get_response(ptr as i32);
                    Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
                };

                println!(
                    "get_response completed! response_buffer = {:?}",
                    response_buffer
                );

                /*let response: OkResponse =
                    serde_json::from_slice(&response_buffer).map_err(|e| {
                        eprintln!("ser: {e}");
                        e
                    })?;

                println!("response = {:?}", response);*/
                // Probably want to convert it back into a string before returning it?
                Ok(response_buffer.into())
                //Ok(n.into())
            })?,
        )?;

        context.eval_global("ubiq_fn.js", include_str!("ubiq_fn.js"))?;
        Ok(())
    }
}
