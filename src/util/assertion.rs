use chrono::{DateTime, Utc};
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

use crate::dto::request::Direction;

pub fn eq<T>(result: &[T], expected: &[T]) -> bool
where
  T: Eq + Hash,
{
  fn count<T>(items: &[T]) -> HashMap<&T, usize>
  where
    T: Eq + Hash,
  {
    let mut cnt = HashMap::new();
    for i in items {
      *cnt.entry(i).or_insert(0) += 1
    }
    cnt
  }
  count(result) == count(expected)
}

pub fn vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
  a.len() == b.len() && !a.iter().zip(b.iter()).any(|(a, b)| *a != *b)
}

pub fn compare_datetime(left: &DateTime<Utc>, right: &DateTime<Utc>) -> bool {
  left.format("%d/%m/%Y %H:%M").to_string() == right.format("%d/%m/%Y %H:%M").to_string()
}

pub fn exist<T>(haystack: &[T], needle: &T) -> bool
where
  T: PartialEq,
{
  haystack.iter().any(|i| i == needle)
}

pub fn exist_all<T>(haystack: &[T], handful: &[T]) -> bool
where
  T: PartialEq,
{
  handful.iter().all(|i| haystack.contains(i))
}

pub fn is_sorted<I>(items: I, direction: Direction) -> bool
where
  I: IntoIterator,
  I::Item: Ord + Clone,
{
  items
    .into_iter()
    .tuple_windows()
    .all(direction.as_closure())
}

#[test]
fn test_exist_assertion() {
  let h = vec![1, 2, 3];
  let n = 2;
  assert!(exist(&h, &n))
}

#[test]
fn test_not_exist_assertion() {
  let h = vec![1, 2, 3];
  let n = 20;
  assert!(!exist(&h, &n))
}

#[test]
fn exist_all_test() {
  let h = vec![1, 2, 3, 4, 5, 6];
  let n = vec![1, 2, 6];
  assert!(exist_all(&h, &n))
}

#[test]
fn test_not_exist_all_assertion() {
  let h = vec![1, 2, 3];
  let n = vec![1, 2, 60];
  assert!(!exist_all(&h, &n))
}

#[test]
fn test_is_sort_assertion() {
  let a = vec![1, 20, 3];
  let b = vec![1, 2, 60];
  let c = vec![100, 20, 6];
  let d = vec![100, 20, 60];
  assert!(!is_sorted(a, Direction::ASC));
  assert!(is_sorted(b, Direction::ASC));
  assert!(is_sorted(c, Direction::DESC));
  assert!(!is_sorted(d, Direction::DESC))
}
