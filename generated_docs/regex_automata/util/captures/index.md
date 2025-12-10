*[regex_automata](../../index.md) / [util](../index.md) / [captures](index.md)*

---

# Module `captures`

Provides types for dealing with capturing groups.

Capturing groups refer to sub-patterns of regexes that some regex engines can
report matching offsets for. For example, matching `[a-z]([0-9]+)` against
`a789` would give `a789` as the overall match (for the implicit capturing group
at index `0`) and `789` as the match for the capturing group `([0-9]+)` (an
explicit capturing group at index `1`).

Not all regex engines can report match offsets for capturing groups. Indeed,
to a first approximation, regex engines that can report capturing group offsets
tend to be quite a bit slower than regex engines that can't. This is because
tracking capturing groups at search time usually requires more "power" that
in turn adds overhead.

Other regex implementations might call capturing groups "submatches."

# Overview

The main types in this module are:

* [`Captures`](#captures) records the capturing group offsets found during a search. It
provides convenience routines for looking up capturing group offsets by either
index or name.
* [`GroupInfo`](#groupinfo) records the mapping between capturing groups and "slots,"
where the latter are how capturing groups are recorded during a regex search.
This also keeps a mapping from capturing group name to index, and capture
group index to name. A `GroupInfo` is used by `Captures` internally to
provide a convenient API. It is unlikely that you'll use a `GroupInfo`
directly, but for example, if you've compiled an Thompson NFA, then you can use
[`thompson::NFA::group_info`](crate::nfa::thompson::NFA::group_info) to get its
underlying `GroupInfo`.

## Contents

- [Structs](#structs)
  - [`Captures`](#captures)
  - [`CapturesDebugMap`](#capturesdebugmap)
  - [`CapturesPatternIter`](#capturespatterniter)
  - [`GroupInfo`](#groupinfo)
  - [`GroupInfoInner`](#groupinfoinner)
  - [`GroupInfoError`](#groupinfoerror)
  - [`GroupInfoPatternNames`](#groupinfopatternnames)
  - [`GroupInfoAllNames`](#groupinfoallnames)
- [Enums](#enums)
  - [`GroupInfoErrorKind`](#groupinfoerrorkind)
- [Type Aliases](#type-aliases)
  - [`CaptureNameMap`](#capturenamemap)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Captures`](#captures) | struct | The span offsets of capturing groups after a match has been found. |
| [`CapturesDebugMap`](#capturesdebugmap) | struct | A little helper type to provide a nice map-like debug representation for our capturing group spans. |
| [`CapturesPatternIter`](#capturespatterniter) | struct | An iterator over all capturing groups in a `Captures` value. |
| [`GroupInfo`](#groupinfo) | struct | Represents information about capturing groups in a compiled regex. |
| [`GroupInfoInner`](#groupinfoinner) | struct | The inner guts of `GroupInfo`. |
| [`GroupInfoError`](#groupinfoerror) | struct | An error that may occur when building a `GroupInfo`. |
| [`GroupInfoPatternNames`](#groupinfopatternnames) | struct | An iterator over capturing groups and their names for a specific pattern. |
| [`GroupInfoAllNames`](#groupinfoallnames) | struct | An iterator over capturing groups and their names for a `GroupInfo`. |
| [`GroupInfoErrorKind`](#groupinfoerrorkind) | enum | The kind of error that occurs when building a `GroupInfo` fails. |
| [`CaptureNameMap`](#capturenamemap) | type | A map from capture group name to its corresponding capture group index. |

## Structs

### `Captures`

```rust
struct Captures {
    group_info: GroupInfo,
    pid: Option<crate::util::primitives::PatternID>,
    slots: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:130-173`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L130-L173)*

The span offsets of capturing groups after a match has been found.

This type represents the output of regex engines that can report the
offsets at which capturing groups matches or "submatches" occur. For
example, the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). When a match
occurs, it will at minimum contain the [`PatternID`](../primitives/index.md) of the pattern that
matched. Depending upon how it was constructed, it may also contain the
start/end offsets of the entire match of the pattern and the start/end
offsets of each capturing group that participated in the match.

Values of this type are always created for a specific [`GroupInfo`](#groupinfo). It is
unspecified behavior to use a `Captures` value in a search with any regex
engine that has a different `GroupInfo` than the one the `Captures` were
created with.

# Constructors

There are three constructors for this type that control what kind of
information is available upon a match:

* `Captures::all`: Will store overall pattern match offsets in addition
to the offsets of capturing groups that participated in the match.
* `Captures::matches`: Will store only the overall pattern
match offsets. The offsets of capturing groups (even ones that participated
in the match) are not available.
* `Captures::empty`: Will only store the pattern ID that matched. No
match offsets are available at all.

If you aren't sure which to choose, then pick the first one. The first one
is what convenience routines like,
[`PikeVM::create_captures`](crate::nfa::thompson::pikevm::PikeVM::create_captures),
will use automatically.

The main difference between these choices is performance. Namely, if you
ask for _less_ information, then the execution of regex search may be able
to run more quickly.

# Notes

It is worth pointing out that this type is not coupled to any one specific
regex engine. Instead, its coupling is with [`GroupInfo`](#groupinfo), which is the
thing that is responsible for mapping capturing groups to "slot" offsets.
Slot offsets are indices into a single sequence of memory at which matching
haystack offsets for the corresponding group are written by regex engines.

# Example

This example shows how to parse a simple date and extract the components of
the date via capturing groups:

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group(1));
assert_eq!(Some(Span::from(5..7)), caps.get_group(2));
assert_eq!(Some(Span::from(8..10)), caps.get_group(3));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: named capturing groups

This example is like the one above, but leverages the ability to name
capturing groups in order to make the code a bit clearer:

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^(?P<y>[0-9]{4})-(?P<m>[0-9]{2})-(?P<d>[0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group_by_name("y"));
assert_eq!(Some(Span::from(5..7)), caps.get_group_by_name("m"));
assert_eq!(Some(Span::from(8..10)), caps.get_group_by_name("d"));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`group_info`**: `GroupInfo`

  The group info that these capture groups are coupled to. This is what
  gives the "convenience" of the `Captures` API. Namely, it provides the
  slot mapping and the name|-->index mapping for capture lookups by name.

- **`pid`**: `Option<crate::util::primitives::PatternID>`

  The ID of the pattern that matched. Regex engines must set this to
  None when no match occurs.

- **`slots`**: `alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>`

  The slot values, i.e., submatch offsets.
  
  In theory, the smallest sequence of slots would be something like
  `max(groups(pattern) for pattern in regex) * 2`, but instead, we use
  `sum(groups(pattern) for pattern in regex) * 2`. Why?
  
  Well, the former could be used in theory, because we don't generally
  have any overlapping APIs that involve capturing groups. Therefore,
  there's technically never any need to have slots set for multiple
  patterns. However, this might change some day, in which case, we would
  need to have slots available.
  
  The other reason is that during the execution of some regex engines,
  there exists a point in time where multiple slots for different
  patterns may be written to before knowing which pattern has matched.
  Therefore, the regex engines themselves, in order to support multiple
  patterns correctly, must have all slots available. If `Captures`
  doesn't have all slots available, then regex engines can't write
  directly into the caller provided `Captures` and must instead write
  into some other storage and then copy the slots involved in the match
  at the end of the search.
  
  So overall, at least as of the time of writing, it seems like the path
  of least resistance is to just require allocating all possible slots
  instead of the conceptual minimum. Another way to justify this is that
  the most common case is a single pattern, in which case, there is no
  inefficiency here since the 'max' and 'sum' calculations above are
  equivalent in that case.
  
  N.B. The mapping from group index to slot is maintained by `GroupInfo`
  and is considered an API guarantee. See `GroupInfo` for more details on
  that mapping.
  
  N.B. `Option<NonMaxUsize>` has the same size as a `usize`.

#### Implementations

- <span id="captures-all"></span>`fn all(group_info: GroupInfo) -> Captures` — [`GroupInfo`](#groupinfo), [`Captures`](#captures)

- <span id="captures-matches"></span>`fn matches(group_info: GroupInfo) -> Captures` — [`GroupInfo`](#groupinfo), [`Captures`](#captures)

- <span id="captures-empty"></span>`fn empty(group_info: GroupInfo) -> Captures` — [`GroupInfo`](#groupinfo), [`Captures`](#captures)

- <span id="captures-is-match"></span>`fn is_match(&self) -> bool`

- <span id="captures-pattern"></span>`fn pattern(&self) -> Option<PatternID>` — [`PatternID`](../primitives/index.md#patternid)

- <span id="captures-get-match"></span>`fn get_match(&self) -> Option<Match>` — [`Match`](../../index.md#match)

- <span id="captures-get-group"></span>`fn get_group(&self, index: usize) -> Option<Span>` — [`Span`](../../index.md#span)

- <span id="captures-get-group-by-name"></span>`fn get_group_by_name(&self, name: &str) -> Option<Span>` — [`Span`](../../index.md#span)

- <span id="captures-iter"></span>`fn iter(&self) -> CapturesPatternIter<'_>` — [`CapturesPatternIter`](#capturespatterniter)

- <span id="captures-group-len"></span>`fn group_len(&self) -> usize`

- <span id="captures-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](#groupinfo)

- <span id="captures-interpolate-string"></span>`fn interpolate_string(&self, haystack: &str, replacement: &str) -> String`

- <span id="captures-interpolate-string-into"></span>`fn interpolate_string_into(&self, haystack: &str, replacement: &str, dst: &mut String)`

- <span id="captures-interpolate-bytes"></span>`fn interpolate_bytes(&self, haystack: &[u8], replacement: &[u8]) -> Vec<u8>`

- <span id="captures-interpolate-bytes-into"></span>`fn interpolate_bytes_into(&self, haystack: &[u8], replacement: &[u8], dst: &mut Vec<u8>)`

- <span id="captures-extract"></span>`fn extract<'h, const N: usize>(&self, haystack: &'h str) -> (&'h str, [&'h str; N])`

- <span id="captures-extract-bytes"></span>`fn extract_bytes<'h, const N: usize>(&self, haystack: &'h [u8]) -> (&'h [u8], [&'h [u8]; N])`

#### Trait Implementations

##### `impl Clone for Captures`

- <span id="captures-clone"></span>`fn clone(&self) -> Captures` — [`Captures`](#captures)

##### `impl Debug for Captures`

- <span id="captures-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CapturesDebugMap<'a>`

```rust
struct CapturesDebugMap<'a> {
    pid: crate::util::primitives::PatternID,
    caps: &'a Captures,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:1217-1220`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L1217-L1220)*

A little helper type to provide a nice map-like debug representation for
our capturing group spans.

#### Trait Implementations

##### `impl Debug for CapturesDebugMap<'a>`

- <span id="capturesdebugmap-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CapturesPatternIter<'a>`

```rust
struct CapturesPatternIter<'a> {
    caps: &'a Captures,
    names: core::iter::Enumerate<GroupInfoPatternNames<'a>>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:1258-1261`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L1258-L1261)*

An iterator over all capturing groups in a `Captures` value.

This iterator includes capturing groups that did not participate in a
match. See the `Captures::iter` method documentation for more details
and examples.

The lifetime parameter `'a` refers to the lifetime of the underlying
`Captures` value.

#### Trait Implementations

##### `impl Clone for CapturesPatternIter<'a>`

- <span id="capturespatterniter-clone"></span>`fn clone(&self) -> CapturesPatternIter<'a>` — [`CapturesPatternIter`](#capturespatterniter)

##### `impl Debug for CapturesPatternIter<'a>`

- <span id="capturespatterniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for CapturesPatternIter<'a>`

##### `impl FusedIterator for CapturesPatternIter<'a>`

##### `impl IntoIterator for CapturesPatternIter<'a>`

- <span id="capturespatterniter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturespatterniter-type-intoiter"></span>`type IntoIter = I`

- <span id="capturespatterniter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CapturesPatternIter<'a>`

- <span id="capturespatterniter-type-item"></span>`type Item = Option<Span>`

- <span id="capturespatterniter-next"></span>`fn next(&mut self) -> Option<Option<Span>>` — [`Span`](../../index.md#span)

- <span id="capturespatterniter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="capturespatterniter-count"></span>`fn count(self) -> usize`

### `GroupInfo`

```rust
struct GroupInfo(alloc::sync::Arc<GroupInfoInner>);
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:1451`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L1451)*

Represents information about capturing groups in a compiled regex.

The information encapsulated by this type consists of the following. For
each pattern:

* A map from every capture group name to its corresponding capture group
index.
* A map from every capture group index to its corresponding capture group
name.
* A map from capture group index to its corresponding slot index. A slot
refers to one half of a capturing group. That is, a capture slot is either
the start or end of a capturing group. A slot is usually the mechanism
by which a regex engine records offsets for each capturing group during a
search.

A `GroupInfo` uses reference counting internally and is thus cheap to
clone.

# Mapping from capture groups to slots

One of the main responsibilities of a `GroupInfo` is to build a mapping
from `(PatternID, u32)` (where the `u32` is a capture index) to something
called a "slot." As mentioned above, a slot refers to one half of a
capturing group. Both combined provide the start and end offsets of
a capturing group that participated in a match.

**The mapping between group indices and slots is an API guarantee.** That
is, the mapping won't change within a semver compatible release.

Slots exist primarily because this is a convenient mechanism by which
regex engines report group offsets at search time. For example, the
[`nfa::thompson::State::Capture`](crate::nfa::thompson::State::Capture)
NFA state includes the slot index. When a regex engine transitions through
this state, it will likely use the slot index to write the current haystack
offset to some region of memory. When a match is found, those slots are
then reported to the caller, typically via a convenient abstraction like a
[`Captures`](#captures) value.

Because this crate provides first class support for multi-pattern regexes,
and because of some performance related reasons, the mapping between
capturing groups and slots is a little complex. However, in the case of a
single pattern, the mapping can be described very simply: for all capture
group indices `i`, its corresponding slots are at `i * 2` and `i * 2 + 1`.
Notice that the pattern ID isn't involved at all here, because it only
applies to a single-pattern regex, it is therefore always `0`.

In the multi-pattern case, the mapping is a bit more complicated. To talk
about it, we must define what we mean by "implicit" vs "explicit"
capturing groups:

* An **implicit** capturing group refers to the capturing group that is
present for every pattern automatically, and corresponds to the overall
match of a pattern. Every pattern has precisely one implicit capturing
group. It is always unnamed and it always corresponds to the capture group
index `0`.
* An **explicit** capturing group refers to any capturing group that
appears in the concrete syntax of the pattern. (Or, if an NFA was hand
built without any concrete syntax, it refers to any capturing group with an
index greater than `0`.)

Some examples:

* `\w+` has one implicit capturing group and zero explicit capturing
groups.
* `(\w+)` has one implicit group and one explicit group.
* `foo(\d+)(?:\pL+)(\d+)` has one implicit group and two explicit groups.

Turning back to the slot mapping, we can now state it as follows:

* Given a pattern ID `pid`, the slots for its implicit group are always
at `pid * 2` and `pid * 2 + 1`.
* Given a pattern ID `0`, the slots for its explicit groups start
at `group_info.pattern_len() * 2`.
* Given a pattern ID `pid > 0`, the slots for its explicit groups start
immediately following where the slots for the explicit groups of `pid - 1`
end.

In particular, while there is a concrete formula one can use to determine
where the slots for the implicit group of any pattern are, there is no
general formula for determining where the slots for explicit capturing
groups are. This is because each pattern can contain a different number
of groups.

The intended way of getting the slots for a particular capturing group
(whether implicit or explicit) is via the `GroupInfo::slot` or
`GroupInfo::slots` method.

See below for a concrete example of how capturing groups get mapped to
slots.

# Example

This example shows how to build a new `GroupInfo` and query it for
information.

```rust
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;
// The number of patterns being tracked.
assert_eq!(4, info.pattern_len());
// We can query the number of groups for any pattern.
assert_eq!(2, info.group_len(PatternID::must(0)));
assert_eq!(1, info.group_len(PatternID::must(1)));
assert_eq!(5, info.group_len(PatternID::must(2)));
assert_eq!(3, info.group_len(PatternID::must(3)));
// An invalid pattern always has zero groups.
assert_eq!(0, info.group_len(PatternID::must(999)));
// 2 slots per group
assert_eq!(22, info.slot_len());

// We can map a group index for a particular pattern to its name, if
// one exists.
assert_eq!(Some("foo"), info.to_name(PatternID::must(3), 2));
assert_eq!(None, info.to_name(PatternID::must(2), 4));
// Or map a name to its group index.
assert_eq!(Some(1), info.to_index(PatternID::must(0), "foo"));
assert_eq!(Some(2), info.to_index(PatternID::must(3), "foo"));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: mapping from capture groups to slots

This example shows the specific mapping from capture group indices for
each pattern to their corresponding slots. The slot values shown in this
example are considered an API guarantee.

```rust
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;

// We first show the slots for each pattern's implicit group.
assert_eq!(Some((0, 1)), info.slots(PatternID::must(0), 0));
assert_eq!(Some((2, 3)), info.slots(PatternID::must(1), 0));
assert_eq!(Some((4, 5)), info.slots(PatternID::must(2), 0));
assert_eq!(Some((6, 7)), info.slots(PatternID::must(3), 0));

// And now we show the slots for each pattern's explicit group.
assert_eq!(Some((8, 9)), info.slots(PatternID::must(0), 1));
assert_eq!(Some((10, 11)), info.slots(PatternID::must(2), 1));
assert_eq!(Some((12, 13)), info.slots(PatternID::must(2), 2));
assert_eq!(Some((14, 15)), info.slots(PatternID::must(2), 3));
assert_eq!(Some((16, 17)), info.slots(PatternID::must(2), 4));
assert_eq!(Some((18, 19)), info.slots(PatternID::must(3), 1));
assert_eq!(Some((20, 21)), info.slots(PatternID::must(3), 2));

// Asking for the slots for an invalid pattern ID or even for an invalid
// group index for a specific pattern will return None. So for example,
// you're guaranteed to not get the slots for a different pattern than the
// one requested.
assert_eq!(None, info.slots(PatternID::must(5), 0));
assert_eq!(None, info.slots(PatternID::must(1), 1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="groupinfo-new"></span>`fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>` — [`GroupInfo`](#groupinfo), [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfo-empty"></span>`fn empty() -> GroupInfo` — [`GroupInfo`](#groupinfo)

- <span id="groupinfo-to-index"></span>`fn to_index(&self, pid: PatternID, name: &str) -> Option<usize>` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfo-to-name"></span>`fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str>` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfo-pattern-names"></span>`fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_>` — [`PatternID`](../primitives/index.md#patternid), [`GroupInfoPatternNames`](#groupinfopatternnames)

- <span id="groupinfo-all-names"></span>`fn all_names(&self) -> GroupInfoAllNames<'_>` — [`GroupInfoAllNames`](#groupinfoallnames)

- <span id="groupinfo-slots"></span>`fn slots(&self, pid: PatternID, group_index: usize) -> Option<(usize, usize)>` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfo-slot"></span>`fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize>` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfo-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="groupinfo-group-len"></span>`fn group_len(&self, pid: PatternID) -> usize` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfo-all-group-len"></span>`fn all_group_len(&self) -> usize`

- <span id="groupinfo-slot-len"></span>`fn slot_len(&self) -> usize`

- <span id="groupinfo-implicit-slot-len"></span>`fn implicit_slot_len(&self) -> usize`

- <span id="groupinfo-explicit-slot-len"></span>`fn explicit_slot_len(&self) -> usize`

- <span id="groupinfo-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for GroupInfo`

- <span id="groupinfo-clone"></span>`fn clone(&self) -> GroupInfo` — [`GroupInfo`](#groupinfo)

##### `impl Debug for GroupInfo`

- <span id="groupinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GroupInfo`

- <span id="groupinfo-default"></span>`fn default() -> GroupInfo` — [`GroupInfo`](#groupinfo)

### `GroupInfoInner`

```rust
struct GroupInfoInner {
    slot_ranges: alloc::vec::Vec<(crate::util::primitives::SmallIndex, crate::util::primitives::SmallIndex)>,
    name_to_index: alloc::vec::Vec<std::collections::HashMap<alloc::sync::Arc<str>, crate::util::primitives::SmallIndex>>,
    index_to_name: alloc::vec::Vec<alloc::vec::Vec<Option<alloc::sync::Arc<str>>>>,
    memory_extra: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:2179-2184`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L2179-L2184)*

The inner guts of `GroupInfo`. This type only exists so that it can
be wrapped in an `Arc` to make `GroupInfo` reference counted.

#### Implementations

- <span id="groupinfoinner-add-first-group"></span>`fn add_first_group(&mut self, pid: PatternID)` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfoinner-add-explicit-group"></span>`fn add_explicit_group<N: AsRef<str>>(&mut self, pid: PatternID, group: SmallIndex, maybe_name: Option<N>) -> Result<(), GroupInfoError>` — [`PatternID`](../primitives/index.md#patternid), [`SmallIndex`](../primitives/index.md#smallindex), [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfoinner-fixup-slot-ranges"></span>`fn fixup_slot_ranges(&mut self) -> Result<(), GroupInfoError>` — [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfoinner-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="groupinfoinner-group-len"></span>`fn group_len(&self, pid: PatternID) -> usize` — [`PatternID`](../primitives/index.md#patternid)

- <span id="groupinfoinner-small-slot-len"></span>`fn small_slot_len(&self) -> SmallIndex` — [`SmallIndex`](../primitives/index.md#smallindex)

#### Trait Implementations

##### `impl Debug for GroupInfoInner`

- <span id="groupinfoinner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GroupInfoInner`

- <span id="groupinfoinner-default"></span>`fn default() -> GroupInfoInner` — [`GroupInfoInner`](#groupinfoinner)

### `GroupInfoError`

```rust
struct GroupInfoError {
    kind: GroupInfoErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:2337-2339`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L2337-L2339)*

An error that may occur when building a `GroupInfo`.

Building a `GroupInfo` does a variety of checks to make sure the
capturing groups satisfy a number of invariants. This includes, but is not
limited to, ensuring that the first capturing group is unnamed and that
there are no duplicate capture groups for a specific pattern.

#### Implementations

- <span id="groupinfoerror-too-many-patterns"></span>`fn too_many_patterns(err: PatternIDError) -> GroupInfoError` — [`PatternIDError`](../primitives/index.md#patterniderror), [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfoerror-too-many-groups"></span>`fn too_many_groups(pattern: PatternID, minimum: usize) -> GroupInfoError` — [`PatternID`](../primitives/index.md#patternid), [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfoerror-missing-groups"></span>`fn missing_groups(pattern: PatternID) -> GroupInfoError` — [`PatternID`](../primitives/index.md#patternid), [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfoerror-first-must-be-unnamed"></span>`fn first_must_be_unnamed(pattern: PatternID) -> GroupInfoError` — [`PatternID`](../primitives/index.md#patternid), [`GroupInfoError`](#groupinfoerror)

- <span id="groupinfoerror-duplicate"></span>`fn duplicate(pattern: PatternID, name: &str) -> GroupInfoError` — [`PatternID`](../primitives/index.md#patternid), [`GroupInfoError`](#groupinfoerror)

#### Trait Implementations

##### `impl Clone for GroupInfoError`

- <span id="groupinfoerror-clone"></span>`fn clone(&self) -> GroupInfoError` — [`GroupInfoError`](#groupinfoerror)

##### `impl Debug for GroupInfoError`

- <span id="groupinfoerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for GroupInfoError`

- <span id="groupinfoerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for GroupInfoError`

- <span id="groupinfoerror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for GroupInfoError`

- <span id="groupinfoerror-to-string"></span>`fn to_string(&self) -> String`

### `GroupInfoPatternNames<'a>`

```rust
struct GroupInfoPatternNames<'a> {
    it: core::slice::Iter<'a, Option<alloc::sync::Arc<str>>>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:2480-2482`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L2480-L2482)*

An iterator over capturing groups and their names for a specific pattern.

This iterator is created by `GroupInfo::pattern_names`.

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

#### Implementations

- <span id="groupinfopatternnames-empty"></span>`fn empty() -> GroupInfoPatternNames<'static>` — [`GroupInfoPatternNames`](#groupinfopatternnames)

#### Trait Implementations

##### `impl Clone for GroupInfoPatternNames<'a>`

- <span id="groupinfopatternnames-clone"></span>`fn clone(&self) -> GroupInfoPatternNames<'a>` — [`GroupInfoPatternNames`](#groupinfopatternnames)

##### `impl Debug for GroupInfoPatternNames<'a>`

- <span id="groupinfopatternnames-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for GroupInfoPatternNames<'a>`

##### `impl FusedIterator for GroupInfoPatternNames<'a>`

##### `impl IntoIterator for GroupInfoPatternNames<'a>`

- <span id="groupinfopatternnames-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="groupinfopatternnames-type-intoiter"></span>`type IntoIter = I`

- <span id="groupinfopatternnames-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for GroupInfoPatternNames<'a>`

- <span id="groupinfopatternnames-type-item"></span>`type Item = Option<&'a str>`

- <span id="groupinfopatternnames-next"></span>`fn next(&mut self) -> Option<Option<&'a str>>`

- <span id="groupinfopatternnames-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="groupinfopatternnames-count"></span>`fn count(self) -> usize`

### `GroupInfoAllNames<'a>`

```rust
struct GroupInfoAllNames<'a> {
    group_info: &'a GroupInfo,
    pids: crate::util::primitives::PatternIDIter,
    current_pid: Option<crate::util::primitives::PatternID>,
    names: Option<core::iter::Enumerate<GroupInfoPatternNames<'a>>>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:2516-2521`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L2516-L2521)*

An iterator over capturing groups and their names for a `GroupInfo`.

This iterator is created by `GroupInfo::all_names`.

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

#### Trait Implementations

##### `impl Debug for GroupInfoAllNames<'a>`

- <span id="groupinfoallnames-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for GroupInfoAllNames<'a>`

- <span id="groupinfoallnames-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="groupinfoallnames-type-intoiter"></span>`type IntoIter = I`

- <span id="groupinfoallnames-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for GroupInfoAllNames<'a>`

- <span id="groupinfoallnames-type-item"></span>`type Item = (PatternID, usize, Option<&'a str>)`

- <span id="groupinfoallnames-next"></span>`fn next(&mut self) -> Option<(PatternID, usize, Option<&'a str>)>` — [`PatternID`](../primitives/index.md#patternid)

## Enums

### `GroupInfoErrorKind`

```rust
enum GroupInfoErrorKind {
    TooManyPatterns {
        err: crate::util::primitives::PatternIDError,
    },
    TooManyGroups {
        pattern: crate::util::primitives::PatternID,
        minimum: usize,
    },
    MissingGroups {
        pattern: crate::util::primitives::PatternID,
    },
    FirstMustBeUnnamed {
        pattern: crate::util::primitives::PatternID,
    },
    Duplicate {
        pattern: crate::util::primitives::PatternID,
        name: alloc::string::String,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:2346-2387`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L2346-L2387)*

The kind of error that occurs when building a `GroupInfo` fails.

We keep this un-exported because it's not clear how useful it is to
export it.

#### Variants

- **`TooManyPatterns`**

  This occurs when too many patterns have been added. i.e., It would
  otherwise overflow a `PatternID`.

- **`TooManyGroups`**

  This occurs when too many capturing groups have been added for a
  particular pattern.

- **`MissingGroups`**

  An error that occurs when a pattern has no capture groups. Either the
  group info must be empty, or all patterns must have at least one group
  (corresponding to the unnamed group for the entire pattern).

- **`FirstMustBeUnnamed`**

  An error that occurs when one tries to provide a name for the capture
  group at index 0. This capturing group must currently always be
  unnamed.

- **`Duplicate`**

  An error that occurs when duplicate capture group names for the same
  pattern are added.
  
  NOTE: At time of writing, this error can never occur if you're using
  regex-syntax, since the parser itself will reject patterns with
  duplicate capture group names. This error can only occur when the
  builder is used to hand construct NFAs.

#### Trait Implementations

##### `impl Clone for GroupInfoErrorKind`

- <span id="groupinfoerrorkind-clone"></span>`fn clone(&self) -> GroupInfoErrorKind` — [`GroupInfoErrorKind`](#groupinfoerrorkind)

##### `impl Debug for GroupInfoErrorKind`

- <span id="groupinfoerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `CaptureNameMap`

```rust
type CaptureNameMap = std::collections::HashMap<alloc::sync::Arc<str>, crate::util::primitives::SmallIndex>;
```

*Defined in [`regex-automata-0.4.13/src/util/captures.rs:2172`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/captures.rs#L2172)*

A map from capture group name to its corresponding capture group index.

This type is actually wrapped inside a Vec indexed by pattern ID on a
`GroupInfo`, since multiple patterns may have the same capture group name.
That is, each pattern gets its own namespace of capture group names.

Perhaps a more memory efficient representation would be
HashMap<(PatternID, Arc<str>), usize>, but this makes it difficult to look
up a capture index by name without producing a `Arc<str>`, which requires
an allocation. To fix this, I think we'd need to define our own unsized
type or something? Anyway, I didn't give this much thought since it
probably doesn't matter much in the grand scheme of things. But it did
stand out to me as mildly wasteful.

