use std::collections::HashSet;

use anyhow::anyhow;

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

// Approach:
//   - Generate program variants (each changes a single nop -> jmp, or jmp -> nop)
//   - Run each until either:
//     - Instruction pointer == length of program (we're 0-indexed here, so that's off the end of
//       the Vec) => Success: this is the fixed variant
//     - We attempt to run the same instruction again => Failure: this case loops forever
//
pub fn day8b(inputs: &[String]) -> anyhow::Result<String> {
    let program: Vec<_> = parse_instructions(inputs);
    let variant_positions: Vec<_> = program.iter().enumerate().filter_map(|(i, instruction)| {
        if instruction.instruction == "nop" || instruction.instruction == "jmp" {
            return Some(i);
        }

        None
    }).collect();
    let variants: Vec<_> = variant_positions.iter().map(|position| {
        let mut variant = program.clone();
        let instruction = &variant[*position];
        if instruction.instruction == "nop" {
            variant[*position] = Instruction{instruction: "jmp".to_string(), data: instruction.data}
        } else {
            variant[*position] = Instruction{instruction: "nop".to_string(), data: instruction.data}
        }

        variant
    }).collect();

    let acc_after_success = variants.iter().find_map(|variant| {
        match execute_program(variant.clone()) {
            Ok(acc) => Some(acc),
            Err(_) => None
        }
    }).unwrap();

    Ok(acc_after_success.to_string())
}

fn execute_program(program: Vec<Instruction>) -> anyhow::Result<isize> {
    let mut acc: isize = 0;
    let mut current: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();

    loop {
        if seen.contains(&current) {
            return Err(anyhow!("program in infinite loop"));
        }
        if current == program.len() {
            return Ok(acc);
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

#[derive(Clone, Debug)]
struct Instruction {
    instruction: String,
    data: isize
}
