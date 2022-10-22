use std::collections::HashSet;
// Derive generates a implementation for the trait debug during compilation. No inheritance. Debug allows printing an entity with all its field.
#[derive(Debug)]
pub struct ChangeWithLeft(pub String, pub u32);
/// This example codex down here is compiled and the assert_eq! macro is validated during
/// testing via cargo test.
/// ```
/// use std::collections::HashSet;
/// use change_calc::ChangeWithLeft;
///
/// let amount = 98;
/// let mut coins = HashSet::<u32>::new();
///
/// coins.insert(50);
/// coins.insert(10);
///
/// let ChangeWithLeft(change, left) = change_calc::calc_change(amount, &coins);
///
/// assert_eq!(change, "50 10 10 10 10");
/// assert_eq!(left, 8);
/// ```
pub fn calc_change(amount: u32, coin_units: &HashSet<u32>) -> ChangeWithLeft {
    let mut change_as_text = String::new();
    let mut left_amount = amount;
    let mut unique_coins: Vec<_> = coin_units.iter().collect();

    unique_coins.sort();
    unique_coins.reverse();

    for coin in unique_coins {
        loop {
            match try_sub_coin(left_amount, coin.clone()) {
                Some(new_amount) => {
                    left_amount = new_amount;
                    change_as_text.push_str(&format!(" {coin}"));
                }
                None => break,
            };
        }
    }

    return ChangeWithLeft(change_as_text.trim().to_string(), left_amount);

    fn try_sub_coin(amount: u32, coin: u32) -> Option<u32> {
        if amount >= coin {
            Some(amount - coin)
        } else {
            None
        }
    }
}
// Only gets compiled during ant tests, (cargo test)
#[cfg(test)]
mod testing {
    // Gives access to outer function calc_change
    use super::*;

    // This function is marked to be called as unit test
    #[test]
    fn should_return_correct_change() {
        assert_change(200, &[50, 25, 1], "50 50 50 50", 0);
        assert_change(122, &[50, 25, 10, 5, 2, 1], "50 50 10 10 2", 0);
        assert_change(83, &[50, 25, 5, 2, 1], "50 25 5 2 1", 0);
        assert_change(90, &[50, 25], "50 25", 15);
    }

    fn assert_change(amount: u32, coins: &[u32], expected_change: &str, expected_left: u32) {
        let ChangeWithLeft(actual_change, actual_left) = calc_change(amount, &build_set(coins));
        assert_eq!(actual_change, expected_change);
        assert_eq!(actual_left, expected_left);
    }

    fn build_set(numbers: &[u32]) -> HashSet<u32> {
        let mut set = HashSet::<u32>::new();

        numbers
            .iter()
            .for_each(|to_insert| (_ = set.insert(*to_insert)));

        set
    }
}
