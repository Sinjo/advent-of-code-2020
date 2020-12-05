pub fn day5a(inputs: &[String]) -> anyhow::Result<String> {
    let seat_ids: Vec<_> = inputs.into_iter().map(seat_to_int).collect();
    let max = seat_ids.into_iter().max().unwrap();

    Ok(max.to_string())
}

pub fn day5b(inputs: &[String]) -> anyhow::Result<String> {
    let mut seat_ids: Vec<_> = inputs.into_iter().map(seat_to_int).collect();
    seat_ids.sort();

    let my_seat = seat_ids.windows(2).find_map(|seats| {
        let difference = seats[1] - seats[0];
        if difference == 2 {
            return Some(seats[0] + 1);
        } else {
            return None
        }
    }).unwrap();

    Ok(my_seat.to_string())
}

fn seat_to_int(seat: &String) -> usize {
    let binary = seat.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");

    usize::from_str_radix(&binary, 2).unwrap()
}
