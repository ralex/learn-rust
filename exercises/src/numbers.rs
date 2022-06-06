fn main() {
    // Given a list of integers, use a vector and return the median (when sorted,
    // the value in the middle position) and mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.
    
    use std::collections::HashMap;
    
    let mut numbers_list = vec![1, 6, 8, 0, 3, 7, 5, 6, 4, 9, 4, 7, 4, 9, 1, 2, 8, 6, 3, 6, 4, 7, 1, 2, 2, 9, 0];
    println!("Raw numbers_list: {:?}", numbers_list);
    numbers_list.sort();
    println!("Sorted numbers_list: {:?}", numbers_list);

    let mut map = HashMap::new();

    for number in &numbers_list {
        let count = map.entry(number).or_insert(0);
        // println!("word: {}, count: {}", number, count);
        *count += 1;
    }
    println!("HashMap map : {:?}", map);

    println!("Median: {:?}", numbers_list.get(numbers_list.len()/2).unwrap());

    println!("Mode: {:?}", map.iter()
                              .max_by_key(|entry | entry.1)
                              .unwrap()
                              .0
    );
}
