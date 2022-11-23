use std::fmt::Display;
trait HasDataWithLength {
    type Data: Display;
    fn get_data_with_length(&self) -> (usize, Self::Data);
}

// String and str are types from the std library.
// We can impl our own traits on them however.
// This is nice for mocking for example.
impl HasDataWithLength for String {
    type Data = String;
    fn get_data_with_length(&self) -> (usize, Self::Data) {
        return (self.len(), self.clone());
    }
}

// We can also decide to implent for an reference
// to some data without changing or creating a new trait.
impl<'a> HasDataWithLength for &'a str {
    type Data = &'a str;
    fn get_data_with_length(&self) -> (usize, Self::Data) {
        return (self.len(), self);
    }
}

fn main() {
    let hello_world = "Hello, world!";
    // hello_world is of type &'static str, aka a string literal stored in the binrary.
    print_count_with_data(&hello_world.to_string());
    println!("Just printing from a reference of the string slice");
    print_count_with_data(&hello_world);
}

fn print_count_with_data<T>(to_print: &T)
where
    T: HasDataWithLength,
{
    let data = to_print.get_data_with_length();
    println!("Count: {}", data.0);
    println!("Data: {}", data.1);
}
