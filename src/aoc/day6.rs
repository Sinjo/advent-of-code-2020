use std::collections::HashSet;

pub fn day6a(inputs: &[String]) -> anyhow::Result<String> {
    let records: Vec<String> = inputs.split(|s| s.is_empty()).
        map(|slice| slice.into_iter().cloned().collect::<Vec<_>>().join(" ")).collect();

    let group_answers: Vec<_> = records.into_iter().map(|group| {
        let answer_chars: Vec<_> = group.replace(" ", "").chars().collect();
        let mut answers = HashSet::new();
        for c in answer_chars {
            answers.insert(c);
        }

        answers.len()
    }).collect();

    let sum: usize = group_answers.into_iter().sum();

    Ok(sum.to_string())
}

pub fn day6b(inputs: &[String]) -> anyhow::Result<String> {
    let records: Vec<Vec<String>> = inputs.split(|s| s.is_empty()).
        map(|slice| slice.into_iter().cloned().collect::<Vec<_>>()).collect();

    let group_hashsets: Vec<Vec<HashSet<char>>> = records.into_iter().map(|group| {
        group.into_iter().map(|row| {
            row.chars().collect()
        }).collect()
    }).collect();

    let group_answers: Vec<HashSet<char>> = group_hashsets.into_iter().map(|g| {
        g.iter().fold(g[0].clone(), |acc, row| {
            acc.intersection(&row).cloned().collect()
        })
    }).collect();

    let group_counts: Vec<_> = group_answers.into_iter().map(|g| g.len()).collect();
    let sum: usize = group_counts.into_iter().sum();

    println!("{:#?}", sum);

    Ok(sum.to_string())
}
