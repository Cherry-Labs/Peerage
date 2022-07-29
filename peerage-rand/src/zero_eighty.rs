use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand_between_0_and_80() -> usize {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    

    let mut duration = since_the_epoch.as_nanos();

    duration += duration + 80;
    duration *= duration - 80;

    (duration % 80) as usize
}