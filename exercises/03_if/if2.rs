// TODO: Fix the compiler error on this function.

// Chxpz: Sorry, using match is far more elegant than using if-else.
fn foo_if_fizz(fizzish: &str) -> &str {
    match fizzish {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "baz",
    }
}

fn main() {
    // You can optionally experiment here.
    let fizz = foo_if_fizz("fizz");
    print!("The fizz is: {}\n", fizz);
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        // This means that calling `foo_if_fizz` with the argument "fizz" should return "foo".
        assert_eq!(foo_if_fizz("fizz"), "foo");
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar");
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz");
    }
}
