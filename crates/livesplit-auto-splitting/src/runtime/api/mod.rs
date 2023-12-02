use std::str;

use anyhow::{format_err, Context as _, Result};
use wasmtime::{Caller, Linker};

use crate::{CreationError, Timer};

use super::Context;

mod process;
mod runtime;
mod setting_value;
mod settings_list;
mod settings_map;
mod timer;
mod user_settings;

pub mod wasi;

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    timer::bind(linker)?;
    runtime::bind(linker)?;
    process::bind(linker)?;
    user_settings::bind(linker)?;
    settings_map::bind(linker)?;
    settings_list::bind(linker)?;
    setting_value::bind(linker)?;
    Ok(())
}

fn memory_and_context<'a, T: Timer>(
    caller: &'a mut Caller<'_, Context<T>>,
) -> (&'a mut [u8], &'a mut Context<T>) {
    caller.data().memory.unwrap().data_and_store_mut(caller)
}

fn get_arr_mut<const N: usize>(memory: &mut [u8], ptr: u32) -> Result<&mut [u8; N]> {
    assert!(N <= u32::MAX as usize);
    Ok(get_slice_mut(memory, ptr, N as _)?.try_into().unwrap())
}

fn get_slice(memory: &[u8], ptr: u32, len: u32) -> Result<&[u8]> {
    memory
        .get(ptr as usize..)
        .context("Out of bounds pointer and length pair.")?
        .get(..len as usize)
        .context("Out of bounds pointer and length pair.")
}

fn get_slice_mut(memory: &mut [u8], ptr: u32, len: u32) -> Result<&mut [u8]> {
    memory
        .get_mut(ptr as usize..)
        .context("Out of bounds pointer and length pair.")?
        .get_mut(..len as usize)
        .context("Out of bounds pointer and length pair.")
}

fn get_str(memory: &[u8], ptr: u32, len: u32) -> Result<&str> {
    let slice = get_slice(memory, ptr, len)?;
    str::from_utf8(slice).map_err(Into::into)
}

fn get_two_slice_mut(
    memory: &mut [u8],
    ptr1: u32,
    len1: u32,
    ptr2: u32,
    len2: u32,
) -> Result<[&mut [u8]; 2]> {
    let (ptr1, ptr2) = (ptr1 as usize, ptr2 as usize);
    let (len1, len2) = (len1 as usize, len2 as usize);
    if ptr1 < ptr2 {
        if ptr2 >= memory.len() {
            return Err(format_err!("Out of bounds pointer and length pair."));
        }
        let (first, second) = memory.split_at_mut(ptr2);
        Ok([
            first
                .get_mut(ptr1..)
                .context("Out of bounds pointer and length pair.")?
                .get_mut(..len1)
                .context("Overlapping pair of pointer ranges.")?,
            second
                .get_mut(..len2)
                .context("Out of bounds pointer and length pair.")?,
        ])
    } else {
        if ptr1 >= memory.len() {
            return Err(format_err!("Out of bounds pointer and length pair."));
        }
        let (first, second) = memory.split_at_mut(ptr1);
        Ok([
            second
                .get_mut(..len1)
                .context("Out of bounds pointer and length pair.")?,
            first
                .get_mut(ptr2..)
                .context("Out of bounds pointer and length pair.")?
                .get_mut(..len2)
                .context("Overlapping pair of pointer ranges.")?,
        ])
    }
}
