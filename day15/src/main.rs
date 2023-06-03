use anyhow::{Context, Error, Result};
use range_collections::range_set::RangeSetRange;
use range_collections::RangeSet;
use std::io::{BufRead, BufReader};

fn part1() -> Result<()> {
    let r = regex::Regex::new(
        r#"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"#,
    )?;

    let relevant_y = 2_000_000;
    let mut not_beacon = RangeSet::<[i32; 4]>::empty();

    let input = std::fs::File::open("day15/input.txt")?;
    for result in BufReader::new(input).lines() {
        let line = result?;
        if line.trim().is_empty() {
            continue;
        }
        let captures = r.captures(line.trim()).context("no matches")?;
        let sx = captures.get(1).context("no sx")?.as_str().parse::<i32>()?;
        let sy = captures.get(2).context("no sy")?.as_str().parse::<i32>()?;
        let bx = captures.get(3).context("no bx")?.as_str().parse::<i32>()?;
        let by = captures.get(4).context("no by")?.as_str().parse::<i32>()?;

        let dx = (sx - bx).abs();
        let dy = (sy - by).abs();
        let radius = dx + dy;

        let dist = (relevant_y - sy).abs();

        if dist <= radius {
            let relevant_range = radius - dist;
            let new_range =
                RangeSet::<[i32; 2]>::from((sx - relevant_range)..(sx + relevant_range + 1));
            not_beacon.union_with(new_range.as_ref());

            if by == relevant_y {
                let beacon = RangeSet::<[i32; 2]>::from((bx)..(bx + 1));
                not_beacon.symmetric_difference_with(beacon.as_ref());
            }
        }
    }

    let mut num_locs = 0;
    for range2 in not_beacon.iter() {
        let RangeSetRange::Range(range) = range2 else {
            return Err(Error::msg("infinite range"));
        };
        num_locs += *range.end - *range.start;
    }

    println!("{num_locs}");

    Ok(())
}

fn part2() -> Result<()> {
    struct SensorRange {
        x: i32,
        y: i32,
        r: i32,
    }

    let r = regex::Regex::new(
        r#"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"#,
    )?;

    let mut sensor_ranges = Vec::new();

    let input = std::fs::File::open("day15/input.txt")?;
    for result in BufReader::new(input).lines() {
        let line = result?;
        if line.trim().is_empty() {
            continue;
        }
        let captures = r.captures(line.trim()).context("no matches")?;
        let sx = captures.get(1).context("no sx")?.as_str().parse::<i32>()?;
        let sy = captures.get(2).context("no sy")?.as_str().parse::<i32>()?;
        let bx = captures.get(3).context("no bx")?.as_str().parse::<i32>()?;
        let by = captures.get(4).context("no by")?.as_str().parse::<i32>()?;

        let dx = (sx - bx).abs();
        let dy = (sy - by).abs();
        let radius = dx + dy;

        sensor_ranges.push(SensorRange {
            x: sx,
            y: sy,
            r: radius,
        });
    }

    let min = 0;
    let max = 4_000_000;

    let relevant_range = RangeSet::<[i32; 2]>::from(min..(max + 1));

    for y in min..=max {
        let mut not_beacon = RangeSet::<[i32; 4]>::empty();
        for sensor_range in sensor_ranges.iter() {
            let dist = (y - sensor_range.y).abs();

            if dist <= sensor_range.r {
                let relevant_range = sensor_range.r - dist;
                let new_range = RangeSet::<[i32; 2]>::from(
                    (sensor_range.x - relevant_range)..(sensor_range.x + relevant_range + 1),
                );
                not_beacon.union_with(new_range.as_ref());
            }
        }

        let mut beacon = not_beacon;
        beacon.intersection_with(relevant_range.as_ref());
        beacon.symmetric_difference_with(relevant_range.as_ref());
        for range2 in beacon.iter() {
            let RangeSetRange::Range(range) = range2 else {
                return Err(Error::msg("infinite range"));
            };
            for x in *range.start..*range.end {
                println!("x={x} y={y} => {}", (x as u64) * 4_000_000 + (y as u64));
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()
}
