use in_str::in_str;

#[test]
fn test_in_str() {
  // simple case
  assert!(in_str!("123")('1'));
  assert!(in_str!("123")('2'));
  assert!(in_str!("123")('3'));
  assert!(!in_str!("123")('4'));

  // with escape
  assert!(in_str!("\n\r\t")('\n'));
  assert!(in_str!("\n\r\t")('\r'));
  assert!(in_str!("\n\r\t")('\t'));
  assert!(!in_str!("\n\r\t")('\\'));
  assert!(!in_str!("\n\r\t")('t'));

  // with code point
  assert!(in_str!("\u{1F600}\u{10ffff}")('\u{1F600}'));
  assert!(in_str!("\u{1F600}\u{10ffff}")('\u{10ffff}'));
  assert!(!in_str!("\u{1F600}\u{10ffff}")('\\'));
  assert!(!in_str!("\u{1F600}\u{10ffff}")('u'));
  assert!(!in_str!("\u{1F600}\u{10ffff}")('{'));
}
