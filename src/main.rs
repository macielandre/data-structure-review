use std::mem::drop;

fn main() {
    let array: Vec<usize> = vec![63, 15, 48, 9, 32, 89, 71, 52, 6, 78, 44, 91, 19, 87, 54, 38, 26, 59, 3, 83, 41, 97, 70, 23, 16, 66, 35, 0, 8, 28, 68, 21, 75, 92, 37, 12, 88, 14, 61, 79, 30, 85, 95, 5, 73, 50, 46, 1, 76, 45, 27, 55, 20, 90, 34, 13, 58, 72, 11, 84, 25, 17, 93, 67, 2, 42, 29, 62, 86, 56, 22, 39, 69, 98, 10, 74, 53, 31, 49, 4, 81, 64, 51, 94, 7, 60, 18, 80, 99, 40, 82, 57, 65, 43, 77, 24, 47, 33, 36, 96];

    let sorted_array= bucket_sort(&array);

    print!("{:?}", sorted_array);
}

fn bucket_sort(array: &Vec<usize>) -> Vec<usize> {
    const INIT_ARRAY: Vec<usize> = vec![];

    let mut buckets: [Vec<usize>; 10] = [INIT_ARRAY; 10];

    for number in array {
        let number_with_pad: String = format!("{:02}", number);
        let bucket_index: usize  = number_with_pad.as_bytes()[0] as usize - 0x30;
        
        buckets[bucket_index].push(*number);
    }

    let mut response_array: Vec<usize> = vec![];

    for bucket in &mut buckets {
        bucket.sort();

        response_array.append(bucket);
    }
    
    drop(buckets);

    return response_array;
}
