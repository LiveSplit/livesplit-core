use {TimeSpan, Timer};

mod balanced_pb;
mod empty;
mod median;

fn run_with_splits(timer: &mut Timer, splits: &[f64]) {
    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    for &split in splits {
        timer.set_game_time(TimeSpan::from_seconds(split));
        timer.split();
    }

    timer.reset(true);
}

fn run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    for &split in splits {
        if let Some(split) = split {
            timer.set_game_time(TimeSpan::from_seconds(split));
            timer.split();
        } else {
            timer.skip_split();
        }
    }

    timer.reset(true);
}
