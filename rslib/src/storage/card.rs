// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use crate::cached_sql;
use crate::card::{Card, CardID, CardQueue, CardType};
use crate::err::Result;
use rusqlite::params;
use rusqlite::{
    types::{FromSql, FromSqlError, ValueRef},
    OptionalExtension,
};
use std::convert::TryFrom;

impl FromSql for CardType {
    fn column_result(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        if let ValueRef::Integer(i) = value {
            Ok(Self::try_from(i as u8).map_err(|_| FromSqlError::InvalidType)?)
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}

impl FromSql for CardQueue {
    fn column_result(value: ValueRef<'_>) -> std::result::Result<Self, FromSqlError> {
        if let ValueRef::Integer(i) = value {
            Ok(Self::try_from(i as i8).map_err(|_| FromSqlError::InvalidType)?)
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}

impl super::StorageContext<'_> {
    pub fn get_card(&mut self, cid: CardID) -> Result<Option<Card>> {
        // the casts are required as Anki didn't prevent add-ons from
        // storing strings or floats in columns before
        let stmt = cached_sql!(
            self.get_card_stmt,
            self.db,
            "
select nid, did, ord, cast(mod as integer), usn, type, queue, due,
cast(ivl as integer), factor, reps, lapses, left, odue, odid,
flags, data from cards where id=?"
        );

        stmt.query_row(params![cid], |row| {
            Ok(Card {
                id: cid,
                nid: row.get(0)?,
                did: row.get(1)?,
                ord: row.get(2)?,
                mtime: row.get(3)?,
                usn: row.get(4)?,
                ctype: row.get(5)?,
                queue: row.get(6)?,
                due: row.get(7)?,
                ivl: row.get(8)?,
                factor: row.get(9)?,
                reps: row.get(10)?,
                lapses: row.get(11)?,
                left: row.get(12)?,
                odue: row.get(13)?,
                odid: row.get(14)?,
                flags: row.get(15)?,
                data: row.get(16)?,
            })
        })
        .optional()
        .map_err(Into::into)
    }
}
