pub fn day5a(inputs: &[String]) -> anyhow::Result<String> {
    let split: Vec<(&str, &str)> = inputs.into_iter().map(|l| l.split_at(7)).collect();
    let replaced: Vec<(_, _)> = split.into_iter().map(|(row, column)| {
        let row_binary = row.replace("F", "0").replace("B", "1");
        let column_binary = column.replace("L", "0").replace("R", "1");

        (row_binary, column_binary)
    }).collect();
    let ints: Vec<(_, _)> = replaced.into_iter().map(|(row, column)| {
        let row_int = usize::from_str_radix(&row, 2).unwrap();
        let column_int = usize::from_str_radix(&column, 2).unwrap();

        (row_int, column_int)
    }).collect();
    let seat_ids: Vec<_> = ints.into_iter().map(|(row, column)| {
        (row * 8) + column
    }).collect();
    let max = seat_ids.into_iter().max().unwrap();

    Ok(max.to_string())
}

pub fn day5b(inputs: &[String]) -> anyhow::Result<String> {
    let split: Vec<(&str, &str)> = inputs.into_iter().map(|l| l.split_at(7)).collect();
    let replaced: Vec<(_, _)> = split.into_iter().map(|(row, column)| {
        let row_binary = row.replace("F", "0").replace("B", "1");
        let column_binary = column.replace("L", "0").replace("R", "1");

        (row_binary, column_binary)
    }).collect();
    let ints: Vec<(_, _)> = replaced.into_iter().map(|(row, column)| {
        let row_int = usize::from_str_radix(&row, 2).unwrap();
        let column_int = usize::from_str_radix(&column, 2).unwrap();

        (row_int, column_int)
    }).collect();
    let mut seat_ids: Vec<_> = ints.into_iter().map(|(row, column)| {
        (row * 8) + column
    }).collect();
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
