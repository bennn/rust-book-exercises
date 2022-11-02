//! In this file, you will design functions to implement a high-level specification.
//! The goal is to have you explore the different possible implementations of a spec in Rust,
//! and to articulate the trade-offs in terms of generality, performance, and usability.

// EXAMPLE: below is a completed function that demonstrates how each problem will work.
// Each problem contains a specification above the function. Your goal is to design the function
// signature and implementation. For each parameter and the return type, you should describe
// (a) a reasonable space of possible types, and (b) your rationale for picking a particular type.
// Make sure to add at least one unit test for your particular implementation.

/// round_all is a function that takes:
///   * v: representing a collection of numbers
/// and rounds every number in-place in v to the nearest integer.
pub fn round_all(
  // (1) v could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &mut[_] because
  //     we do not need to change the size or order of the collection, but do need to change the elements.
  // (2) v could be a &mut [{number type}], and "round to the nearest integer" implies the use of floats.
  // (3) The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
  //     function that works for both types, so we arbitrarily pick f32 for now.
  v: &mut [f32],
)
// No return value, since this function only mutates an input.
{
  for n in v.iter_mut() {
    *n = n.round();
  }
}

#[test]
fn round_all_test() {
  let mut v = vec![0.3, 0.7];
  round_all(&mut v);
  assert_eq!(v, vec![0., 1.]);
}

// Now you try!

/// P2a: find_contains is a function that takes:
///   * haystack: representing a collection of strings
///   * needle: representing a particular string
/// and returns a value:
///   * representing which strings in the collection contain the needle
pub fn find_contains(
  haystack: &Vec<&str>,
  // haystack must be a collection. Don't need write access (no mut). Choosing Vec over [_] because
  // it seems more common (though, we don't need to resize). Using &str because we don't mutate the
  // elements.
  needle: &str
  // needle is a reference to a string range. No copying, no mutating, no need to choose at the
  // call site between ranges and strings.
) -> Vec<usize>
// return collection of indices into the haystack
// if k in the result, then haystack[k] contains the needle
// - could go further, with indices into the strings too
// - usize = don't care about the number type
{
  let mut found = Vec::new();
  for (ii, str) in haystack.iter().enumerate() {
    if str.contains(needle) {
      found.push(ii)
    }
  }
  return found
}

#[test]
fn find_contains_test() {
  let v = vec!["apple", "banana", "cherry", "coconut", "grape"];
  assert_eq!(find_contains(&v, "ap"), vec![0, 4])
}

/// P2b: fill_progress_bar is a function that takes:
///   * buf: a string to fill
///   * delims: a pair of delimiters to wrap the bar
///   * frac: the fraction of the bar to display
/// Then places a textual representation of the progress bar into `buf`.
/// For example, at a progress of 20% with bracketed delimiters, the bar would be:
///   [==        ]
pub fn fill_progress_bar(
  buf:&mut str,
  // buf = mutable sequence. Need to mutate, otherwise free choice
  delims: (&char, &char),
  // delims = 2 chars, read only
  // could ask for a symbol representing a fixed set of defaults (bracket, paren, ....)
  // could allow multi-char delims
  // but those are harder for me to work with
  frac: usize
  // frac = integer between 0 and 100
)
// no return type, mutate buf
{
  let ll = buf.len();
  let tofill = (((ll - 2) as f32) * ( (frac as f32) / 100.0 )) as usize;
  let buf2 = unsafe { buf.as_bytes_mut() };
  for ii in 0..ll {
    if ii == 0 {
      buf2[ii] = delims.0.clone() as u8
    } else if ii == (ll-1) {
      buf2[ii] = delims.1.clone() as u8
    } else if ii <= tofill {
      buf2[ii] = '=' as u8
    }
  }
  return
}

#[test]
fn test_fill_progress_bar() {
  let mut buf = String::from("            ");
  fill_progress_bar(&mut buf, (&'[', &']'), 90);
  assert_eq!(buf, "[========= ]")
}
