use std::io;
use std::io::BufReader;
use std::io::Read;

use std::collections::BTreeSet;
use std::collections::HashMap;

use apache_avro::types::Value;
use apache_avro::Reader;

pub fn map2keyset(m: HashMap<String, Value>, s: &mut BTreeSet<String>) {
    for key in m.into_keys() {
        s.insert(key);
    }
}

pub fn row2keyset(row: Vec<(String, Value)>, s: &mut BTreeSet<String>) {
    for pair in row.into_iter() {
        let (key, _) = pair;
        s.insert(key);
    }
}

pub fn val2keyset(expected2b_record: Value, s: &mut BTreeSet<String>) {
    match expected2b_record {
        Value::Record(r) => row2keyset(r, s),
        Value::Map(m) => map2keyset(m, s),
        _ => {}
    }
}

pub fn values2keyset<I>(values: I, s: &mut BTreeSet<String>) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Value, io::Error>>,
{
    for val in values {
        val2keyset(val?, s);
    }
    Ok(())
}

pub fn reader2keyset<R>(rdr: R, s: &mut BTreeSet<String>) -> Result<(), io::Error>
where
    R: Read,
{
    let br = BufReader::new(rdr);
    let ar: Reader<_> = Reader::new(br).map_err(io::Error::other)?;
    let mapd = ar.map(|r| r.map_err(io::Error::other));
    values2keyset(mapd, s)
}

pub fn stdin2keyset() -> Result<BTreeSet<String>, io::Error> {
    let mut buf: BTreeSet<String> = BTreeSet::new();
    let i = io::stdin();
    let il = i.lock();
    reader2keyset(il, &mut buf)?;
    Ok(buf)
}
