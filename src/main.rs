//! demo-cli — a tiny, zero-dependency CLI used to demo the axp `/axp run` PR
//! trigger. CI builds this into a Linux binary, `axp ci upload` ships it into
//! the experiment's source-less `demo-cli` slot, and the agent under test runs
//! it inside the sandbox.
//!
//! The `hash` command emits a deterministic FNV-1a-64 digest of its argument.
//! It is deliberately something an agent cannot compute in its head, so a
//! correct answer is evidence the agent actually invoked the compiled binary.

use std::env;
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "\
demo-cli — axp PR-trigger demo tool

USAGE:
    demo-cli <COMMAND> [ARGS]

COMMANDS:
    hash <TEXT>     Print the FNV-1a-64 digest of TEXT as 16 hex chars
    greet <NAME>    Print a friendly greeting
    help            Show this help
    version         Show the version

FLAGS:
    -h, --help      Show this help
    -V, --version   Show the version";

/// FNV-1a 64-bit hash. Stable across platforms — no crates, no surprises.
fn fnv1a_64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("hash") => match args.get(1) {
            Some(text) => {
                println!("{:016x}", fnv1a_64(text.as_bytes()));
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("error: `hash` needs a TEXT argument\n\n{USAGE}");
                ExitCode::FAILURE
            }
        },
        Some("greet") => match args.get(1) {
            Some(name) => {
                println!("Hello, {name}! — from demo-cli v{VERSION}");
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("error: `greet` needs a NAME argument\n\n{USAGE}");
                ExitCode::FAILURE
            }
        },
        Some("version") | Some("--version") | Some("-V") => {
            println!("demo-cli {VERSION}");
            ExitCode::SUCCESS
        }
        Some("help") | Some("--help") | Some("-h") | None => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("error: unknown command `{other}`\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}
