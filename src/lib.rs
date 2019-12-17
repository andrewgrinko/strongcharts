use std::error::Error;

use serde::{Serialize, Deserialize};
use itertools::Itertools;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    date: String,
    #[serde(rename = "Workout Name")]
    workout_name: String,
    #[serde(rename = "Exercise Name")]
    exercise_name: String,
    #[serde(rename = "Set Order")]
    set_order: String,
    weight: f64,
    reps: u32,
    distance: f64,
    seconds: u32
}

#[derive(Debug, Serialize)]
pub struct ExerciseStats {
    exercise_name: String,
    records: Vec<Record>
}

#[derive(Debug, Serialize)]
pub struct Stats {
    exercises: Vec<ExerciseStats>
}

pub fn read_csv(str: &str) -> Result<Stats, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(str.as_bytes());
    let mut rows: Vec<Record> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        rows.push(record);
    }

    let grouped = &rows
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&b.exercise_name, &a.exercise_name))
        .group_by(|o| o.exercise_name.clone());

    let mut result = Stats { exercises: Vec::new() };

    for (key, group) in grouped {
        let exercise_stats = ExerciseStats {
            exercise_name: key,
            records: group.sorted_by(|a, b| Ord::cmp(&a.date, &b.date)).collect()
        };
        result.exercises.push(exercise_stats);
    }

    Ok(result)
}
