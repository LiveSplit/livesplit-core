//! Provide this as an object containing both the run and the splits information
//! So it should roughly look like this:
//!
//! ```json
//! {
//!     "run": {
//!         "id": "957",
//!         ...
//!     },
//!     "splits": [
//!         { "name": "Level 1", ... },
//!         ...
//!     ]
//! }
//! ```

use std::io::Read;
use std::result::Result as StdResult;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use {Run, TimeSpan, RealTime, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Json(err: JsonError) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct ApiObject {
    run: ApiRun,
    splits: Vec<Split>,
}

#[derive(Deserialize)]
struct ApiRun {
    attempts: Option<u32>,
    game: Option<Game>,
    category: Option<Category>,
}

#[derive(Deserialize)]
struct Game {
    name: String,
}

#[derive(Deserialize)]
struct Category {
    name: String,
}

#[derive(Deserialize)]
struct Split {
    name: String,
    finish_time: f64,
    best: Option<f64>,
    skipped: bool,
}

pub fn parse<R: Read>(source: R) -> Result<Run> {
    let mut run = Run::new(Vec::new());

    let obj: ApiObject = from_reader(source)?;

    if let Some(attempts) = obj.run.attempts {
        run.set_attempt_count(attempts);
    }
    if let Some(game) = obj.run.game {
        run.set_game_name(game.name);
    }
    if let Some(category) = obj.run.category {
        run.set_category_name(category.name);
    }

    for split in obj.splits {
        let mut segment = Segment::new(split.name);

        if let Some(best) = split.best {
            segment.set_best_segment_time(RealTime(Some(TimeSpan::from_seconds(best))).into());
        }

        if !split.skipped {
            segment.set_personal_best_split_time(RealTime(Some(TimeSpan::from_seconds(split.finish_time))).into());
        }

        run.push_segment(segment);
    }

    Ok(run)
}
