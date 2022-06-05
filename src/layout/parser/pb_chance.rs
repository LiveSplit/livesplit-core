use super::{end_tag, parse_children, Result};

pub use crate::component::pb_chance::Component;
use crate::util::xml::Reader;

pub fn settings(reader: &mut Reader<'_>, _: &mut Component) -> Result<()> {
    parse_children(reader, |reader, _, _| -> Result<()> {
        // Unused:
        // AttemptCount
        // UsePercentOfAttempts
        // UseFixedAttempts
        // IgnoreRunCount
        // FIXME:
        // DisplayOdds
        end_tag(reader)
    })?;

    Ok(())
}
