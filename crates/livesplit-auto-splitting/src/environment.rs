use crate::{
    pointer::{PointerType, PointerValue},
    process::{Offset, Process},
};
use num_traits::FromPrimitive;
use std::{collections::HashMap, error::Error, mem, str, time::Duration};
use wasmtime::{Memory, Trap};

pub struct Environment {
    pub memory: Option<Memory>,
    pub process_name: String,
    pointer_paths: Vec<PointerPath>,
    pub tick_rate: Duration,
    pub process: Option<Process>,
    pub variable_changes: HashMap<String, String>,
}

struct PointerPath {
    module_name: String,
    offsets: Vec<i64>,
    current: PointerValue,
    old: PointerValue,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            memory: None,
            process_name: String::new(),
            pointer_paths: Vec::new(),
            tick_rate: Duration::from_secs(1) / 60,
            process: None,
            variable_changes: HashMap::new(),
        }
    }
}

fn trap_from_err(e: impl Error + Send + Sync + 'static) -> Trap {
    Trap::new(anyhow::Error::from(e).to_string())
}

fn get_bytes(memory: &mut Option<Memory>, ptr: i32, len: i32) -> Result<&mut [u8], Trap> {
    let memory = unsafe {
        memory
            .as_mut()
            .ok_or_else(|| Trap::new("There is no memory to use"))?
            .data_unchecked_mut()
    };

    let ptr = ptr as u32 as usize;
    let len = len as u32 as usize;

    memory
        .get_mut(ptr..ptr + len)
        .ok_or_else(|| Trap::new("Index out of bounds"))
}

fn read_str(memory: &mut Option<Memory>, ptr: i32, len: i32) -> Result<&str, Trap> {
    let bytes = get_bytes(memory, ptr, len)?;
    str::from_utf8(bytes).map_err(trap_from_err)
}

impl Environment {
    pub fn set_process_name(&mut self, ptr: i32, len: i32) -> Result<(), Trap> {
        let process_name = read_str(&mut self.memory, ptr, len)?;
        self.process_name.clear();
        self.process_name.push_str(process_name);
        Ok(())
    }

    pub fn push_pointer_path(
        &mut self,
        ptr: i32,
        len: i32,
        pointer_type: i32,
    ) -> Result<i32, Trap> {
        let pointer_type = PointerType::from_u32(pointer_type as u32)
            .ok_or_else(|| Trap::new("Invalid pointer type"))
            .unwrap();

        let current = match pointer_type {
            PointerType::U8 => PointerValue::U8(0),
            PointerType::U16 => PointerValue::U16(0),
            PointerType::U32 => PointerValue::U32(0),
            PointerType::U64 => PointerValue::U64(0),
            PointerType::I8 => PointerValue::I8(0),
            PointerType::I16 => PointerValue::I16(0),
            PointerType::I32 => PointerValue::I32(0),
            PointerType::I64 => PointerValue::I64(0),
            PointerType::F32 => PointerValue::F32(0.0),
            PointerType::F64 => PointerValue::F64(0.0),
            PointerType::String => PointerValue::String(String::new()),
        };

        let module_name = read_str(&mut self.memory, ptr, len)?.to_owned();

        let id = self.pointer_paths.len();

        self.pointer_paths.push(PointerPath {
            module_name,
            offsets: Vec::new(),
            old: current.clone(),
            current,
        });

        Ok(id as _)
    }

    pub fn push_offset(&mut self, pointer_path_id: i32, offset: i64) -> Result<(), Trap> {
        self.pointer_paths
            .get_mut(pointer_path_id as u32 as usize)
            .ok_or_else(|| Trap::new("Specified invalid pointer path"))?
            .offsets
            .push(offset);

        Ok(())
    }

    pub fn get_val<T>(
        &self,
        pointer_path_id: i32,
        current: i32,
        convert: impl FnOnce(&PointerValue) -> Option<T>,
    ) -> Result<T, Trap> {
        let pointer_path = self
            .pointer_paths
            .get(pointer_path_id as u32 as usize)
            .ok_or_else(|| Trap::new("Specified invalid pointer path"))?;

        let value = if current != 0 {
            &pointer_path.current
        } else {
            &pointer_path.old
        };

        convert(value).ok_or_else(|| Trap::new("The types did not match"))
    }

    pub fn scan_signature(&mut self, ptr: i32, len: i32) -> Result<i64, Trap> {
        // TODO: Don't trap
        if let Some(process) = &self.process {
            let signature = read_str(&mut self.memory, ptr, len)?;
            let address = process.scan_signature(signature).map_err(trap_from_err)?;
            return Ok(address.unwrap_or(0) as i64);
        }

        Ok(0)
    }

    pub fn set_tick_rate(&mut self, ticks_per_sec: f64) {
        log::info!("New Tick Rate: {}", ticks_per_sec);
        self.tick_rate = Duration::from_secs_f64(1.0 / ticks_per_sec);
    }

    pub fn print_message(&mut self, ptr: i32, len: i32) -> Result<(), Trap> {
        let message = read_str(&mut self.memory, ptr, len)?;
        log::info!(target: "Auto Splitter", "{}", message);
        Ok(())
    }

    pub fn read_into_buf(&mut self, address: i64, buf: i32, buf_len: i32) -> Result<i32, Trap> {
        if let Some(process) = &self.process {
            if process
                .read_buf(address as u64, get_bytes(&mut self.memory, buf, buf_len)?)
                .is_err()
            {
                // TODO: possibly handle this error in a more robust way
                return Ok(0);
            }
        }
        Ok(1)
    }

    pub fn set_variable(
        &mut self,
        key_ptr: i32,
        key_len: i32,
        value_ptr: i32,
        value_len: i32,
    ) -> Result<(), Trap> {
        let key = read_str(&mut self.memory, key_ptr, key_len)?.to_owned();
        let value = read_str(&mut self.memory, value_ptr, value_len)?.to_owned();
        self.variable_changes.insert(key, value);
        Ok(())
    }

    pub fn update_values(&mut self, just_connected: bool) -> anyhow::Result<()> {
        let process = self
            .process
            .as_mut()
            .expect("The process should be connected at this point");

        for pointer_path in &mut self.pointer_paths {
            let mut address = if !pointer_path.module_name.is_empty() {
                process.module_address(&pointer_path.module_name)?
            } else {
                0
            };
            let mut offsets = pointer_path.offsets.iter().cloned().peekable();
            if process.is_64bit() {
                while let Some(offset) = offsets.next() {
                    address = (address as Offset).wrapping_add(offset) as u64;
                    if offsets.peek().is_some() {
                        address = process.read(address)?;
                    }
                }
            } else {
                while let Some(offset) = offsets.next() {
                    address = (address as i32).wrapping_add(offset as i32) as u64;
                    if offsets.peek().is_some() {
                        address = process.read::<u32>(address)? as u64;
                    }
                }
            }
            match &mut pointer_path.old {
                PointerValue::U8(v) => *v = process.read(address)?,
                PointerValue::U16(v) => *v = process.read(address)?,
                PointerValue::U32(v) => *v = process.read(address)?,
                PointerValue::U64(v) => *v = process.read(address)?,
                PointerValue::I8(v) => *v = process.read(address)?,
                PointerValue::I16(v) => *v = process.read(address)?,
                PointerValue::I32(v) => *v = process.read(address)?,
                PointerValue::I64(v) => *v = process.read(address)?,
                PointerValue::F32(v) => *v = process.read(address)?,
                PointerValue::F64(v) => *v = process.read(address)?,
                PointerValue::String(_) => todo!(),
            }
        }

        if just_connected {
            for pointer_path in &mut self.pointer_paths {
                pointer_path.current.clone_from(&pointer_path.old);
            }
        } else {
            for pointer_path in &mut self.pointer_paths {
                mem::swap(&mut pointer_path.current, &mut pointer_path.old);
            }
        }

        Ok(())
    }
}
