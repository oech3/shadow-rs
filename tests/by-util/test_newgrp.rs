// This file is part of the shadow-rs package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
// spell-checker:ignore newgrp

//! Integration tests for the `newgrp` utility.
//!
//! `newgrp` replaces the current process via `execv`, so most meaningful
//! tests must verify error paths (the success path never returns).
//! Non-root tests exercise clap parsing and error handling.

use std::ffi::OsString;

/// Run `uumain` with the given args, returning the exit code.
fn run(args: &[&str]) -> i32 {
    let os_args: Vec<OsString> = args.iter().map(|s| (*s).into()).collect();
    newgrp::uumain(os_args.into_iter())
}

// ---------------------------------------------------------------------------
// Non-root tests
// ---------------------------------------------------------------------------

#[test]
fn test_help_exits_zero() {
    let code = run(&["newgrp", "--help"]);
    assert_eq!(code, 0, "--help should exit 0");
}

#[test]
fn test_nonexistent_group() {
    // Should fail because the group does not exist.
    let code = run(&["newgrp", "nonexistent_group_99999"]);
    assert_eq!(code, 1, "nonexistent group should exit 1");
}
