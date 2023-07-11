/// Calculate the median of a vector of floating-point numbers.
///
/// # Arguments
///
/// * `numbers` - A vector of floating-point numbers
///
/// # Examples
///
/// ```
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = calculate_median(numbers);
/// assert_eq!(result, Some(3.0));
/// ```
fn calculate_median(numbers: Vec<f32>) -> Option<f32> {
    let length = numbers.len();

    // If the vector is empty, return None
    // We could've used the `is_empty` method, but we need the length later anyway
    if length == 0 {
        return None;
    }

    // Create a clone of the vector so the original is not affected
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let middle = length / 2;

    // If the vector has an even number of elements, return the average of the two middle elements
    if length % 2 == 0 {
        return Some((sorted_numbers[middle - 1] + sorted_numbers[middle]) / 2.0);
    }

    // If the vector has an odd number of elements, return the middle element
    Some(sorted_numbers[middle])
}

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = calculate_median(numbers);
    println!("Result: {:?}", result);
}

#[test]
fn test_calculate_median_with_empty_vector() {
    let numbers = vec![];
    let result = calculate_median(numbers);
    assert_eq!(result, None);
}

#[test]
fn test_calculate_median_with_one_element() {
    let numbers = vec![1.0];
    let result = calculate_median(numbers);
    assert_eq!(result, Some(1.0));
}

#[test]
fn test_calculate_median_with_two_elements() {
    let numbers = vec![1.0, 2.0];
    let result = calculate_median(numbers);
    assert_eq!(result, Some(1.5));
}

#[test]
fn test_calculate_median_with_three_elements() {
    let numbers = vec![1.0, 2.0, 3.0];
    let result = calculate_median(numbers);
    assert_eq!(result, Some(2.0));
}

#[test]
fn test_calculate_median_with_three_unsorted_elements() {
    let numbers = vec![3.0, 1.0, 2.0];
    let result = calculate_median(numbers);
    assert_eq!(result, Some(2.0));
}
