// This file is part of the shadow-rs package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
// spell-checker:ignore nscd sssd

//! `nscd` (Name Service Cache Daemon) cache invalidation.
//!
//! After modifying `/etc/passwd`, `/etc/shadow`, or `/etc/group`,
//! the `nscd` cache must be invalidated so lookups reflect the changes.
//! Also supports `sssd` cache invalidation.

use std::process::Command;

/// Invalidate the `nscd` and `sssd` caches for the given database.
///
/// The `database` should be one of `"passwd"`, `"shadow"`, or `"group"`.
///
/// Silently succeeds if `nscd`/`sssd` is not installed or not running —
/// this matches GNU shadow-utils behavior.
///
/// Subprocesses are spawned with a sanitized environment to prevent the
/// caller's full (potentially tainted) env from leaking into child processes
/// running in a setuid context.
pub fn invalidate_cache(database: &str) {
    // Minimal, known-safe environment for child processes.
    let clean_env = [("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")];

    // Use absolute paths to avoid PATH-based lookups in setuid context.
    let _ = Command::new("/usr/sbin/nscd")
        .arg("-i")
        .arg(database)
        .env_clear()
        .envs(clean_env)
        .status();

    // sssd: sss_cache with the appropriate flag
    let flag = match database {
        "passwd" | "shadow" => "-U",
        "group" => "-G",
        _ => return,
    };
    let _ = Command::new("/usr/sbin/sss_cache")
        .arg(flag)
        .env_clear()
        .envs(clean_env)
        .status();
}
