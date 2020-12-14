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
    or_mask: u64,
    float_mask: u64
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
        or_mask: 0,
        float_mask: 0
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

// No longer need and_mask
// Still use or_mask, but on address (to set relevant 1s)
// Need to add float_mask: 1 if floating, 0 otherwise
pub fn day14b(inputs: &[String]) -> anyhow::Result<String> {
    let program = parse_input(inputs);

    // start with the identity mask
    let mut current_mask = Mask {
        and_mask: u64::MAX,
        or_mask: 0,
        float_mask: 0
    };
    let mut memory_state = BTreeMap::new();

    for instruction in program.iter() {
        match instruction {
            Instruction::Mask(m) => {
                current_mask = *m;
            },
            Instruction::Write(w) => {
                // Apply or_mask first
                // Put into single element list
                //
                // For each index in float_mask (actually while index != 0)
                //   - if 1 => return both: original number, number with that bit xor 1
                let or_masked = w.address | current_mask.or_mask;

                let mut index: u64 = 1;
                let mut addresses = vec![or_masked];

                while index != 0 {
                    if (index & current_mask.float_mask) != 0 {
                        let mut new_addresses: Vec<_> = addresses.iter().map(|a| a ^ index).collect();
                        addresses.append(&mut new_addresses);
                    }

                    index <<= 1;
                }

                for address in addresses.iter() {
                    memory_state.insert(address.clone(), w.value);
                }
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

            let float_mask_str = mask_str.replace("1", "0").replace("X", "1");
            let float_mask: u64 = u64::from_str_radix(&float_mask_str, 2).unwrap();

            return Instruction::Mask(
                Mask {
                    and_mask: and_mask,
                    or_mask: or_mask,
                    float_mask: float_mask
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
