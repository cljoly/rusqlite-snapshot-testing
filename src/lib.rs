// SPDX-License-Identifier: MPL-2.0
// Copyright Clément Joly and contributors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// The doc is extracted from the README.md file at build time
#![doc = include_str!(concat!(env!("OUT_DIR"), "/readme_for_rustdoc.md"))]

use rusqlite::{params, Connection};

// Serialize for snapshotting
pub fn read_all_rows(conn: &Connection, table: &str) -> tabled::Table {
    use rusqlite::types::Value;
    use tabled::builder::Builder;
    use tabled::settings::{Panel, Style};

    let query = format!("SELECT * FROM {table}");
    let mut stmt = conn.prepare(&query).unwrap();
    let count = stmt.column_count();

    let mut table = Builder::default();
    table.push_record(
        stmt.column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
    stmt.query(params![])
        .unwrap()
        .mapped(|row| {
            Ok((0..count)
                .into_iter()
                .map(|i| format!("{:?}", row.get_unwrap::<_, Value>(i)))
                .collect::<Vec<_>>())
        })
        .for_each(|s| table.push_record(s.unwrap()));

    let mut table = table.build();
    table.with(Style::psql()).with(Panel::header(query));
    table
}

// TODO Whole database, only schema, data truncation, transform some of the data like insta

#[cfg(test)]
mod tests {
    use super::*;

    use rusqlite::Connection;

    #[test]
    fn basic_feature() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE friend (name, year_of_birth INTEGER);", ())
            .unwrap();

        insta::assert_snapshot!("empty_table", read_all_rows(&conn, "friend"));

        let mut stmt = conn
            .prepare("INSERT INTO friend (name, year_of_birth) VALUES (?1, ?2)")
            .unwrap();
        for (year, name) in &[
            ("alice", 1977),
            ("bob", 1987),
            ("charlie", 2000),
            ("daphne", 1950),
            ("eve", 1984),
        ] {
            stmt.execute((name, year)).unwrap();
        }
        insta::assert_snapshot!("with_data", read_all_rows(&conn, "friend"));
        // Insert a record of a different type, it’ll be captured in the snapshot
        stmt.execute((1337, 2013)).unwrap();
        insta::assert_snapshot!(
            "with_data_of_inconsistent_types",
            read_all_rows(&conn, "friend")
        );
    }
}
