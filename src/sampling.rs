use rand::Rng;
use std::collections::HashMap;

fn random_weight() -> i32 {
    rand::thread_rng().gen_range(i32::MIN..i32::MAX)
}

pub fn reservoir_sample<T: std::fmt::Debug>(
    mut items: impl Iterator<Item = T>,
    n: usize,
) -> Vec<T> {
    // Create an empty reservoir.
    //
    // This is a map (weight) -> (item).
    let mut reservoir: HashMap<i32, T> = HashMap::with_capacity(n);

    // Fill the reservoir with the first n items.  If there are less
    // than n items, we can exit immediately.
    for _ in 1..=n {
        match items.next() {
            Some(this_item) => reservoir.insert(random_weight(), this_item),
            None => return reservoir.into_values().collect(),
        };
    }

    // What's the largest weight seen so far?
    let mut max_weight: i32 = *reservoir.keys().max().unwrap();

    // Now go through the remaining items.
    for this_item in items {
        // Choose a weight for this item.
        let this_weight = random_weight();

        // If this is greater than the weights seen so far, we can ignore
        // this item and move on to the next one.
        if this_weight > max_weight {
            continue;
        }

        // Replace the item that had the max weight with the new item,
        // then recalculate the max weight.
        assert!(reservoir.remove(&max_weight).is_some());
        reservoir.insert(this_weight, this_item);
        max_weight = *reservoir.keys().max().unwrap();
    }

    reservoir.into_values().collect()
}

#[cfg(test)]
mod reservoir_sample_tests {
    use crate::sampling::reservoir_sample;

    // If there are no items, then the sample is empty.
    #[test]
    fn it_returns_an_empty_sample_for_an_empty_input() {
        let items: Vec<usize> = vec![];
        let sample = reservoir_sample(items.into_iter(), 5);

        assert_eq!(sample.len(), 0);
    }

    // If there are less items than the sample size, then the sample is
    // the complete set.
    #[test]
    fn it_returns_complete_sample_if_less_items_than_sample_size() {
        let items = vec!["a", "b", "c"];
        let sample = reservoir_sample(items.into_iter(), 5);

        assert!(equivalent_items(sample, vec!["a", "b", "c"]));
    }

    // If there's an equal number of items to the sample size, then the
    // sample is the complete set.
    #[test]
    fn it_returns_complete_sample_if_item_count_equal_to_sample_size() {
        let items = vec!["a", "b", "c"];
        let sample = reservoir_sample(items.into_iter(), 3);

        assert!(equivalent_items(sample, vec!["a", "b", "c"]));
    }

    fn equivalent_items<T: std::cmp::PartialEq + std::cmp::Ord>(
        mut vec1: Vec<T>,
        mut vec2: Vec<T>,
    ) -> bool {
        vec1.sort();
        vec2.sort();

        vec1 == vec2
    }
}
