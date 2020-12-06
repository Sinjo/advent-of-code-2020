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
