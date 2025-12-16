*[itertools](../index.md) / [grouping_map](index.md)*

---

# Module `grouping_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GroupingMapFn`](#groupingmapfn) | struct |  |
| [`GroupingMap`](#groupingmap) | struct | `GroupingMap` is an intermediate struct for efficient group-and-fold operations. |
| [`new_map_for_grouping`](#new-map-for-grouping) | fn |  |
| [`new`](#new) | fn | Creates a new `GroupingMap` from `iter` |
| [`MapForGrouping`](#mapforgrouping) | type | A wrapper to allow for an easy [`into_grouping_map_by`](crate::Itertools::into_grouping_map_by) |
| [`GroupingMapBy`](#groupingmapby) | type | `GroupingMapBy` is an intermediate struct for efficient group-and-fold operations. |

## Structs

### `GroupingMapFn<F>`

```rust
struct GroupingMapFn<F>(F);
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:15`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L15)*

#### Trait Implementations

##### `impl Any for GroupingMapFn<F>`

- <span id="groupingmapfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GroupingMapFn<F>`

- <span id="groupingmapfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GroupingMapFn<F>`

- <span id="groupingmapfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: clone::Clone> Clone for GroupingMapFn<F>`

- <span id="groupingmapfn-clone"></span>`fn clone(&self) -> GroupingMapFn<F>` — [`GroupingMapFn`](#groupingmapfn)

##### `impl CloneToUninit for GroupingMapFn<F>`

- <span id="groupingmapfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<F> Debug for GroupingMapFn<F>`

- <span id="groupingmapfn-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for GroupingMapFn<F>`

- <span id="groupingmapfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GroupingMapFn<F>`

- <span id="groupingmapfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for GroupingMapFn<F>`

##### `impl<V, F: FnMut(&V) -> K> MapSpecialCaseFn for GroupingMapFn<F>`

- <span id="groupingmapfn-mapspecialcasefn-type-out"></span>`type Out = (K, V)`

- <span id="groupingmapfn-mapspecialcasefn-call"></span>`fn call(&mut self, v: V) -> <Self as >::Out` — [`MapSpecialCaseFn`](../adaptors/map/index.md#mapspecialcasefn)

##### `impl ToOwned for GroupingMapFn<F>`

- <span id="groupingmapfn-toowned-type-owned"></span>`type Owned = T`

- <span id="groupingmapfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="groupingmapfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GroupingMapFn<F>`

- <span id="groupingmapfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groupingmapfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GroupingMapFn<F>`

- <span id="groupingmapfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groupingmapfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GroupingMap<I>`

```rust
struct GroupingMap<I> {
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:59-61`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L59-L61)*

`GroupingMap` is an intermediate struct for efficient group-and-fold operations.
It groups elements by their key and at the same time fold each group
using some aggregating operation.

No method on this struct performs temporary allocations.

#### Implementations

- <span id="groupingmap-aggregate"></span>`fn aggregate<FO, R>(self, operation: FO) -> HashMap<K, R>`

  This is the generic way to perform any operation on a `GroupingMap`.
  It's suggested to use this method only to implement custom operations
  when the already provided ones are not enough.
  
  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in an `HashMap`.
  
  The `operation` function is invoked on each element with the following parameters:
   - the current value of the accumulator of the group if there is currently one;
   - a reference to the key of the group this element belongs to;
   - the element from the source being aggregated;
  
  If `operation` returns `Some(element)` then the accumulator is updated with `element`,
  otherwise the previous accumulation is discarded.
  
  Return a `HashMap` associating the key of each group with the result of aggregation of
  that group's elements. If the aggregation of the last element of a group discards the
  accumulator then there won't be an entry associated to that group's key.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![2, 8, 5, 7, 9, 0, 4, 10];
  let lookup = data.into_iter()
      .into_grouping_map_by(|&n| n % 4)
      .aggregate(|acc, _key, val| {
          if val == 0 || val == 10 {
              None
          } else {
              Some(acc.unwrap_or(0) + val)
          }
      });
  
  assert_eq!(lookup[&0], 4);        // 0 resets the accumulator so only 4 is summed
  assert_eq!(lookup[&1], 5 + 9);
  assert_eq!(lookup.get(&2), None); // 10 resets the accumulator and nothing is summed afterward
  assert_eq!(lookup[&3], 7);
  assert_eq!(lookup.len(), 3);      // The final keys are only 0, 1 and 2
  ```

- <span id="groupingmap-fold-with"></span>`fn fold_with<FI, FO, R>(self, init: FI, operation: FO) -> HashMap<K, R>`

  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in a new map.
  
  `init` is called to obtain the initial value of each accumulator.
  
  `operation` is a function that is invoked on each element with the following parameters:
   - the current value of the accumulator of the group;
   - a reference to the key of the group this element belongs to;
   - the element from the source being accumulated.
  
  Return a `HashMap` associating the key of each group with the result of folding that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  #[derive(Debug, Default)]
  struct Accumulator {
    acc: usize,
  }
  
  let lookup = (1..=7)
      .into_grouping_map_by(|&n| n % 3)
      .fold_with(|_key, _val| Default::default(), |Accumulator { acc }, _key, val| {
          let acc = acc + val;
          Accumulator { acc }
       });
  
  assert_eq!(lookup[&0].acc, 3 + 6);
  assert_eq!(lookup[&1].acc, 1 + 4 + 7);
  assert_eq!(lookup[&2].acc, 2 + 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-fold"></span>`fn fold<FO, R>(self, init: R, operation: FO) -> HashMap<K, R>`

  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in a new map.
  
  `init` is the value from which will be cloned the initial value of each accumulator.
  
  `operation` is a function that is invoked on each element with the following parameters:
   - the current value of the accumulator of the group;
   - a reference to the key of the group this element belongs to;
   - the element from the source being accumulated.
  
  Return a `HashMap` associating the key of each group with the result of folding that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = (1..=7)
      .into_grouping_map_by(|&n| n % 3)
      .fold(0, |acc, _key, val| acc + val);
  
  assert_eq!(lookup[&0], 3 + 6);
  assert_eq!(lookup[&1], 1 + 4 + 7);
  assert_eq!(lookup[&2], 2 + 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-reduce"></span>`fn reduce<FO>(self, operation: FO) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in a new map.
  
  This is similar to `fold` but the initial value of the accumulator is the first element of the group.
  
  `operation` is a function that is invoked on each element with the following parameters:
   - the current value of the accumulator of the group;
   - a reference to the key of the group this element belongs to;
   - the element from the source being accumulated.
  
  Return a `HashMap` associating the key of each group with the result of folding that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = (1..=7)
      .into_grouping_map_by(|&n| n % 3)
      .reduce(|acc, _key, val| acc + val);
  
  assert_eq!(lookup[&0], 3 + 6);
  assert_eq!(lookup[&1], 1 + 4 + 7);
  assert_eq!(lookup[&2], 2 + 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-fold-first"></span>`fn fold_first<FO>(self, operation: FO) -> HashMap<K, V>`

  See [`.reduce()`](GroupingMap::reduce).

- <span id="groupingmap-collect"></span>`fn collect<C>(self) -> HashMap<K, C>`

  Groups elements from the `GroupingMap` source by key and collects the elements of each group in
  an instance of `C`. The iteration order is preserved when inserting elements.
  
  Return a `HashMap` associating the key of each group with the collection containing that group's elements.
  
  ```rust
  use itertools::Itertools;
  use std::collections::HashSet;
  
  let lookup = vec![0, 1, 2, 3, 4, 5, 6, 2, 3, 6].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .collect::<HashSet<_>>();
  
  assert_eq!(lookup[&0], vec![0, 3, 6].into_iter().collect::<HashSet<_>>());
  assert_eq!(lookup[&1], vec![1, 4].into_iter().collect::<HashSet<_>>());
  assert_eq!(lookup[&2], vec![2, 5].into_iter().collect::<HashSet<_>>());
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-max"></span>`fn max(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the maximum of each group.
  
  If several elements are equally maximum, the last element is picked.
  
  Returns a `HashMap` associating the key of each group with the maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .max();
  
  assert_eq!(lookup[&0], 12);
  assert_eq!(lookup[&1], 7);
  assert_eq!(lookup[&2], 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-max-by"></span>`fn max_by<F>(self, compare: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the maximum of each group
  with respect to the specified comparison function.
  
  If several elements are equally maximum, the last element is picked.
  
  Returns a `HashMap` associating the key of each group with the maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .max_by(|_key, x, y| y.cmp(x));
  
  assert_eq!(lookup[&0], 3);
  assert_eq!(lookup[&1], 1);
  assert_eq!(lookup[&2], 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-max-by-key"></span>`fn max_by_key<F, CK>(self, f: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the element of each group
  that gives the maximum from the specified function.
  
  If several elements are equally maximum, the last element is picked.
  
  Returns a `HashMap` associating the key of each group with the maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .max_by_key(|_key, &val| val % 4);
  
  assert_eq!(lookup[&0], 3);
  assert_eq!(lookup[&1], 7);
  assert_eq!(lookup[&2], 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-min"></span>`fn min(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the minimum of each group.
  
  If several elements are equally minimum, the first element is picked.
  
  Returns a `HashMap` associating the key of each group with the minimum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .min();
  
  assert_eq!(lookup[&0], 3);
  assert_eq!(lookup[&1], 1);
  assert_eq!(lookup[&2], 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-min-by"></span>`fn min_by<F>(self, compare: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the minimum of each group
  with respect to the specified comparison function.
  
  If several elements are equally minimum, the first element is picked.
  
  Returns a `HashMap` associating the key of each group with the minimum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .min_by(|_key, x, y| y.cmp(x));
  
  assert_eq!(lookup[&0], 12);
  assert_eq!(lookup[&1], 7);
  assert_eq!(lookup[&2], 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-min-by-key"></span>`fn min_by_key<F, CK>(self, f: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the element of each group
  that gives the minimum from the specified function.
  
  If several elements are equally minimum, the first element is picked.
  
  Returns a `HashMap` associating the key of each group with the minimum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .min_by_key(|_key, &val| val % 4);
  
  assert_eq!(lookup[&0], 12);
  assert_eq!(lookup[&1], 4);
  assert_eq!(lookup[&2], 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-minmax"></span>`fn minmax(self) -> HashMap<K, MinMaxResult<V>>` — [`MinMaxResult`](../minmax/index.md#minmaxresult)

  Groups elements from the `GroupingMap` source by key and find the maximum and minimum of
  each group.
  
  If several elements are equally maximum, the last element is picked.
  If several elements are equally minimum, the first element is picked.
  
  See [`Itertools::minmax`](crate::Itertools::minmax) for the non-grouping version.
  
  Differences from the non grouping version:
  - It never produces a `MinMaxResult::NoElements`
  - It doesn't have any speedup
  
  Returns a `HashMap` associating the key of each group with the minimum and maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{OneElement, MinMax};
  
  let lookup = vec![1, 3, 4, 5, 7, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .minmax();
  
  assert_eq!(lookup[&0], MinMax(3, 12));
  assert_eq!(lookup[&1], MinMax(1, 7));
  assert_eq!(lookup[&2], OneElement(5));
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-minmax-by"></span>`fn minmax_by<F>(self, compare: F) -> HashMap<K, MinMaxResult<V>>` — [`MinMaxResult`](../minmax/index.md#minmaxresult)

  Groups elements from the `GroupingMap` source by key and find the maximum and minimum of
  each group with respect to the specified comparison function.
  
  If several elements are equally maximum, the last element is picked.
  If several elements are equally minimum, the first element is picked.
  
  It has the same differences from the non-grouping version as `minmax`.
  
  Returns a `HashMap` associating the key of each group with the minimum and maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{OneElement, MinMax};
  
  let lookup = vec![1, 3, 4, 5, 7, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .minmax_by(|_key, x, y| y.cmp(x));
  
  assert_eq!(lookup[&0], MinMax(12, 3));
  assert_eq!(lookup[&1], MinMax(7, 1));
  assert_eq!(lookup[&2], OneElement(5));
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-minmax-by-key"></span>`fn minmax_by_key<F, CK>(self, f: F) -> HashMap<K, MinMaxResult<V>>` — [`MinMaxResult`](../minmax/index.md#minmaxresult)

  Groups elements from the `GroupingMap` source by key and find the elements of each group
  that gives the minimum and maximum from the specified function.
  
  If several elements are equally maximum, the last element is picked.
  If several elements are equally minimum, the first element is picked.
  
  It has the same differences from the non-grouping version as `minmax`.
  
  Returns a `HashMap` associating the key of each group with the minimum and maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{OneElement, MinMax};
  
  let lookup = vec![1, 3, 4, 5, 7, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .minmax_by_key(|_key, &val| val % 4);
  
  assert_eq!(lookup[&0], MinMax(12, 3));
  assert_eq!(lookup[&1], MinMax(4, 7));
  assert_eq!(lookup[&2], OneElement(5));
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-sum"></span>`fn sum(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and sums them.
  
  This is just a shorthand for `self.reduce(|acc, _, val| acc + val)`.
  It is more limited than `Iterator::sum` since it doesn't use the `Sum` trait.
  
  Returns a `HashMap` associating the key of each group with the sum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .sum();
  
  assert_eq!(lookup[&0], 3 + 9 + 12);
  assert_eq!(lookup[&1], 1 + 4 + 7);
  assert_eq!(lookup[&2], 5 + 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-product"></span>`fn product(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and multiply them.
  
  This is just a shorthand for `self.reduce(|acc, _, val| acc * val)`.
  It is more limited than `Iterator::product` since it doesn't use the `Product` trait.
  
  Returns a `HashMap` associating the key of each group with the product of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .product();
  
  assert_eq!(lookup[&0], 3 * 9 * 12);
  assert_eq!(lookup[&1], 1 * 4 * 7);
  assert_eq!(lookup[&2], 5 * 8);
  assert_eq!(lookup.len(), 3);
  ```

#### Trait Implementations

##### `impl Any for GroupingMap<I>`

- <span id="groupingmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GroupingMap<I>`

- <span id="groupingmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GroupingMap<I>`

- <span id="groupingmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for GroupingMap<I>`

- <span id="groupingmap-clone"></span>`fn clone(&self) -> GroupingMap<I>` — [`GroupingMap`](#groupingmap)

##### `impl CloneToUninit for GroupingMap<I>`

- <span id="groupingmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for GroupingMap<I>`

- <span id="groupingmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GroupingMap<I>`

- <span id="groupingmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GroupingMap<I>`

- <span id="groupingmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for GroupingMap<I>`

##### `impl ToOwned for GroupingMap<I>`

- <span id="groupingmap-toowned-type-owned"></span>`type Owned = T`

- <span id="groupingmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="groupingmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GroupingMap<I>`

- <span id="groupingmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groupingmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GroupingMap<I>`

- <span id="groupingmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groupingmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `new_map_for_grouping`

```rust
fn new_map_for_grouping<K, I: Iterator, F: FnMut(&<I as >::Item) -> K>(iter: I, key_mapper: F) -> crate::adaptors::map::MapSpecialCase<I, GroupingMapFn<F>>
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:28-36`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L28-L36)*

### `new`

```rust
fn new<I, K, V>(iter: I) -> GroupingMap<I>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:39-45`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L39-L45)*

Creates a new `GroupingMap` from `iter`

## Type Aliases

### `MapForGrouping<I, F>`

```rust
type MapForGrouping<I, F> = crate::adaptors::map::MapSpecialCase<I, GroupingMapFn<F>>;
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:12`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L12)*

A wrapper to allow for an easy [`into_grouping_map_by`](crate::Itertools::into_grouping_map_by)

### `GroupingMapBy<I, F>`

```rust
type GroupingMapBy<I, F> = GroupingMap<crate::adaptors::map::MapSpecialCase<I, GroupingMapFn<F>>>;
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:50`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L50)*

`GroupingMapBy` is an intermediate struct for efficient group-and-fold operations.

See [`GroupingMap`](#groupingmap) for more informations.

