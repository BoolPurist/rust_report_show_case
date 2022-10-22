fn main() {
    let args: Vec<String> = std::env::args().collect();
    let answer: u32 = args
        .get(1)
        .expect("Answer please")
        .parse()
        .expect("Number please");
    match test_if_you_know_purpose_of_life(answer) {
        Ok(_) => println!("Me smart :)"),
        Err(msg) => println!("Testimony of my ignorance: {msg}"),
    }
}

const THE_TRUTH: u32 = 42;
const EXPLAINING_WELL_KNOW: &str = "The answer is 42, you fool !";
fn test_if_you_know_purpose_of_life(answer: u32) -> Result<(), &'static str> {
    return_if_with!(answer, THE_TRUTH, Ok(()));
    Err(EXPLAINING_WELL_KNOW)
}

#[macro_export]
macro_rules! return_if_with {
    ($to_test:ident, $cond:expr, $ret_val:expr) => {
        if ($to_test == $cond) {
            return $ret_val;
        }
    };
}
