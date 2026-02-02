use crate::{Class, Result, jni_cpp};
use std::{
    collections::BTreeMap,
    fs::{File, create_dir_all},
    io::BufWriter,
    path::Path,
};

mod jni;

pub fn write<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut path = path.as_ref().to_owned();

    path.push("jni");
    create_dir_all(&path)?;
    jni::write(&path, classes)?;
    path.pop();

    path.push("LiveSplitCoreJNI.cpp");
    jni_cpp::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    Ok(())
}
