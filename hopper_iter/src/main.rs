/// In Rust custom iterator are sturcts which take another iterator.
/// Custom iterator implement the trait Iterator
struct Hopper<I> {
    duration_outside_interval: u32,
    current_step: u32,
    duration_inside_interval: u32,
    iter: I,
}

impl<I> Hopper<I> {
    pub fn new(
        iter: I,
        duration_outside_interval: u32,
        duration_inside_interval: u32,
        start: bool,
    ) -> Self {
        if duration_outside_interval == 0 || duration_inside_interval == 0 {
            panic!(
                "{} and {} must not be zero !",
                stringify!(duration_outside_interval),
                stringify!(duration_inside_interval)
            );
        }

        Hopper {
            iter,
            duration_outside_interval,
            duration_inside_interval,
            current_step: if start {
                0
            } else {
                duration_inside_interval + 1
            },
        }
    }
}

impl<I> Iterator for Hopper<I>
where
    I: Iterator,
{
    type Item = I::Item;

    /// Logic to return an element from an iterator.
    fn next(&mut self) -> Option<Self::Item> {
        self.current_step += 1;
        if self.current_step > self.duration_inside_interval {
            self.current_step = 0;

            while let Some(next_value) = self.iter.next() {
                self.current_step += 1;
                if self.current_step > self.duration_outside_interval {
                    self.current_step = 1;
                    return Some(next_value);
                }
            }

            return None;
        }

        self.iter.next()
    }
}

/// Supertrait of trait Iterator with default implementations only.
/// These default implementations as function will be used on iterator.
trait HopperExt: Iterator {
    fn hopp(self, outside_interval_duration: u32, inside_interval_duration: u32) -> Hopper<Self>
    where
        Self: Sized,
    {
        Hopper::new(
            self,
            inside_interval_duration,
            outside_interval_duration,
            true,
        )
    }

    fn hopp_past_start(
        self,
        inside_interval_duration: u32,
        outside_interval_duration: u32,
    ) -> Hopper<Self>
    where
        Self: Sized,
    {
        Hopper::new(
            self,
            outside_interval_duration,
            inside_interval_duration,
            false,
        )
    }
}

/// By implementing HopperExt for Iterator, structs which implement the iterator trait get access
/// to the default implementation of the trait HopperExt.
/// This is comparable to the concept of extension methods in c#.
/// I: Iterator is generic. This means all structs which implement trait Iterator,  implement
/// trait HopperExt now.
impl<I: Iterator> HopperExt for I {}

/// Usage: (1..20).into_iter().hopp(2, 3)
/// Param1: 2 print up to 2 numbers.
/// Param1: 3 After 2 numbers skip the next 3 numbers.
/// output: [1, 2, 6, 7, 11, 12, 16, 17]
#[allow(dead_code)]
fn usage() {
    for x in (1..20).into_iter().hopp(2, 3) {
        println!("{:?}", x);
    }
}

fn main() {
    // (1..40) is a range.
    //
    // into_iter() creates iterator which takes owner ship of range and iterates over every
    // element as owned value.
    //
    // filter creates a struct which is a another iterator. This filter iterator has the base
    // iterator as inner field to operate on.
    //
    // hopp comes from the Supertrait HopperExt which returns a Hopper struct as another iterator.
    // This iterator now operates on the filter iterator in an inner field.
    //
    // x is an element returned by the Hopper iterator.
    for x in (1..40)
        .into_iter()
        .filter(|x| *x % 2 == 1)
        .hopp_past_start(4, 7)
    {
        println!("{:?}", x);
    }
}
