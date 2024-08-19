use std::{ffi::CString, path::PathBuf, str::FromStr};

use wiredtiger_rs::Connection;

fn main() -> anyhow::Result<()> {
    let dir = PathBuf::from_str("/tmp/wt-simple")?;
    let table = "my_table";
    let mut conn = Connection::open(&dir)?;

    {
        let mut session = conn.open_session()?;
        session.create_table(&table)?;
        let mut cursor = session.open_cursor(&table)?;
        cursor.put(&CString::new("hello")?, &CString::new("world")?)?;
    }

    {
        let mut session = conn.open_session()?;
        let mut cursor = session.open_cursor(&table)?;
        while cursor.advance().is_ok() {
            println!("{:?} -> {:?}", cursor.get_key()?, cursor.get_value()?,);
        }
    }
    Ok(())
}
