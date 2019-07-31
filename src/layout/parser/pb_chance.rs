use super::{end_tag, parse_children, Result};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::pb_chance::Component;

pub fn settings<R>(reader: &mut Reader<R>, buf: &mut Vec<u8>, _: &mut Component) -> Result<()>
where
    R: BufRead,
{
    parse_children(reader, buf, |reader, tag| -> Result<()> {
        // Unused:
        // AttemptCount
        // UsePercentOfAttempts
        // UseFixedAttempts
        // IgnoreRunCount
        // FIXME:
        // DisplayOdds
        end_tag(reader, tag.into_buf())
    })?;

    Ok(())
}
