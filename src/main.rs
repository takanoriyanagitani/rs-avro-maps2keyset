use std::collections::BTreeSet;
use std::io;
use std::process::ExitCode;

use rs_avro_maps2keyset::avro2rows2keyset::stdin2keyset;

fn stdin2avro2keyset() -> Result<BTreeSet<String>, io::Error> {
    stdin2keyset()
}

fn set2stdout(s: BTreeSet<String>) -> Result<(), io::Error> {
    for key in s.iter() {
        println!("{key}");
    }
    Ok(())
}

fn stdin2avro2keyset2stdout() -> Result<(), io::Error> {
    let keyset: BTreeSet<String> = stdin2avro2keyset()?;
    set2stdout(keyset)
}

fn sub() -> Result<(), io::Error> {
    stdin2avro2keyset2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
