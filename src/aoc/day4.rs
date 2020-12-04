use regex::Regex;
use std::collections::HashMap;

pub fn day4a(inputs: &[String]) -> anyhow::Result<String> {
    let pattern = Regex::new(r"(\w+):((\w|#)+)").unwrap();

    let records: Vec<String> = inputs.split(|s| s.is_empty()).
        map(|slice| slice.into_iter().cloned().collect::<Vec<_>>().join(" ")).collect();

    let mut parsed: Vec<HashMap<String,String>> = records.into_iter().map(|r| {
        let captures: Vec<_> = pattern.find_iter(&r).map(|pair| {
            let split: Vec<_> = pair.as_str().split(":").collect();
            let k = split[0].to_string();
            let v = split[1].to_string();

            (k, v)
        }).collect();

        let mut capture_hash: HashMap<String, String> = HashMap::new();
        for (k,v) in captures.iter() {
            capture_hash.insert(k.to_string(), v.to_string());
        }
        capture_hash
    }).collect();

    for p in parsed.iter_mut() {
        p.remove("cid");
    }

    Ok(parsed.into_iter().filter(|r| validate_record(r)).count().to_string())
}

pub fn validate_record(record: &HashMap<String,String>) -> bool {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    keys.into_iter().map(|k| record.contains_key(k)).fold(true, |acc, key_present| acc & key_present)
}
