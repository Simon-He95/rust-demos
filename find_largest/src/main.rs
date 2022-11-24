fn main() {
    // find largest number
    let number_list = vec![5, 2, 19, 55, 1, 3];
    let largest = find_largest(&number_list);
    println!("The largest number is: {}", largest);
}

fn find_largest(number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];
    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}
