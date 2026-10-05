//! Provides the parser for LibreSplit (formerly Urn) splits files.

use crate::{
    Lang, Run, Segment, Time, TimeSpan,
    comparison::world_record,
    platform::{
        path::{Path, PathBuf},
        prelude::*,
    },
    settings::Image,
    timing::formatter::{self, TimeFormatter},
};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde::{Deserialize as DeserializeTrait, Deserializer};
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the LibreSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        #[cfg_attr(not(feature = "std"), snafu(source(false)))]
        source: JsonError,
    },
}

/// The Result type for the LibreSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// A LibreSplit time can either be a legacy timestamp string or an object
/// containing separate real-time and game-time timestamps.
#[derive(Deserialize)]
#[serde(untagged)]
enum TimePayload {
    Legacy(TimeValue),
    Separate {
        #[serde(default)]
        real_time: Option<TimeValue>,
        #[serde(default)]
        game_time: Option<TimeValue>,
    },
}

/// A timestamp in a LibreSplit time value. LibreSplit uses `-` for a missing
/// value when the other timing method is present.
struct TimeValue(Option<TimeSpan>);

impl<'de> DeserializeTrait<'de> for TimeValue {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <&str>::deserialize(deserializer)?;
        if value == "-" {
            return Ok(Self(None));
        }

        TimeSpan::parse(value, Lang::English)
            .map(Some)
            .map(Self)
            .map_err(|_| serde::de::Error::custom("invalid LibreSplit time"))
    }
}

impl TimePayload {
    fn into_time(self) -> Time {
        fn nonzero(value: Option<TimeValue>) -> Option<TimeSpan> {
            // Legacy files use zero for an empty time. New files use `-` for
            // the same purpose when only one timing method has a value.
            value
                .and_then(|value| value.0)
                .filter(|value| *value != TimeSpan::zero())
        }

        match self {
            Self::Legacy(value) => Time::new().with_real_time(nonzero(Some(value))),
            Self::Separate {
                real_time,
                game_time,
            } => Time::new()
                .with_real_time(nonzero(real_time))
                .with_game_time(nonzero(game_time)),
        }
    }
}

#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    name: Option<Cow<'a, str>>,
    #[serde(borrow)]
    category: Option<Cow<'a, str>>,
    #[serde(borrow)]
    title: Option<Cow<'a, str>>,
    /// The icon can be a file path or a data URL. Remote URLs aren't loaded.
    #[serde(borrow)]
    icon: Option<Cow<'a, str>>,
    attempt_count: Option<u32>,
    start_delay: Option<TimeSpan>,
    world_record: Option<TimePayload>,
    splits: Option<Vec<Split<'a>>>,
    // TODO: Parse LibreSplit's `comparison_method` once Run can represent the
    // authoritative timing method. Serde intentionally ignores it for now.
}

#[derive(Deserialize)]
struct Split<'a> {
    #[serde(borrow)]
    title: Option<Cow<'a, str>>,
    /// The icon can be a file path or a data URL. Remote URLs aren't loaded.
    #[serde(borrow)]
    icon: Option<Cow<'a, str>>,
    time: Option<TimePayload>,
    best_time: Option<TimePayload>,
    best_segment: Option<TimePayload>,
}

// Match the scheme as the URL parser would, ignoring tabs and newlines.
fn strip_data_url_prefix(source: &str) -> Option<&[u8]> {
    let mut bytes = source.trim_matches(|c| c <= ' ').as_bytes().iter();
    for expected in b"data:" {
        if !bytes
            .find(|&&byte| !matches!(byte, b'\t' | b'\n' | b'\r'))?
            .eq_ignore_ascii_case(expected)
        {
            return None;
        }
    }
    Some(bytes.as_slice())
}

// Decode only the body. Image loading does not need the MIME type or fragment.
fn decode_data_url(source: &[u8], buf: &mut Vec<u8>) -> Option<()> {
    let source = source.split(|&byte| byte == b'#').next()?;
    let comma = source.iter().position(|&byte| byte == b',')?;
    let (header, body) = source.split_at(comma);
    let mut header = header
        .iter()
        .rev()
        .copied()
        .filter(|byte| !matches!(byte, b'\t' | b'\n' | b'\r'))
        .skip_while(|&byte| byte == b' ');
    let base64 = b"base64".iter().rev().all(|expected| {
        header
            .next()
            .is_some_and(|byte| byte.eq_ignore_ascii_case(expected))
    }) && header.find(|&byte| byte != b' ') == Some(b';');

    buf.clear();
    let mut bytes = body[1..]
        .iter()
        .copied()
        .filter(|byte| !matches!(byte, b'\t' | b'\n' | b'\r'));
    while let Some(byte) = bytes.next() {
        let mut hex = bytes.clone();
        if byte == b'%'
            && let Some(high) = hex.next().and_then(|b| (b as char).to_digit(16))
            && let Some(low) = hex.next().and_then(|b| (b as char).to_digit(16))
        {
            buf.push((high * 16 + low) as u8);
            bytes = hex;
        } else {
            // Invalid percent escapes remain literal, as in URL percent decoding.
            buf.push(byte);
        }
    }

    if base64 {
        let len = base64_simd::forgiving_decode_inplace(buf).ok()?.len();
        buf.truncate(len);
    }
    Some(())
}

fn load_icon(
    source: &str,
    #[allow(unused)] load_files_path: Option<&Path>,
    icon_buf: &mut Vec<u8>,
    #[allow(unused)] path_buf: &mut PathBuf,
) -> Option<Image> {
    if let Some(source) = strip_data_url_prefix(source) {
        decode_data_url(source, icon_buf)?;
        return Some(Image::new(icon_buf.as_slice().into(), Image::ICON));
    }

    #[cfg(feature = "std")]
    if !source.is_empty()
        && let Some(load_files_path) = load_files_path
    {
        return Image::from_file(
            crate::platform::path::relative_to(path_buf, load_files_path, Path::new(source)),
            icon_buf,
            Image::ICON,
        )
        .ok();
    }

    None
}

/// Attempts to parse an LibreSplit (formerly Urn) splits file. In addition to
/// the source to parse, you can specify the path to load additional files from
/// the file system. If you are using livesplit-core in a server-like
/// environment, set this to [`None`]. Only client-side applications should
/// provide a path here.
pub fn parse(source: &str, #[allow(unused)] load_files_path: Option<&Path>) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    let mut icon_buf = Vec::new();
    let mut path_buf = Default::default();

    // Legacy files only have a combined title. Like LibreSplit, use it as
    // the game name if no explicit name is provided.
    if let Some(name) = splits.name.or(splits.title) {
        run.set_game_name(name);
    }
    if let Some(category) = splits.category {
        run.set_category_name(category);
    }

    if let Some(icon) = splits.icon
        && let Some(image) = load_icon(&icon, load_files_path, &mut icon_buf, &mut path_buf)
    {
        run.set_game_icon(image);
    }

    if let Some(attempt_count) = splits.attempt_count {
        run.set_attempt_count(attempt_count);
    }
    if let Some(start_delay) = splits.start_delay {
        run.set_offset(-start_delay);
    }
    if let Some(world_record) = splits.world_record.map(TimePayload::into_time)
        && let Some(world_record) = world_record.real_time.or(world_record.game_time)
    {
        run.metadata_mut()
            .custom_variable_mut(world_record::NAME)
            .permanent()
            // FIXME: This should probably depend on the locale or:
            // FIXME: Custom variables should support TimeSpans directly.
            .set_value(
                formatter::Regular::new()
                    .format(Some(world_record), Lang::English)
                    .to_string(),
            );
    }

    // Best Split Times can be used for the Segment History Every single best
    // split time should be included as its own run, since the best split times
    // could be apart from each other less than the best segments, so we have to
    // assume they are from different runs.
    let mut attempt_history_index = 1;

    if let Some(splits) = splits.splits {
        for split in splits {
            let mut segment = Segment::new(split.title.unwrap_or_default());
            if let Some(time) = split.time {
                segment.set_personal_best_split_time(time.into_time());
            }
            if let Some(best_segment) = split.best_segment {
                segment.set_best_segment_time(best_segment.into_time());
            }

            if let Some(best_time) = split.best_time {
                let best_split_time = best_time.into_time();
                if best_split_time.real_time.is_some() || best_split_time.game_time.is_some() {
                    run.add_attempt_with_index(
                        Time::default(),
                        attempt_history_index,
                        None,
                        None,
                        None,
                    );

                    // Insert a new run that skips to the current split
                    for already_inserted_segment in run.segments_mut() {
                        already_inserted_segment
                            .segment_history_mut()
                            .insert(attempt_history_index, Time::default());
                    }

                    segment
                        .segment_history_mut()
                        .insert(attempt_history_index, best_split_time);

                    attempt_history_index += 1;
                }
            }

            if let Some(icon) = split.icon
                && let Some(image) = load_icon(&icon, load_files_path, &mut icon_buf, &mut path_buf)
            {
                segment.set_icon(image);
            }

            run.push_segment(segment);
        }
    }

    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_name_and_category() {
        let run = parse(
            r#"{
                "name": "Castlevania: Symphony of the Night",
                "category": "Any% NSC",
                "splits": [{"title": "Mist"}]
            }"#,
            None,
        )
        .unwrap();

        assert_eq!(run.game_name(), "Castlevania: Symphony of the Night");
        assert_eq!(run.category_name(), "Any% NSC");
        assert_eq!(run.segment(0).name(), "Mist");
    }

    #[test]
    fn legacy_title_is_used_as_game_name() {
        let run = parse(r#"{"title": "SotN Any% NSC"}"#, None).unwrap();

        assert_eq!(run.game_name(), "SotN Any% NSC");
        assert!(run.category_name().is_empty());
    }

    #[test]
    fn explicit_name_takes_precedence_over_legacy_title() {
        for name in ["SotN", ""] {
            let source = serde_json::json!({
                "name": name,
                "category": "Any% NSC",
                "title": "Legacy Title",
            })
            .to_string();
            let run = parse(&source, None).unwrap();

            assert_eq!(run.game_name(), name);
            assert_eq!(run.category_name(), "Any% NSC");
        }
    }

    #[test]
    fn optional_metadata() {
        for (source, game, category) in [
            (r#"{}"#, "", ""),
            (r#"{"name": "SotN"}"#, "SotN", ""),
            (r#"{"category": "Any% NSC"}"#, "", "Any% NSC"),
            (r#"{"name": null, "category": null, "icon": null}"#, "", ""),
            (
                r#"{"name": null, "title": "Legacy Title", "category": "Any%"}"#,
                "Legacy Title",
                "Any%",
            ),
        ] {
            let run = parse(source, None).unwrap();

            assert_eq!(run.game_name(), game);
            assert_eq!(run.category_name(), category);
            assert!(run.game_icon().is_empty());
        }
    }

    #[test]
    fn loads_base64_data_url_icons_without_file_access() {
        let game_icon = br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"/>"#;
        let segment_icon = br#"<svg xmlns="http://www.w3.org/2000/svg" width="2" height="2"/>"#;
        let source = serde_json::json!({
            "icon": format!(
                "data:image/svg+xml;base64,{}",
                base64_simd::STANDARD.encode_to_string(game_icon),
            ),
            "splits": [{
                "title": "Mist",
                "icon": format!(
                    "data:image/svg+xml;charset=utf-8;base64,{}",
                    base64_simd::STANDARD.encode_to_string(segment_icon),
                ),
            }],
        })
        .to_string();
        let run = parse(&source, None).unwrap();

        assert_eq!(run.game_icon().data(), game_icon);
        assert_eq!(run.segment(0).icon().data(), segment_icon);
    }

    #[test]
    fn loads_percent_encoded_data_url_icons() {
        let run = parse(
            r#"{
                "icon": "data:image/svg+xml,%3Csvg%20width=%221%22/%3E",
                "splits": [{
                    "title": "Mist",
                    "icon": "data:image/svg+xml,%3Csvg%20width=%222%22/%3E"
                }]
            }"#,
            None,
        )
        .unwrap();

        assert_eq!(run.game_icon().data(), br#"<svg width="1"/>"#);
        assert_eq!(run.segment(0).icon().data(), br#"<svg width="2"/>"#);
    }

    #[test]
    fn decodes_data_url_bodies() {
        let cases: &[(&str, &[u8])] = &[
            ("data:,", b""),
            ("data:;base64,", b""),
            ("data:;base64,SGVsbG8=", b"Hello"),
            ("data:;base64,SGVsbG8", b"Hello"),
            ("data:;base64,SGVsbG8=#ignored", b"Hello"),
            ("data:;base64,+/8=", &[0xfb, 0xff]),
            ("data:;base64,%2B%2f8%3D", &[0xfb, 0xff]),
            ("data:;base64,%20S%0AGV%0CsbG8=", b"Hello"),
            ("data:;base64,AB==", &[0]),
            ("data:image/png;charset=utf-8; BASE64  ,SGVsbG8=", b"Hello"),
            (
                "\0 \tDa\nTa:\rimage/png; BA\tSE64 \t,SG V\tsbG8=\r\n ",
                b"Hello",
            ),
            ("data:;base64;charset=utf-8,SGVsbG8=", b"SGVsbG8="),
            ("data:;base64=1,SGVsbG8=", b"SGVsbG8="),
            ("data:;base 64,SGVsbG8=", b"SGVsbG8="),
            ("data:;%62ase64,SGVsbG8=", b"SGVsbG8="),
            ("data:,a+b%20c", b"a+b c"),
            ("data:,a,b", b"a,b"),
            ("data:,a%23b#ignored", b"a#b"),
            ("data:,%00%ff%7F", &[0, 255, 127]),
            ("data:,%0%GG%%41", b"%0%GG%A"),
            ("data:,%", b"%"),
            ("data:,%A", b"%A"),
            ("data:,%\r4\n1", b"A"),
            ("data:,a\tb\nc\rd", b"abcd"),
            ("data:,é", "é".as_bytes()),
            ("data:, X ", b" X"),
            ("data:,X%20", b"X "),
        ];
        let mut buf = vec![1, 2, 3];
        for &(source, expected) in cases {
            let body = strip_data_url_prefix(source).unwrap();
            assert_eq!(decode_data_url(body, &mut buf), Some(()), "{source:?}");
            assert_eq!(buf, expected, "{source:?}");
        }
    }

    #[test]
    fn distinguishes_data_urls_from_file_paths() {
        for source in [
            "game.png",
            "data.png",
            "data/icon.png",
            "data :image/png,AA",
        ] {
            assert!(strip_data_url_prefix(source).is_none(), "{source:?}");
        }
    }

    #[test]
    fn ignores_invalid_data_url_icons() {
        for icon in [
            "data:image/png;base64",
            "data:image/png;base64,not-base64",
            "data:image/png;base64,AA=A",
            "DATA:image/png;base64,SGVsbG8===",
            "data:;base64,A",
            "data:image/png#fragment,AA==",
        ] {
            let source = serde_json::json!({
                "name": "SotN",
                "icon": icon,
                "splits": [{"title": "Mist", "icon": icon}],
            })
            .to_string();
            let run = parse(&source, None).unwrap();

            assert_eq!(run.game_name(), "SotN");
            assert_eq!(run.segment(0).name(), "Mist");
            assert!(run.game_icon().is_empty());
            assert!(run.segment(0).icon().is_empty());
        }
    }

    #[cfg(feature = "std")]
    #[test]
    #[cfg_attr(target_os = "wasi", ignore = "temporary directories are not supported")]
    fn loads_game_and_segment_icons() {
        use std::{fs, path::PathBuf, time::SystemTime};

        struct TempDir(PathBuf);

        impl Drop for TempDir {
            fn drop(&mut self) {
                fs::remove_dir_all(&self.0).unwrap();
            }
        }

        let dir = TempDir(std::env::temp_dir().join(format!(
            "livesplit-core-libresplit-icons-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        )));
        fs::create_dir(&dir.0).unwrap();
        fs::create_dir(dir.0.join("icons")).unwrap();

        let game_icon_path = dir.0.join("icons/game.png");
        let segment_icon_path = dir.0.join("icons/segment.png");
        image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 0, 0, 255]))
            .save(&game_icon_path)
            .unwrap();
        image::RgbaImage::from_pixel(1, 1, image::Rgba([0, 255, 0, 255]))
            .save(&segment_icon_path)
            .unwrap();
        let game_icon = fs::read(&game_icon_path).unwrap();
        let segment_icon = fs::read(&segment_icon_path).unwrap();
        let splits_path = dir.0.join("splits.json");

        for icon_path in [Path::new("icons/game.png"), game_icon_path.as_path()] {
            let source = serde_json::json!({
                "name": "SotN",
                "icon": icon_path.to_str().unwrap(),
                "splits": [{"title": "Mist", "icon": "icons/segment.png"}],
            })
            .to_string();
            let run = parse(&source, Some(&splits_path)).unwrap();

            assert_eq!(run.game_icon().data(), game_icon);
            assert_eq!(run.segment(0).icon().data(), segment_icon);

            let run = parse(&source, None).unwrap();
            assert!(run.game_icon().is_empty());
            assert!(run.segment(0).icon().is_empty());
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn ignores_unavailable_game_icons() {
        for icon in ["", "missing-game-icon.png", "https://example.com/game.png"] {
            let source = serde_json::json!({
                "name": "SotN",
                "icon": icon,
                "splits": [{"title": "Mist"}],
            })
            .to_string();
            let run = parse(&source, Some(Path::new("splits.json"))).unwrap();

            assert_eq!(run.game_name(), "SotN");
            assert_eq!(run.segment(0).name(), "Mist");
            assert!(run.game_icon().is_empty());
        }
    }
}
