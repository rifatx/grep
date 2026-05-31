// This file is part of the uutils grep package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// End-to-end search throughput, driven through the real `uumain` entry point
/// so the whole pipeline (input buffering, searcher, output) is exercised.
///
/// Matching a pattern against an already-split line in isolation cannot reveal
/// how the *searcher* feeds data to the matcher (e.g. scanning whole buffers
/// instead of testing one line at a time). These cases do: a literal pattern
/// (which a buffer-at-a-time engine can accelerate) and an extended-regex
/// control (which cannot), over a multi-megabyte file.
fn bench_search(c: &mut Criterion) {
    use std::ffi::OsString;

    // A log-like file large enough to cross many internal read buffers.
    let mut content = String::new();
    for i in 0..80_000u32 {
        if i % 100 == 0 {
            content.push_str(&format!(
                "2024-01-15 10:30:{:02} ERROR worker-{i} connection reset\n",
                i % 60
            ));
        } else {
            content.push_str(&format!(
                "2024-01-15 10:30:{:02} INFO  worker-{i} request handled in {}ms\n",
                i % 60,
                i % 1000
            ));
        }
    }
    assert!(content.len() > 4 * 1024 * 1024);

    let mut path = std::env::temp_dir();
    path.push(format!("uu_grep_bench_{}.log", std::process::id()));
    std::fs::write(&path, &content).unwrap();
    let path_arg = path.clone().into_os_string();

    // `-q` with a pattern that never matches forces a full scan of the file and
    // produces no output, so the timing reflects pure scanning throughput.
    let run = |extra_flag: Option<&str>, pattern: &str| {
        let mut args: Vec<OsString> = vec![OsString::from("grep"), OsString::from("-q")];
        if let Some(flag) = extra_flag {
            args.push(OsString::from(flag));
        }
        args.push(OsString::from(pattern));
        args.push(path_arg.clone());
        // No match => Err(exit code 1); we only care about the work, not status.
        let _ = uu_grep::uumain(args.into_iter());
    };

    let mut group = c.benchmark_group("search");

    group.bench_function("scan_literal_no_match", |b| {
        b.iter(|| run(None, black_box("NONEXISTENT_TOKEN_XYZ")))
    });

    group.bench_function("scan_regex_no_match", |b| {
        b.iter(|| run(Some("-E"), black_box("NON[0-9]EXISTENT_TOKEN")))
    });

    group.finish();
    let _ = std::fs::remove_file(&path);
}

criterion_group!(benches, bench_search);
criterion_main!(benches);
