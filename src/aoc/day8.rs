use std::collections::HashSet;

pub fn day8a(inputs: &[String]) -> anyhow::Result<String> {
    let program: Vec<_> = parse_instructions(inputs);
    let mut acc: isize = 0;
    let mut current: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();

    loop {
        if seen.contains(&current) {
            return Ok(acc.to_string());
        }
        seen.insert(current);

        let instruction = &program[current];

        match instruction.instruction.as_str() {
            "nop" => {
                current += 1;
            },
            "acc" => {
                acc += instruction.data;
                current += 1;
            },
            "jmp" => {
                let next: isize = current as isize + instruction.data;
                current = next as usize;
            },
            _ => panic!("unknown instruction: {}", instruction.instruction)
        }
    }
}

fn parse_instructions(inputs: &[String]) -> Vec<Instruction> {
    inputs.iter().map(|s| {
        let parts: Vec<&str> = s.split(" ").collect();
        let (instruction, data_string) = (parts[0], parts[1]);
        let data: isize = data_string.parse().unwrap();

        Instruction {
            instruction: instruction.to_string(),
            data: data
        }
    }).collect()
}

struct Instruction {
    instruction: String,
    data: isize
}
