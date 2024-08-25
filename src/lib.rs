use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    path::Path,
};

pub mod error;

// Low level wrappers around the raw C API, but without any safety guarantees.
pub mod raw;

pub use error::{Error, Result};

pub struct Connection {
    inner: raw::Connection,
}

impl Connection {
    pub fn open(path: &Path) -> Result<Connection> {
        Ok(Connection {
            inner: raw::Connection::open(path)?,
        })
    }

    pub fn open_session(&mut self) -> Result<Session> {
        Ok(Session {
            inner: self.inner.open_session()?,
            _lifetime: &PhantomData,
        })
    }
}

/// All data operations are performed in the context of a WT_SESSION.
/// This encapsulates the thread and transactional context of the operation.
/// Thread safety: A WT_SESSION handle is not usually shared between threads, see Multithreading for more information.
pub struct Session<'a> {
    inner: raw::Session,
    _lifetime: &'a PhantomData<()>,
}

impl<'a> Session<'a> {
    pub fn create_table(&mut self, name: &str) -> Result<()> {
        self.inner.create_table(name)
    }
    pub fn open_cursor(&mut self, table_name: &str) -> Result<Cursor> {
        Ok(Cursor {
            inner: self.inner.open_cursor(table_name)?,
            _lifetime: &PhantomData,
        })
    }
}

/// Cursor is a safe wrapper around raw::Cursor. It has appropriate lifetimes and safe methods
pub struct Cursor<'a> {
    inner: raw::Cursor,
    _lifetime: &'a PhantomData<()>,
}

impl<'a> Cursor<'a> {
    pub fn reset(&'a mut self) -> Result<()> {
        self.inner.reset()
    }

    pub fn advance(&mut self) -> Result<()> {
        self.inner.advance()
    }

    pub fn get_key(&mut self) -> Result<CString> {
        let v = unsafe { self.inner.get_key()? };
        Ok(v.to_owned())
    }

    pub fn get_value(&mut self) -> Result<CString> {
        let v = unsafe { self.inner.get_value()? };
        Ok(v.to_owned())
    }

    pub fn put(&mut self, key: &CStr, value: &CStr) -> Result<()> {
        unsafe {
            self.inner.set_key(key)?;
            self.inner.set_value(value)?;
            self.inner.insert()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_check() {
        // If this test fails, you probably need to bump the crate version too.
        assert_eq!(
            [
                wiredtiger_sys::WIREDTIGER_VERSION_MAJOR,
                wiredtiger_sys::WIREDTIGER_VERSION_MINOR,
                wiredtiger_sys::WIREDTIGER_VERSION_PATCH
            ],
            [10, 0, 0]
        );
    }

    #[test]
    fn it_works() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let mut conn = Connection::open(dir.path())?;

        {
            let mut session = conn.open_session()?;
            session.create_table("foo")?;
            let mut cursor = session.open_cursor("foo")?;
            cursor.put(c"hello", c"world")?;
        }

        let mut session = conn.open_session()?;
        let mut cursor = session.open_cursor("foo")?;
        cursor.advance()?;
        assert_eq!(cursor.get_key()?, c"hello".into());
        assert_eq!(cursor.get_value()?, c"world".into());
        Ok(())
    }
}
