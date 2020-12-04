use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::iter::FromIterator;

pub fn day3a(inputs: &[String]) -> anyhow::Result<String> {
    let width: u128 = inputs[0].chars().count().try_into().unwrap();
    let height: u128 = inputs.into_iter().count().try_into().unwrap();

    let tree_points: Vec<Vec<(u128, u128)>> = inputs.into_iter().enumerate().map(|(j, line)| {
        line.chars().enumerate().map(|(i, c)| {
            if c == '#' {
                return Some((u128::try_from(i).unwrap(),u128::try_from(j).unwrap()));
            } else {
                return None
            }
        }).filter_map(|p| p ).collect()
    }).collect();
    let flattened: Vec<(u128, u128)> = tree_points.into_iter().flatten().collect();

    let lookup: HashSet<(u128, u128)> = HashSet::from_iter(flattened);

    let mut x: u128 = 0;
    let mut trees = 0;
    for y in 0..height {
        if lookup.contains(&(x,y)) {
            trees += 1;
        }

        x = (x + 3) % width
    }

    Ok(trees.to_string())
}

pub fn day3b(inputs: &[String]) -> anyhow::Result<String> {
    let width: u128 = inputs[0].chars().count().try_into().unwrap();
    let height: u128 = inputs.into_iter().count().try_into().unwrap();

    let tree_points: Vec<Vec<(u128, u128)>> = inputs.into_iter().enumerate().map(|(j, line)| {
        line.chars().enumerate().map(|(i, c)| {
            if c == '#' {
                return Some((u128::try_from(i).unwrap(),u128::try_from(j).unwrap()));
            } else {
                return None
            }
        }).filter_map(|p| p ).collect()
    }).collect();
    let flattened: Vec<(u128, u128)> = tree_points.into_iter().flatten().collect();

    let lookup: HashSet<(u128, u128)> = HashSet::from_iter(flattened);

    // (right, down)
    let slopes = vec![
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ];

    let multiplied_trees_per_slope = slopes.into_iter().map(|(right, down)| {
        let mut x = 0;
        let mut trees: u128 = 0;
        for y in 0..height {
            if y % down != 0 {
                continue;
            }

            if lookup.contains(&(x,y)) {
                trees += 1;
            }

            x = (x + right) % width
        }

        return trees;
    }).fold(1u128, |acc, trees| acc * trees);

    Ok(multiplied_trees_per_slope.to_string())
}
