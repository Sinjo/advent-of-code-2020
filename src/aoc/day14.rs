// For part 1:
//   - To set bits to one: construct a bitmask with the relevant bits
//     set to 1 (everything else 0), then OR with input
//   - To set bits to zero: construct a bitmask with the relevant bits
//     set to 0 (everything else 1), then AND with input
//   - X always needs to be the no-op bit
use regex::Regex;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Mask(Mask),
    Write(Write)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mask {
    and_mask: u64,
    or_mask: u64
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Write {
    address: u64,
    value: u64
}

pub fn day14a(inputs: &[String]) -> anyhow::Result<String> {
    let program = parse_input(inputs);

    // start with the identity mask
    let mut current_mask = Mask {
        and_mask: u64::MAX,
        or_mask: 0
    };
    let mut memory_state = BTreeMap::new();

    for instruction in program.iter() {
        match instruction {
            Instruction::Mask(m) => {
                current_mask = *m;
            },
            Instruction::Write(w) => {
                let masked = w.value & current_mask.and_mask | current_mask.or_mask;
                memory_state.insert(w.address, masked);
            }
        }
    }

    let answer: u64 = memory_state.values().sum();

    Ok(answer.to_string())
}

fn parse_input(inputs: &[String]) -> Vec<Instruction> {
    let mask_pattern = Regex::new(r"^mask = (?P<mask>[X01]+)$").unwrap();
    let write_pattern = Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();

    inputs.iter().map(|i| {
        if let Some(captures) = mask_pattern.captures(i) {
            let mask_str = captures.name("mask").unwrap().as_str();

            let and_mask_str = mask_str.replace("X", "1");
            let and_mask: u64 = u64::from_str_radix(&and_mask_str, 2).unwrap();

            let or_mask_str = mask_str.replace("X", "0");
            let or_mask: u64 = u64::from_str_radix(&or_mask_str, 2).unwrap();

            return Instruction::Mask(
                Mask {
                    and_mask: and_mask,
                    or_mask: or_mask
                }
            )
        } else if let Some(captures) = write_pattern.captures(i) {
            let address_str = captures.name("address").unwrap().as_str();
            let value_str = captures.name("value").unwrap().as_str();

            let address: u64 = address_str.parse().unwrap();
            let value: u64 = value_str.parse().unwrap();

            return Instruction::Write(
                Write {
                    address: address,
                    value: value
                }
            )
        } else {
            panic!("unrecognised instruction: {}", i);
        }
    }).collect()
}
