use hands_on_2::{IsThereSegmentTree, MinMaxSegmentTree};
use std::fs::File;
use std::io::{self, BufRead, Result};
use std::path::Path;

fn main() -> io::Result<()> {
    let _ = exercise_1_tests();
    let _ = exercise_2_tests();
    Ok(())
}

/// Utilities to open input and ouput file
fn open_test_files(
    input_path: &str,
    output_path: &str,
) -> Result<(io::BufReader<File>, io::BufReader<File>)> {
    let input_file = File::open(Path::new(input_path))?;
    let output_file = File::open(Path::new(output_path))?;
    Ok((
        io::BufReader::new(input_file),
        io::BufReader::new(output_file),
    ))
}

/// Utilities to read lines and parse them as a vector of values
fn read_next_line_as_numbers<T: std::str::FromStr>(
    reader: &mut io::BufReader<File>,
) -> Result<Vec<T>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line
        .split_whitespace()
        .filter_map(|x| x.parse::<T>().ok())
        .collect())
}

/// Function to perform tests on the MinMaxSegmentTree implementation
fn exercise_1_tests() -> Result<()> {
    let mut total_correct = 0;
    let mut total_max_queries = 0;

    println!("---- Min and Max ----");

    for test_num in 0..10 {
        // open input and output file
        let (mut input_reader, mut output_reader) = open_test_files(
            &format!("../Testset_handson2_p1/input{}.txt", test_num),
            &format!("../Testset_handson2_p1/output{}.txt", test_num),
        )?;

        let header = read_next_line_as_numbers::<usize>(&mut input_reader)?;
        let vector_length = header[0];
        let num_queries = header[1];

        // read the values vector
        let values = read_next_line_as_numbers::<u32>(&mut input_reader)?;
        if values.len() != vector_length {
            panic!("La lunghezza del vettore non corrisponde alla lunghezza dichiarata.");
        }

        // build the segment tree
        let mut seg_tree = MinMaxSegmentTree::new(&values);
        let mut correct_count = 0;
        let mut max_query_count = 0;

        for _ in 0..num_queries {
            let query = read_next_line_as_numbers::<usize>(&mut input_reader)?;
            let op = query[0];
            let i = query[1] - 1;
            let j = query[2] - 1;

            if op == 0 {
                // update the segment tree with the specified value
                let t = query[3] as u32;
                seg_tree.update(i, j, t);
            } else {
                // get the maximum value in the specified range
                max_query_count += 1;
                let expected = read_next_line_as_numbers::<u32>(&mut output_reader)?[0];
                let result = seg_tree.max(i, j);

                if result == expected {
                    correct_count += 1;
                }
            }
        }

        println!(
            "Test {}: Corrette {}/{}",
            test_num, correct_count, max_query_count
        );

        total_correct += correct_count;
        total_max_queries += max_query_count;
    }

    println!("Totale corrette: {}/{}\n", total_correct, total_max_queries);
    Ok(())
}

/// Function to perform tests on the IsThereSegmentTree implementation
fn exercise_2_tests() -> Result<()> {
    let mut total_correct = 0;
    let mut total_max_queries = 0;

    println!("---- Is There ----");

    for test_num in 0..7 {
        // open input and output file
        let (mut input_reader, mut output_reader) = open_test_files(
            &format!("../Testset_handson2_p2/input{}.txt", test_num),
            &format!("../Testset_handson2_p2/output{}.txt", test_num),
        )?;

        let header = read_next_line_as_numbers::<usize>(&mut input_reader)?;
        let num_segments = header[0];
        let num_queries = header[1];

        // read the segments and add its starting and ending point in a vector
        let mut segments = Vec::new();
        for _ in 0..num_segments {
            let segment = read_next_line_as_numbers::<u32>(&mut input_reader)?;
            segments.push((segment[0], segment[1]));
        }

        // build the segment tree on this new vector
        let seg_tree = IsThereSegmentTree::new(&segments, num_segments);
        let mut correct_count = 0;

        // perform the query
        for _ in 0..num_queries {
            let query = read_next_line_as_numbers::<usize>(&mut input_reader)?;
            let i = query[0];
            let j = query[1];
            let k = query[2] as u32;

            let expected = read_next_line_as_numbers::<usize>(&mut output_reader)?[0] == 1;
            let result = seg_tree.is_there(i, j, k);

            if result == expected {
                correct_count += 1;
            }
        }

        println!(
            "Test {}: Corrette {}/{}",
            test_num, correct_count, num_queries
        );

        total_correct += correct_count;
        total_max_queries += num_queries;
    }

    println!("Totale corrette: {}/{}\n", total_correct, total_max_queries);
    Ok(())
}
