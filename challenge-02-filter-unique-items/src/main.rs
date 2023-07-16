/// Returns a new vector containing only the unique values in the given vector.
/// 
/// # Arguments
/// 
/// * `values` - A vector of integer
/// 
/// # Examples
/// 
/// ```
/// let values = vec![1, 1, 2, 3, 3];
/// let unique_values = unique(values);
/// assert_eq!(unique_values, vec![1, 2, 3]);
/// ```
fn unique_basic(mut values: Vec<i32>) -> Vec<i32> {
    // Sort the vector
    values.sort();

    // Remove duplicates, this requires the vector to be sorted
    values.dedup();

    // Return the vector
    values
}

/// Returns a new vector containing only the unique values in the given vector.
/// 
/// # Arguments
/// 
/// * `values` - A vector of values that implement  and `Ord` traits
/// 
/// # Examples
/// 
/// ```
/// // Using integers
/// let values = vec![1, 1, 2, 3, 3];
/// let unique_values = unique(values);
/// assert_eq!(unique_values, vec![1, 2, 3]);
/// 
/// // Using 64 bit integers
/// let values = vec![1i64, 1i64, 2i64, 3i64, 3i64];
/// let unique_values = unique(values);
/// assert_eq!(unique_values, vec![1i64, 2i64, 3i64]);
/// ```
fn unique<T: Ord>(values: Vec<T>) -> Vec<T> {
    // Initialise an empty vector to store the unique values
    let mut unique_values = vec![];

    // Iterate over the values
    for value in values {
        // If the value is not in the vector, add it
        if !unique_values.contains(&value) {
            unique_values.push(value);
        }
    }

    // Return the vector
    unique_values
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_basic_empty_list() {
    let list: Vec<i32> = vec![];
    assert_eq!(unique_basic(list), vec![]);
}

#[test]
fn test_basic_all_unique_values() {
    let list = vec![1, 4, 5];
    assert_eq!(unique_basic(list.clone()), list);
}

#[test]
fn test_basic_duplicate_values() {
    let list = vec![1, 1, 3];
    assert_eq!(unique_basic(list), vec![1, 3]);
}

#[test]
fn test_empty_list() {
    let list: Vec<i32> = vec![];
    assert_eq!(unique(list), vec![]);
}

#[test]
fn test_all_unique_values() {
    let list = vec![1, 4, 5];
    assert_eq!(unique(list.clone()), list);
}

#[test]
fn test_duplicate_values() {
    let list = vec![1, 1, 3];
    assert_eq!(unique(list), vec![1, 3]);
}

#[test]
fn test_accepts_i64() {
    let list = vec![1i64, 1i64, 2i64];
    assert_eq!(unique(list), vec![1i64, 2i64]);
}

#[test]
fn test_retains_order() {
    let list = vec![1, 3, 3, 1, 2, 1];
    assert_eq!(unique(list), vec![1, 3, 2]);
}
