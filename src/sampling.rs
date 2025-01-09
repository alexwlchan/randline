pub fn reservoir_sample<T: std::fmt::Debug>(
  mut items: impl Iterator<Item = T>,
  n: usize
) -> Vec<T> {

  // Create an empty reservoir.
  let mut reservoir: Vec<T> = Vec::with_capacity(n);

  // Fill the reservoir with the first n items.  If there are less
  // than n items, we can exit immediately.
  for _ in 1..=n {
    match items.next() {
      Some(this_item) => reservoir.push(this_item),
      None => return reservoir,
    };
  }

  vec![]
}

#[cfg(test)]
mod reservoir_sample_tests {
  use crate::sampling::reservoir_sample;

  // If there are no items, then the sample is empty.
  #[test]
  fn it_returns_an_empty_sample_for_an_empty_input() {
    let items: Vec<usize> = vec![];
    let sample = reservoir_sample(items.iter(), 5);

    assert_eq!(sample.len(), 0);
  }

  // If there are less items than the sample size, then the sample is
  // the complete set.
  #[test]
  fn it_returns_complete_sample_if_less_items_than_sample_size() {
    let items = vec!["a", "b", "c"];
    let sample = reservoir_sample(items.iter(), 5);

    assert_eq!(sample.len(), 3);
    assert_eq!(*sample[0], "a");
    assert_eq!(*sample[1], "b");
    assert_eq!(*sample[2], "c");
  }

  // If there's an equal number of items to the sample size, then the
  // sample is the complete set.
  #[test]
  fn it_returns_complete_sample_if_item_count_equal_to_sample_size() {
    let items = vec!["a", "b", "c"];
    let sample = reservoir_sample(items.iter(), 3);

    println!("AWLC = {:?}", sample);

    assert_eq!(sample.len(), 3);
    assert_eq!(*sample[0], "a");
    assert_eq!(*sample[1], "b");
    assert_eq!(*sample[2], "c");
  }
}