//! Dumps every redb table of a ChipFlow database file to JSON on stdout.
//!
//! Usage: `audit_dump <raw|connect> <path>`. `connect` first opens the file
//! through `Db::connect`, which applies this tree's startup upgrades, and
//! then dumps it; `raw` dumps the file as it is.

use std::collections::BTreeMap;

use redb::{ReadOnlyDatabase, ReadableDatabase, ReadableMultimapTable, ReadableTable};
use redb::{MultimapTableDefinition, TableDefinition, TableHandle, MultimapTableHandle};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn decode(bytes: &[u8]) -> Value {
    match serde_json::from_slice::<Value>(bytes) {
        Ok(value) => value,
        Err(_) => json!({
            "$bytes_sha256": format!("{:x}", Sha256::digest(bytes)),
            "$len": bytes.len(),
        }),
    }
}

fn dump(path: &str) -> Value {
    // A copy of a live file needs the repair a read-only open refuses.
    let read_only = std::env::var_os("AUDIT_REPAIR").is_none();
    let txn = match ReadOnlyDatabase::open(path).ok().filter(|_| read_only) {
        Some(db) => db.begin_read().expect("begin read"),
        None => redb::Database::open(path)
            .expect("open copy")
            .begin_read()
            .expect("begin read"),
    };
    let mut tables = BTreeMap::new();
    for handle in txn.list_tables().expect("list tables") {
        let name = handle.name().to_string();
        let mut rows = BTreeMap::new();
        let mut hashes = BTreeMap::new();
        let kind;
        if let Ok(tbl) = txn.open_table(TableDefinition::<&str, &[u8]>::new(&name)) {
            kind = "str->bytes";
            for item in tbl.iter().expect("iter") {
                let (k, v) = item.expect("row");
                hashes.insert(
                    k.value().to_string(),
                    format!("{:x}", Sha256::digest(v.value())),
                );
                rows.insert(k.value().to_string(), decode(v.value()));
            }
        } else if let Ok(tbl) = txn.open_table(TableDefinition::<&str, &str>::new(&name)) {
            kind = "str->str";
            for item in tbl.iter().expect("iter") {
                let (k, v) = item.expect("row");
                rows.insert(k.value().to_string(), json!(v.value()));
            }
        } else if let Ok(tbl) = txn.open_table(TableDefinition::<&str, u64>::new(&name)) {
            kind = "str->u64";
            for item in tbl.iter().expect("iter") {
                let (k, v) = item.expect("row");
                rows.insert(k.value().to_string(), json!(v.value()));
            }
        } else {
            kind = "unknown";
        }
        tables.insert(
            name,
            json!({"kind": kind, "count": rows.len(), "rows": rows, "sha": hashes}),
        );
    }
    for handle in txn.list_multimap_tables().expect("list multimaps") {
        let name = handle.name().to_string();
        let mut rows = BTreeMap::new();
        let mut kind = "unknown";
        let mut pairs = 0usize;
        if let Ok(tbl) = txn.open_multimap_table(MultimapTableDefinition::<&str, &str>::new(&name)) {
            kind = "multimap str->str";
            for item in tbl.iter().expect("iter") {
                let (k, values) = item.expect("row");
                let mut list: Vec<String> = values
                    .map(|v| v.expect("value").value().to_string())
                    .collect();
                list.sort();
                pairs += list.len();
                rows.insert(k.value().to_string(), json!(list));
            }
        }
        tables.insert(
            name,
            json!({"kind": kind, "count": rows.len(), "pairs": pairs, "rows": rows}),
        );
    }
    json!({ "tables": tables })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (mode, path) = (args[1].as_str(), args[2].as_str());
    if mode == "mutate" {
        mutate(path, &args[3]);
        return;
    }
    if mode == "connect" {
        let db = chipflow::db::Db::connect(path).expect("Db::connect");
        drop(db);
    }
    println!("{}", serde_json::to_string(&dump(path)).expect("serialize"));
}

/// Applies one deliberate change to a scratch copy, so a comparison must fail.
#[allow(dead_code)]
pub fn mutate(path: &str, how: &str) {
    let db = redb::Database::open(path).expect("open");
    let txn = db.begin_write().expect("write");
    {
        match how {
            "change" | "delete" => {
                let mut tbl = txn
                    .open_table(TableDefinition::<&str, &[u8]>::new("tasks"))
                    .expect("tasks");
                let (key, bytes) = {
                    let (k, v) = tbl.first().expect("first").expect("a task");
                    (k.value().to_string(), v.value().to_vec())
                };
                if how == "delete" {
                    tbl.remove(key.as_str()).expect("remove");
                } else {
                    let mut row: Value = serde_json::from_slice(&bytes).expect("json");
                    let position = row["position"].as_f64().expect("position");
                    row["position"] = json!(position + 1.0);
                    let out = serde_json::to_vec(&row).expect("bytes");
                    tbl.insert(key.as_str(), out.as_slice()).expect("insert");
                }
                eprintln!("mutated tasks row {key} ({how})");
            }
            _ => {
                let mut tbl = txn
                    .open_multimap_table(MultimapTableDefinition::<&str, &str>::new(
                        "tasks_by_column",
                    ))
                    .expect("index");
                let (key, value) = {
                    let (k, mut values) = tbl.iter().expect("iter").next().expect("a row").expect("row");
                    let v = values.next().expect("a value").expect("value");
                    (k.value().to_string(), v.value().to_string())
                };
                tbl.remove(key.as_str(), value.as_str()).expect("remove pair");
                eprintln!("removed tasks_by_column pair under {key}");
            }
        }
    }
    txn.commit().expect("commit");
}
