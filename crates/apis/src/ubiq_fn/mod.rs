use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct UbiqFn;

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
            "__ubiquitous_functions_invoke",
            context.wrap_callback(|_, _this_arg, args| {
                //println!("Name: {}", name);
                //println!("Args: {:?}", args);
                //println!("This: {:?}", this_arg);
                println!("Made it across to rust!");
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
                let n = "Hello From Rust!!";
                Ok(n.into())
                //Ok(n.into())
            })?,
        )?;

        context.eval_global("ubiq_fn.js", include_str!("ubiq_fn.js"))?;
        Ok(())
    }
}
