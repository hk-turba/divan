// Tests that ensure weird (but valid) usage behave as expected.

use std::collections::HashSet;

use divan::__private::ENTRIES;

#[divan::bench]
fn lifetime<'a>() -> &'a str {
    "hello"
}

#[divan::bench]
fn embedded() {
    #[divan::bench]
    fn inner() {
        #[divan::bench]
        fn inner() {}
    }
}

#[divan::bench]
fn r#raw_ident() {}

// Test that each function appears the expected number of times.
#[test]
fn count() {
    let mut inner_count = 0;

    for entry in ENTRIES {
        if entry.path.contains("inner") {
            inner_count += 1;
        }
    }

    assert_eq!(inner_count, 2);
}

// Test expected `Entry.path` values.
#[test]
fn path() {
    for entry in ENTRIES {
        // Embedded functions do not contain their parent function's name in
        // their path because that's not how `module_path!()` works.
        if entry.path.contains("inner") {
            assert_eq!(entry.path, "weird_usage::inner");
        }

        // "r#" is removed from raw identifiers.
        if entry.path.contains("raw_ident") {
            assert_eq!(entry.path, "weird_usage::raw_ident");
        }
    }
}

// Test that each benchmarked function has a unique type ID.
#[test]
fn unique_id() {
    let mut seen = HashSet::new();

    for entry in ENTRIES {
        let id = (entry.get_id)();
        assert!(seen.insert(id), "Function '{}' does not have a unique type ID", entry.path);
    }
}
