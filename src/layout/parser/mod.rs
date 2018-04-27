mod xml_util;
pub use self::xml_util::{Error, Result};

use std::io::BufRead;
use self::xml_util::{end_tag, parse_base, parse_children, text, text_err, text_parsed, Tag};
use super::{Component, Layout};
use component::{blank_space, current_comparison, current_pace, delta, detailed_timer, graph,
                possible_time_save, previous_segment, separator, splits, sum_of_best, text, timer,
                title, total_playtime};
use quick_xml::reader::Reader;
use settings::{Color, Gradient};
use time::formatter::Accuracy;

enum GradientKind {
    Transparent,
    Plain,
    Vertical,
    Horizontal,
}

struct GradientBuilder {
    kind: GradientKind,
    first: Color,
    second: Color,
}

impl GradientBuilder {
    fn new() -> Self {
        Self {
            kind: GradientKind::Transparent,
            first: Color::transparent(),
            second: Color::transparent(),
        }
    }

    fn parse_background<'a, R>(
        &mut self,
        reader: &mut Reader<R>,
        tag: Tag<'a>,
    ) -> Result<Option<Tag<'a>>>
    where
        R: BufRead,
    {
        if tag.name() == b"BackgroundColor" {
            color(reader, tag.into_buf(), |c| self.first = c)?;
        } else if tag.name() == b"BackgroundColor2" {
            color(reader, tag.into_buf(), |c| self.second = c)?;
        } else if tag.name() == b"BackgroundGradient" {
            text_err(reader, tag.into_buf(), |text| {
                self.kind = match &*text {
                    "Plain" => GradientKind::Plain,
                    "Vertical" => GradientKind::Vertical,
                    "Horizontal" => GradientKind::Horizontal,
                    _ => return Err(Error::UnexpectedGradientType),
                };
                Ok(())
            })?;
        } else {
            return Ok(Some(tag));
        }
        Ok(None)
    }

    fn build(self) -> Gradient {
        match self.kind {
            GradientKind::Transparent => Gradient::Transparent,
            GradientKind::Plain => {
                if self.first == Color::transparent() {
                    Gradient::Transparent
                } else {
                    Gradient::Plain(self.first)
                }
            }
            GradientKind::Horizontal => Gradient::Horizontal(self.first, self.second),
            GradientKind::Vertical => Gradient::Vertical(self.first, self.second),
        }
    }
}

fn color<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Color),
{
    text_err(reader, buf, |text| {
        let n = u32::from_str_radix(&text, 16)?;
        let b = (n & 0xFF) as u8;
        let g = ((n >> 8) & 0xFF) as u8;
        let r = ((n >> 16) & 0xFF) as u8;
        let a = ((n >> 24) & 0xFF) as u8;
        f(Color::from([r, g, b, a]));
        Ok(())
    })
}

fn parse_bool<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(bool),
{
    text_err(reader, buf, |t| match &*t {
        "True" => {
            f(true);
            Ok(())
        }
        "False" => {
            f(false);
            Ok(())
        }
        _ => Err(Error::Bool),
    })
}

fn accuracy<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Accuracy),
{
    text_err(reader, buf, |t| {
        f(match &*t {
            "Tenths" => Accuracy::Tenths,
            "Seconds" => Accuracy::Seconds,
            "Hundredths" => Accuracy::Hundredths,
            _ => return Err(Error::Accuracy),
        });
        Ok(())
    })
}

fn blank_space_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut blank_space::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"SpaceHeight" {
                text_parsed(reader, tag.into_buf(), |h| settings.height = h)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    settings.background = background_builder.build();

    Ok(())
}

fn current_comparison_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut current_comparison::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn current_pace_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut current_pace::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Comparison" {
                text(reader, tag.into_buf(), |t| {
                    settings.comparison_override = if t == "Current Comparison" {
                        None
                    } else {
                        Some(t.into_owned())
                    };
                })
            } else if tag.name() == b"Accuracy" {
                accuracy(reader, tag.into_buf(), |a| settings.accuracy = a)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn component<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Component),
{
    let mut component = None;

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"Path" {
            text_err(reader, tag.into_buf(), |text| {
                component = Some(match &*text {
                    "LiveSplit.BlankSpace.dll" => blank_space::Component::new().into(),
                    "LiveSplit.CurrentComparison.dll" => {
                        current_comparison::Component::new().into()
                    }
                    "LiveSplit.RunPrediction.dll" => current_pace::Component::new().into(),
                    "LiveSplit.Delta.dll" => delta::Component::new().into(),
                    "LiveSplit.DetailedTimer.dll" => detailed_timer::Component::new().into(),
                    "LiveSplit.Graph.dll" => graph::Component::new().into(),
                    "LiveSplit.PossibleTimeSave.dll" => possible_time_save::Component::new().into(),
                    "LiveSplit.PreviousSegment.dll" => previous_segment::Component::new().into(),
                    "" => separator::Component::new().into(),
                    "LiveSplit.Splits.dll" => splits::Component::new().into(),
                    "LiveSplit.SumOfBest.dll" => sum_of_best::Component::new().into(),
                    "LiveSplit.Text.dll" => text::Component::new().into(),
                    "LiveSplit.Timer.dll" => timer::Component::new().into(),
                    "LiveSplit.Title.dll" => title::Component::new().into(),
                    "LiveSplit.TotalPlaytime.dll" => total_playtime::Component::new().into(),
                    _ => return Ok(()),
                });
                Ok(())
            })
        } else if tag.name() == b"Settings" {
            // Assumption: Settings always has to come after the Path.
            // Otherwise we need to cache the settings and load them later.
            if let Some(ref mut component) = component {
                match *component {
                    Component::BlankSpace(ref mut c) => {
                        blank_space_settings(reader, tag.into_buf(), c)
                    }
                    Component::CurrentComparison(ref mut c) => {
                        current_comparison_settings(reader, tag.into_buf(), c)
                    }
                    Component::CurrentPace(ref mut c) => {
                        current_pace_settings(reader, tag.into_buf(), c)
                    }
                    // _ => unimplemented!(),
                    _ => end_tag(reader, tag.into_buf()), // ignore other components for now
                }
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            end_tag(reader, tag.into_buf())
        }
    })?;

    if let Some(component) = component {
        // TODO Parse Component Settings
        f(component);
    }

    Ok(())
}

fn parse_general_settings<R: BufRead>(
    layout: &mut Layout,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<()> {
    let settings = layout.general_settings_mut();
    let (mut bg1, mut bg2) = (Color::transparent(), Color::transparent());

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"TextColor" {
            color(reader, tag.into_buf(), |color| {
                settings.text_color = color;
            })
        } else if tag.name() == b"BackgroundColor" {
            color(reader, tag.into_buf(), |color| {
                bg1 = color;
            })
        } else if tag.name() == b"BackgroundColor2" {
            color(reader, tag.into_buf(), |color| {
                bg2 = color;
            })
        } else if tag.name() == b"ThinSeparatorsColor" {
            color(reader, tag.into_buf(), |color| {
                settings.thin_separators_color = color;
            })
        } else if tag.name() == b"SeparatorsColor" {
            color(reader, tag.into_buf(), |color| {
                settings.separators_color = color;
            })
        } else if tag.name() == b"PersonalBestColor" {
            color(reader, tag.into_buf(), |color| {
                settings.personal_best_color = color;
            })
        } else if tag.name() == b"AheadGainingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.ahead_gaining_time_color = color;
            })
        } else if tag.name() == b"AheadLosingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.ahead_losing_time_color = color;
            })
        } else if tag.name() == b"BehindGainingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.behind_gaining_time_color = color;
            })
        } else if tag.name() == b"BehindLosingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.behind_losing_time_color = color;
            })
        } else if tag.name() == b"BestSegmentColor" {
            color(reader, tag.into_buf(), |color| {
                settings.best_segment_color = color;
            })
        } else if tag.name() == b"NotRunningColor" {
            color(reader, tag.into_buf(), |color| {
                settings.not_running_color = color;
            })
        } else if tag.name() == b"PausedColor" {
            color(reader, tag.into_buf(), |color| {
                settings.paused_color = color;
            })
        } else if tag.name() == b"BackgroundType" {
            // TODO Background stuff
            end_tag(reader, tag.into_buf())
        } else {
            end_tag(reader, tag.into_buf())
        }
    })

    // TODO Post Process Background color
}

pub fn parse<R: BufRead>(source: R) -> Result<Layout> {
    let reader = &mut Reader::from_reader(source);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    let mut buf = Vec::with_capacity(4096);
    // let mut buf2 = Vec::with_capacity(4096);

    let mut layout = Layout::new();

    parse_base(reader, &mut buf, b"Layout", |reader, tag| {
        parse_children(reader, tag.into_buf(), |reader, tag| {
            if tag.name() == b"Settings" {
                parse_general_settings(&mut layout, reader, tag.into_buf())
            } else if tag.name() == b"Components" {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    // We kinda assume everything in here is a component I guess?
                    component(reader, tag.into_buf(), |c| {
                        layout.push(c);
                    })
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        })
    })?;

    // TODO Validate

    Ok(layout)
}
