use change_calc;
use std::collections::HashSet;

macro_rules! build_set {
    ($($k:expr),+) => {{
       let mut _tmp_set = std::collections::HashSet::new();
       $(_tmp_set.insert($k);)+
        _tmp_set
    }};
}

fn main() {
    let set: HashSet<u32> = build_set![50, 20, 10, 5, 2];
    println!("Unit as possible values in a change:\n {:?}", set);
    const AMOUNT: u32 = 238;
    println!(
        "Amount {AMOUNT} as change {:?}",
        change_calc::calc_change(AMOUNT, &set)
    );
}
