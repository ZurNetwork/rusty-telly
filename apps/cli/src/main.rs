//! A command-line UI over the command engine. It serves a Full session over
//! the in-memory demo account until the MTProto adapter lands.
//!
//! ```text
//! cli commands                                         # the Full edition's commands
//! printf 'chat.open {"chat":100}\nmessage.recent\n' | cli
//! ```
//!
//! Each stdin line is one command, `<id> [json params]`, and each result is
//! printed as one line of JSON.

use std::io::{self, BufRead};
use std::process::ExitCode;
use std::rc::Rc;

use application::commands::{self, CommandError};
use application::{Edition, Ports, Session};
use serde_json::Value;

/// Why one stdin line failed.
#[derive(Debug, thiserror::Error)]
enum LineError {
    #[error("params are not JSON: {0}")]
    Params(#[from] serde_json::Error),
    #[error(transparent)]
    Command(#[from] CommandError),
}

fn main() -> ExitCode {
    let argument = std::env::args().nth(1);
    match argument.as_deref() {
        None => pollster::block_on(serve_stdin()),
        Some("commands") => list_commands(),
        Some(other) => {
            eprintln!("unknown argument `{other}`: run `cli commands`, or pipe commands on stdin");
            ExitCode::FAILURE
        }
    }
}

fn list_commands() -> ExitCode {
    for spec in commands::for_edition(Edition::Full) {
        println!("{:<16} {:<18} {}", spec.id, spec.label, spec.params);
    }
    ExitCode::SUCCESS
}

async fn serve_stdin() -> ExitCode {
    let demo = Rc::new(adapter_mem::fixture::demo());
    let mut session = Session::full(Ports::from(demo));
    let mut any_failed = false;

    for line in io::stdin().lock().lines() {
        let Ok(line) = line else {
            eprintln!("stdin is not readable text");
            return ExitCode::FAILURE;
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (id, raw_params) = line.split_once(' ').unwrap_or((line, "{}"));
        match run_line(&mut session, id, raw_params).await {
            Ok(result) => println!("{result}"),
            Err(failure) => {
                eprintln!("{id}: {failure}");
                any_failed = true;
            }
        }
    }

    if any_failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

async fn run_line(session: &mut Session, id: &str, raw_params: &str) -> Result<Value, LineError> {
    let params: Value = serde_json::from_str(raw_params)?;
    let result = commands::dispatch(session, id, params).await?;
    Ok(result)
}
