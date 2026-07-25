use crate::{
    auto_splitting::settings::{
        List as AutoSplitterSettingsList, Map as AutoSplitterSettingsMap,
        Value as AutoSplitterSettingValue,
    },
    platform::prelude::*,
    util::xml::{
        DisplayAlreadyEscaped, NO_ATTRIBUTES, Reader, Writer,
        helper::{
            Error as XmlError, attribute, end_tag, parse_base, parse_children, text,
            text_as_escaped_string_err,
        },
    },
};
use alloc::{format, string::String};
use core::fmt;

/// The ASR-compatible Auto Splitter Settings stored inside a LiveSplit splits
/// file.
///
/// LiveSplit's Auto Splitting Runtime persists both the script path and a
/// recursively typed `CustomSettings` payload inside the splits file. This
/// type intentionally uses the auto splitting runtime's settings map directly:
/// the structured representation is only available with the `auto-splitting`
/// feature, while builds without that feature continue to preserve the raw XML
/// through [`crate::Run::auto_splitter_settings`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StoredAutoSplitterSettings {
    script_path: Option<String>,
    settings_map: AutoSplitterSettingsMap,
}

impl StoredAutoSplitterSettings {
    const FORMAT_VERSION: &str = "1.0";

    /// Creates an empty settings payload.
    #[inline]
    pub fn new() -> Self {
        Self {
            script_path: None,
            settings_map: AutoSplitterSettingsMap::new(),
        }
    }

    /// Returns the stored auto splitter path, if one is configured.
    #[inline]
    pub fn script_path(&self) -> Option<&str> {
        self.script_path.as_deref()
    }

    /// Sets the stored auto splitter path.
    #[inline]
    pub fn set_script_path<S>(&mut self, script_path: Option<S>)
    where
        S: Into<String>,
    {
        self.script_path = script_path.map(Into::into);
    }

    /// Accesses the stored custom settings map.
    #[inline]
    pub const fn settings_map(&self) -> &AutoSplitterSettingsMap {
        &self.settings_map
    }

    /// Grants mutable access to the stored custom settings map.
    #[inline]
    pub const fn settings_map_mut(&mut self) -> &mut AutoSplitterSettingsMap {
        &mut self.settings_map
    }

    /// Replaces the stored custom settings map.
    #[inline]
    pub fn set_settings_map(&mut self, settings_map: AutoSplitterSettingsMap) {
        self.settings_map = settings_map;
    }

    /// Takes ownership of the stored custom settings map.
    #[inline]
    pub fn into_settings_map(self) -> AutoSplitterSettingsMap {
        self.settings_map
    }

    /// Returns whether both the script path and the stored custom settings are
    /// empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.script_path.is_none() && self.settings_map.is_empty()
    }

    /// Parses the XML contents stored inside a LiveSplit
    /// `<AutoSplitterSettings>` element.
    pub fn parse(source: &str) -> Result<Self, StoredAutoSplitterSettingsParseError> {
        if source.trim().is_empty() {
            return Ok(Self::new());
        }

        // The splits parser exposes the interior of the
        // `<AutoSplitterSettings>` element. Wrapping it in the outer tag lets
        // this structured, opt-in parser reuse the existing XML reader without
        // changing the lossless raw-XML behavior of the normal splits parser.
        let wrapped = format!("<AutoSplitterSettings>{source}</AutoSplitterSettings>");
        let mut reader = Reader::new(&wrapped);
        let mut settings = Self::new();

        parse_base(&mut reader, "AutoSplitterSettings", |reader, _| {
            parse_children(reader, |reader, tag, _| match tag.name() {
                "Version" => end_tag::<StoredAutoSplitterSettingsParseError>(reader),
                "ScriptPath" => text(reader, |path| {
                    settings.script_path = Some(path.into_owned())
                }),
                "CustomSettings" => {
                    settings.settings_map = parse_settings_map(reader)?;
                    Ok(())
                }
                _ => end_tag::<StoredAutoSplitterSettingsParseError>(reader),
            })
        })?;

        Ok(settings)
    }

    /// Serializes the settings into the XML contents expected inside a
    /// LiveSplit `<AutoSplitterSettings>` element.
    pub fn write_xml<W: fmt::Write>(&self, writer: W) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let writer = &mut Writer::new_skip_header(writer);
        writer.tag_with_text_content("Version", NO_ATTRIBUTES, Self::FORMAT_VERSION)?;

        if let Some(script_path) = self.script_path() {
            writer.tag_with_text_content("ScriptPath", NO_ATTRIBUTES, script_path)?;
        }

        writer.tag_with_content("CustomSettings", NO_ATTRIBUTES, |writer| {
            write_settings_map(writer, &self.settings_map)
        })?;

        Ok(())
    }

    /// Serializes the settings into a string suitable for
    /// [`crate::Run::auto_splitter_settings_mut`].
    pub fn to_xml_string(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut output = String::new();
        self.write_xml(&mut output)
            .expect("writing supported auto splitter setting values to a string should not fail");
        output
    }
}

fn parse_settings_map(
    reader: &mut Reader<'_>,
) -> Result<AutoSplitterSettingsMap, StoredAutoSplitterSettingsParseError> {
    let mut map = AutoSplitterSettingsMap::new();

    parse_children(reader, |reader, tag, attributes| {
        ensure_setting_element(tag.name())?;

        let mut id = None;
        attribute::<_, StoredAutoSplitterSettingsParseError>(attributes, "id", |value| {
            id = Some(value.into_owned())
        })?;
        let id = id.ok_or(StoredAutoSplitterSettingsParseError::MissingIdAttribute)?;

        map.insert(id.into(), parse_setting_value(reader, attributes)?);
        Ok::<_, StoredAutoSplitterSettingsParseError>(())
    })?;

    Ok(map)
}

fn parse_settings_list(
    reader: &mut Reader<'_>,
) -> Result<AutoSplitterSettingsList, StoredAutoSplitterSettingsParseError> {
    let mut list = AutoSplitterSettingsList::new();

    parse_children(reader, |reader, tag, attributes| {
        ensure_setting_element(tag.name())?;
        list.push(parse_setting_value(reader, attributes)?);
        Ok::<_, StoredAutoSplitterSettingsParseError>(())
    })?;

    Ok(list)
}

fn ensure_setting_element(name: &str) -> Result<(), StoredAutoSplitterSettingsParseError> {
    if name == "Setting" {
        Ok(())
    } else {
        Err(StoredAutoSplitterSettingsParseError::UnexpectedElement {
            name: name.to_owned(),
        })
    }
}

fn parse_setting_value(
    reader: &mut Reader<'_>,
    attributes: crate::util::xml::Attributes<'_>,
) -> Result<AutoSplitterSettingValue, StoredAutoSplitterSettingsParseError> {
    let mut kind = None;
    let mut string_value = None;

    for (key, value) in attributes.iter() {
        match key {
            "type" => kind = Some(value.unescape_str()),
            "value" => string_value = Some(value.unescape_str()),
            _ => {}
        }
    }

    match kind.as_deref() {
        Some("map") => Ok(AutoSplitterSettingValue::Map(parse_settings_map(reader)?)),
        Some("list") => Ok(AutoSplitterSettingValue::List(parse_settings_list(reader)?)),
        Some("bool") => text_as_escaped_string_err(reader, |value| {
            parse_bool(value).map(AutoSplitterSettingValue::Bool)
        }),
        Some("i64") => text_as_escaped_string_err(reader, |value| {
            value
                .parse()
                .map(AutoSplitterSettingValue::I64)
                .map_err(StoredAutoSplitterSettingsParseError::from)
        }),
        Some("f64") => text_as_escaped_string_err(reader, |value| {
            value
                .parse()
                .map(AutoSplitterSettingValue::F64)
                .map_err(StoredAutoSplitterSettingsParseError::from)
        }),
        Some("string") => {
            end_tag::<StoredAutoSplitterSettingsParseError>(reader)?;
            Ok(AutoSplitterSettingValue::String(
                string_value.unwrap_or_default().into(),
            ))
        }
        Some(kind) => Err(StoredAutoSplitterSettingsParseError::UnknownType {
            name: kind.to_owned(),
        }),
        None => Err(StoredAutoSplitterSettingsParseError::MissingTypeAttribute),
    }
}

fn write_settings_map<W: fmt::Write>(
    writer: &mut Writer<W>,
    map: &AutoSplitterSettingsMap,
) -> fmt::Result {
    for (key, value) in map.iter() {
        writer.tag("Setting", |mut tag| {
            tag.attribute("id", key)?;
            write_setting_value(tag, value)
        })?;
    }
    Ok(())
}

fn write_settings_list<W: fmt::Write>(
    writer: &mut Writer<W>,
    list: &AutoSplitterSettingsList,
) -> fmt::Result {
    for value in list.iter() {
        writer.tag("Setting", |tag| write_setting_value(tag, value))?;
    }
    Ok(())
}

fn write_setting_value<W: fmt::Write>(
    mut tag: crate::util::xml::AttributeWriter<'_, W>,
    value: &AutoSplitterSettingValue,
) -> fmt::Result {
    match value {
        AutoSplitterSettingValue::Map(map) => {
            tag.attribute("type", "map")?;
            tag.content(|writer| write_settings_map(writer, map))
        }
        AutoSplitterSettingValue::List(list) => {
            tag.attribute("type", "list")?;
            tag.content(|writer| write_settings_list(writer, list))
        }
        AutoSplitterSettingValue::Bool(value) => {
            tag.attribute("type", "bool")?;
            tag.text_content(bool_text(*value))
        }
        AutoSplitterSettingValue::I64(value) => {
            tag.attribute("type", "i64")?;
            tag.text_content(DisplayAlreadyEscaped(*value))
        }
        AutoSplitterSettingValue::F64(value) => {
            tag.attribute("type", "f64")?;
            tag.text_content(DisplayAlreadyEscaped(*value))
        }
        AutoSplitterSettingValue::String(value) => {
            tag.attribute("type", "string")?;
            tag.attribute("value", value.as_ref())
        }
        // `Value` is non-exhaustive so future runtime value kinds cannot be
        // silently serialized into a different shape. Failing the write is
        // safer than losing a user's setting when the runtime gains a variant
        // that this file format adapter does not understand yet.
        _ => Err(fmt::Error),
    }
}

/// The error returned when parsing the stored ASR-compatible Auto Splitter
/// Settings fails.
#[derive(Debug, snafu::Snafu)]
pub enum StoredAutoSplitterSettingsParseError {
    /// The XML structure itself was malformed.
    Xml {
        /// The underlying XML parsing error.
        source: XmlError,
    },
    /// A `<Setting>` element was missing its `type` attribute.
    MissingTypeAttribute,
    /// A map entry was missing its `id` attribute.
    MissingIdAttribute,
    /// A setting used an unknown `type` discriminator.
    UnknownType {
        /// The unknown type name that was encountered.
        name: String,
    },
    /// An unexpected XML element was encountered while parsing the stored
    /// settings tree.
    UnexpectedElement {
        /// The unexpected element name that was encountered.
        name: String,
    },
    /// Failed to parse an integer value.
    ParseInt {
        /// The underlying integer parsing error.
        source: core::num::ParseIntError,
    },
    /// Failed to parse a floating point value.
    ParseFloat {
        /// The underlying floating point parsing error.
        source: core::num::ParseFloatError,
    },
    /// Failed to parse a boolean value.
    Bool,
}

impl From<XmlError> for StoredAutoSplitterSettingsParseError {
    fn from(source: XmlError) -> Self {
        Self::Xml { source }
    }
}

impl From<core::num::ParseIntError> for StoredAutoSplitterSettingsParseError {
    fn from(source: core::num::ParseIntError) -> Self {
        Self::ParseInt { source }
    }
}

impl From<core::num::ParseFloatError> for StoredAutoSplitterSettingsParseError {
    fn from(source: core::num::ParseFloatError) -> Self {
        Self::ParseFloat { source }
    }
}

fn bool_text(value: bool) -> &'static str {
    if value { "True" } else { "False" }
}

fn parse_bool(value: &str) -> Result<bool, StoredAutoSplitterSettingsParseError> {
    match value {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(StoredAutoSplitterSettingsParseError::Bool),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_payload_roundtrips_as_empty_string() {
        let settings = StoredAutoSplitterSettings::new();

        assert!(settings.is_empty());
        assert_eq!(settings.to_xml_string(), "");
        assert_eq!(
            StoredAutoSplitterSettings::parse("").unwrap(),
            StoredAutoSplitterSettings::new()
        );
    }

    #[test]
    fn parses_and_serializes_reference_shape() {
        // Keep text values on indented lines to mirror common XML formatter
        // output. This explicitly verifies that formatting whitespace is
        // trimmed before paths and scalar settings are interpreted.
        //
        // The Windows-style path is also intentional. Script paths are opaque
        // XML text at this layer and must round-trip unchanged even when these
        // tests run on a non-Windows host.
        let xml = r#"
            <Version>
                1.0
            </Version>
            <ScriptPath>
                C:\Auto Splitters\Game.wasm
            </ScriptPath>
            <CustomSettings>
                <Setting id="outer" type="map">
                    <Setting id="enabled" type="bool">
                        True
                    </Setting>
                    <Setting id="threshold" type="f64">
                        2.5
                    </Setting>
                    <Setting id="choices" type="list">
                        <Setting type="string" value="first"/>
                        <Setting type="i64">
                            7
                        </Setting>
                    </Setting>
                </Setting>
            </CustomSettings>
        "#;

        let parsed = StoredAutoSplitterSettings::parse(xml).unwrap();
        assert_eq!(parsed.script_path(), Some(r"C:\Auto Splitters\Game.wasm"));

        let outer = parsed
            .settings_map()
            .get("outer")
            .expect("outer map should be present");
        assert!(matches!(
            outer,
            AutoSplitterSettingValue::Map(map)
                if matches!(map.get("enabled"), Some(AutoSplitterSettingValue::Bool(true)))
        ));

        assert_eq!(
            StoredAutoSplitterSettings::parse(&parsed.to_xml_string()).unwrap(),
            parsed
        );
    }
}
