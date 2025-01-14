use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ptr;

struct WeightedItem<T> {
    item: T,
    weight: f64,
}

// Two items are only equal if they are identical -- that is, they're
// the same underlying object in memory.
//
// [I suppose it's theoretically possible that there could be duplicate
// reservoir entries, if the RNG was bugged and the input has repeated
// values -- seems unlikely in practice, but this protects against it
// just in case.]
impl<T> PartialEq for WeightedItem<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl<T> Eq for WeightedItem<T> {}

// Rust doesn't implement ordering for f64 because it includes NaN
// which makes everything a mess.  In particular NaN isn't comparable
// with other floating-point numbers.
//
// We're generating all the f64 weights we'll be dealing with, so we
// know we'll never have NaN in the mix -- we can do a partial comparison
// and assert the two values are comparable when we unwrap.
impl<T> PartialOrd for WeightedItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for WeightedItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}

/// Choose a sample of `k` items from the iterator `items.
///
/// Each item has an equal chance of being picked -- that is, there's
/// a 1/N chance of choosing an item, where N is the length of the iterator.
///
/// This implements "Algorithm L" for reservoir sampling, as described
/// on the Wikipedia page:
/// https://en.wikipedia.org/wiki/Reservoir_sampling#Optimal:_Algorithm_L
///
pub fn reservoir_sample<T>(mut items: impl Iterator<Item = T>, k: usize) -> Vec<T> {
    // Taking a sample with k=0 doesn't make much sense in practice,
    // but we include this to avoid problems downstream.
    if k == 0 {
        return vec![];
    }

    // Create an empty reservoir.
    let mut reservoir: BinaryHeap<WeightedItem<T>> = BinaryHeap::with_capacity(k);

    // Fill the reservoir with the first k items.  If there are less
    // than n items, we can exit immediately.
    for _ in 1..=k {
        match items.next() {
            Some(this_item) => reservoir.push(WeightedItem {
                item: this_item,
                weight: pick_weight(),
            }),
            None => return reservoir.into_vec().into_iter().map(|r| r.item).collect(),
        };
    }

    // What's the largest weight seen so far?
    //
    // Note: we're okay to `unwrap()` here because we know that `reservoir`
    // contains at least one item.  Either `items` was non-empty, or if itwas
    // was empty, then we'd already have returned when trying to fill the
    // reservoir with the first k items.
    let mut max_weight: f64 = reservoir.peek().unwrap().weight;

    // Now go through the remaining items.
    for this_item in items {
        // Choose a weight for this item.
        let this_weight = pick_weight();

        // If this is greater than the weights seen so far, we can ignore
        // this item and move on to the next one.
        if this_weight > max_weight {
            continue;
        }

        // Otherwise, this item has a lower weight than the current item
        // with max weight -- so we'll replace that item.
        assert!(reservoir.pop().is_some());
        reservoir.push(WeightedItem {
            item: this_item,
            weight: this_weight,
        });

        // Recalculate the max weight for the new sample.
        max_weight = reservoir.peek().unwrap().weight;
    }

    let sample: Vec<T> = reservoir.into_vec().into_iter().map(|r| r.item).collect();
    assert!(sample.len() == k);
    sample
}

/// Create a random weight u_i ~ U[0,1]
fn pick_weight() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

#[cfg(test)]
mod reservoir_sample_tests {
    use super::*;
    use std::collections::HashMap;

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

    // If k=0, then it returns an empty sample.
    #[test]
    fn it_returns_an_empty_sample_if_k_zero() {
        let items = vec!["a", "b", "c"];
        let sample = reservoir_sample(items.into_iter(), 0);

        assert_eq!(sample.len(), 0);
    }

    // It chooses items with a uniform distribution -- every item has
    // an equal chance of being picked.
    //
    // We take a large number of samples of the integers 0..n, and check
    // that each integer is picked about as many times as we expect.
    #[test]
    fn test_distribution() {
        let k = 20;
        let n = 100;
        let iterations = 10000;

        // How often was each integer picked?
        let mut counts: HashMap<i32, usize> = HashMap::new();

        // Run many iterations, create a sample, and record how many
        // times each integer was picked.
        for _ in 0..iterations {
            let items = 0..n;
            let sample = reservoir_sample(items, k);

            for s in sample.into_iter() {
                *counts.entry(s).or_insert(0) += 1;
            }
        }

        // Now check that each number appears roughly as many times
        // as we'd expect (within reasonable bounds).
        let total_samples = iterations * k;
        let expected = total_samples as f64 / n as f64;

        for item in 0..n {
            let item_count = *counts.get(&item).unwrap_or(&0);

            let ratio = (item_count as f64) / expected;
            assert!(
                ratio > 0.8 && ratio < 1.2,
                "Distribution appears skewed: count={}, expected={}",
                item_count,
                expected
            );
        }
    }

    /// Returns true if two vectors contain the same items (but potentially
    /// in a different order), false otherwise.
    ///
    ///     equivalent_items(vec![1, 3, 2], vec![3, 2, 1])
    ///     => true
    ///
    ///     equivalent_items(vec![4, 5, 6], vec![3, 2, 1])
    ///     => false
    ///
    fn equivalent_items<T: std::cmp::PartialEq + std::cmp::Ord>(
        mut vec1: Vec<T>,
        mut vec2: Vec<T>,
    ) -> bool {
        vec1.sort();
        vec2.sort();

        vec1 == vec2
    }
}
