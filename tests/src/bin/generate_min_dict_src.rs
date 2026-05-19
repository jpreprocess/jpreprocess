use std::path::PathBuf;

fn main() {
    let naist_jdic_path = PathBuf::from("tests/data/naist-jdic/");
    let dict_csv_path = PathBuf::from("tests/data/min-dict/min-dict.csv");
    let output_dir = PathBuf::from("tests/data/min-dict-src/");

    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    let dict_csv_str = std::fs::read_to_string(dict_csv_path).expect("Failed to read min-dict.csv");
    let unk_csv_str =
        std::fs::read_to_string(naist_jdic_path.join("unk.def")).expect("Failed to read unk.def");

    let mut id_mapping = std::collections::HashMap::new();
    let mut new_id = 0;
    let line_iter = dict_csv_str.lines().chain(unk_csv_str.lines());
    for line in line_iter {
        let mut parts = line.split(',').skip(1);
        let left_id = parts.next().unwrap().parse::<i32>().unwrap();
        let right_id = parts.next().unwrap().parse::<i32>().unwrap();

        if !id_mapping.contains_key(&left_id) {
            id_mapping.insert(left_id, new_id);
            new_id += 1;
        }
        if !id_mapping.contains_key(&right_id) {
            id_mapping.insert(right_id, new_id);
            new_id += 1;
        }
    }

    let matrix_str = std::fs::read_to_string(naist_jdic_path.join("matrix.def"))
        .expect("Failed to read matrix.def");

    let subset_matrix = matrix_str
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut parts = line.split(' ');
            let left_id = parts.next().unwrap().parse::<i32>().unwrap();
            let right_id = parts.next().unwrap().parse::<i32>().unwrap();
            let cost = parts.next().unwrap().parse::<i32>().unwrap();

            if let (Some(&new_left_id), Some(&new_right_id)) =
                (id_mapping.get(&left_id), id_mapping.get(&right_id))
            {
                Some(format!("{} {} {}", new_left_id, new_right_id, cost))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let subset_matrix_str = format!(
        "{} {}\n{}",
        id_mapping.len(),
        id_mapping.len(),
        subset_matrix
    );
    std::fs::write(output_dir.join("matrix.def"), subset_matrix_str).unwrap();

    let subset_dict = dict_csv_str
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let string = parts.next().unwrap();
            let left_id = parts.next().unwrap().parse::<i32>().unwrap();
            let right_id = parts.next().unwrap().parse::<i32>().unwrap();
            let rest = parts.collect::<Vec<_>>().join(",");

            let new_left_id = id_mapping.get(&left_id).unwrap();
            let new_right_id = id_mapping.get(&right_id).unwrap();
            format!("{},{},{},{}", string, new_left_id, new_right_id, rest)
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(output_dir.join("min-dict-subset.csv"), subset_dict).unwrap();

    let subset_unk = unk_csv_str
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let string = parts.next().unwrap();
            let left_id = parts.next().unwrap().parse::<i32>().unwrap();
            let right_id = parts.next().unwrap().parse::<i32>().unwrap();
            let rest = parts.collect::<Vec<_>>().join(",");

            let new_left_id = id_mapping.get(&left_id).unwrap();
            let new_right_id = id_mapping.get(&right_id).unwrap();
            format!("{},{},{},{}", string, new_left_id, new_right_id, rest)
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(output_dir.join("unk.def"), subset_unk).unwrap();

    std::fs::copy(
        naist_jdic_path.join("char.def"),
        output_dir.join("char.def"),
    )
    .unwrap();
    std::fs::copy(
        naist_jdic_path.join("feature.def"),
        output_dir.join("feature.def"),
    )
    .unwrap();
}
