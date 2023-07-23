/// This function prints any type which implements the `Display` trait.
/// 
/// # Examples
/// 
/// ```
/// let a: &str = "?";
/// info(&a);
/// 
/// let b: String = "?".to_string();
/// info(&b);
/// ```
fn info<T: std::fmt::Display>(a: &T) {
    println!("{}", a);
}

/// This function is equivalent to the previous one.
/// The difference is that this function uses the `ToString` trait. This
/// means that any type which has the to_string method can be used.
/// 
/// # Examples
/// 
/// ```
/// let a: &str = "?";
/// info_with_to_string(&a);
/// 
/// let b: String = "?".to_string();
/// info_with_to_string(&b);
/// ```
fn info_with_to_string<T: ToString>(a: &T) {
    // Note: This allocates memory, as a string is created to be printed.
    println!("{}", a.to_string());
}


/// This function is equivalent to the previous one.
/// The difference is that this function uses the `AsRef` trait. This
/// means that any type which has the as_ref method can be used.
/// 
/// # Examples
/// 
/// ```
/// let a: &str = "?";
/// info_with_as_ref(&a);
/// 
/// let b: String = "?".to_string();
/// info_with_as_ref(&b);
/// ```
fn info_with_as_ref<T: AsRef<str>>(a: &T)
{
    // Note: This doesn't cause an allocation
    println!("{}", a.as_ref());
}

fn main() {
    // Note: These examples haven't been put together in a testable way.
    //       This is just to show how the functions can be used.
    let a: &str = "?";
    let b: String = "?".to_string();

    info(&a);
    info(&b);

    info_with_to_string(&a);
    info_with_to_string(&b);

    info_with_as_ref(&a);
    info_with_as_ref(&b);
}
