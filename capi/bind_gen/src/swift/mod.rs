use crate::{Class, Result};
use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

mod code;
mod header;

static MODULE_MAP: &str = include_str!("module.map");

pub fn write<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut path = path.as_ref().to_owned();

    path.push("LiveSplitCoreNative");
    create_dir_all(&path)?;

    path.push("livesplit_core.h");
    header::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("module.map");
    write!(BufWriter::new(File::create(&path)?), "{}", MODULE_MAP)?;
    path.pop();

    path.pop();

    path.push("LiveSplitCore.swift");
    code::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    Ok(())
}
