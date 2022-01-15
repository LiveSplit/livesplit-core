use crate::Class;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufWriter, Result},
    path::Path,
};

mod code;
mod header;

static MODULE_MAP: &str = include_str!("module.modulemap");
static LIVESPLIT_CORE_C: &str = "/*
 This file exists to make the Swift Package Manager recognize the folder as a
 C target. To compile this, you will need to add a folder containing the
 livesplit_core static library to the linker path.
 */
";

pub fn write<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut path = path.as_ref().to_owned();

    path.push("LiveSplitCore");
    fs::create_dir_all(&path)?;

    path.push("LiveSplitCore.swift");
    code::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.pop();

    path.push("CLiveSplitCore");
    fs::create_dir(&path)?;

    path.push("livesplit_core.c");
    fs::write(&path, LIVESPLIT_CORE_C)?;
    path.pop();

    path.push("include");
    fs::create_dir(&path)?;

    path.push("livesplit_core.h");
    header::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("module.modulemap");
    fs::write(&path, MODULE_MAP).map_err(Into::into)
}
