use std::io::{self, BufRead};
use std::result::Result as StdResult;
use std::num::ParseIntError;
use {Run, time_span, TimeSpan, GameTime, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Empty
        ExpectedCategoryName
        ExpectedAttemptCount
        ExpectedWorldName
        ExpectedWorldTime
        Attempt(err: ParseIntError) {
            from()
        }
        Time(err: time_span::ParseError) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

pub fn parse<R: BufRead>(source: R) -> Result<Run> {
    let mut run = Run::new(Vec::new());

    let mut lines = source.lines();

    let line = lines.next().ok_or(Error::Empty)??;
    let mut splits = line.split('|');
    let category_name = splits.next().ok_or(Error::ExpectedCategoryName)?;
    if !category_name.starts_with('#') {
        return Err(Error::ExpectedCategoryName);
    }
    run.set_category_name(&category_name[1..]);
    run.set_attempt_count(splits.next().ok_or(Error::ExpectedAttemptCount)?.parse()?);
    let mut total_time = TimeSpan::zero();
    let mut next_line = lines.next();
    while let Some(line) = next_line {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let mut splits = line.split('|');
        let world_name = splits.next().ok_or(Error::ExpectedWorldName)?;
        total_time += splits.next().ok_or(Error::ExpectedWorldTime)?.parse()?;
        next_line = lines.next();
        let mut has_acts = false;
        while let Some(line) = next_line {
            let line = line?;
            if line.starts_with('*') {
                run.push_segment(Segment::new(&line[1..]));
                has_acts = true;
                next_line = lines.next();
            } else {
                next_line = Some(Ok(line));
                break;
            }
        }
        let time = GameTime(Some(total_time)).into();
        if has_acts {
            run.segments_mut()
                .last_mut()
                .unwrap()
                .set_personal_best_split_time(time);
        } else {
            let mut segment = Segment::new(world_name);
            segment.set_personal_best_split_time(time);
            run.push_segment(segment);
        }
    }

    Ok(run)
}
