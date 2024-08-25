use crate::error::{Error, Result};
use std::{
    ffi::{CStr, CString},
    path::Path,
};

use wiredtiger_sys::{WT_CONNECTION, WT_CURSOR, WT_SESSION};

/// A connection to a WiredTiger database.
/// The connection may be opened within the same address space as the caller or accessed over a socket connection.
/// Most applications will open a single connection to a database for each process. The first process to open a connection to a database will access the database in its own address space. Subsequent connections (if allowed) will communicate with the first process over a socket connection to perform their operations.
/// Thread safety: A WT_CONNECTION handle may be shared between threads. See Multithreading for more information.
pub struct Connection {
    inner: *mut WT_CONNECTION,
}
unsafe impl Sync for Connection {}
unsafe impl Send for Connection {}

impl Connection {
    pub fn open(path: &Path) -> Result<Connection> {
        unsafe {
            let mut connection = std::ptr::null_mut();
            let home = CString::new(path.as_os_str().as_encoded_bytes())?;
            let config = c"create,statistics=(all)";
            let ret = wiredtiger_sys::wiredtiger_open(
                home.as_ptr(),
                std::ptr::null_mut(),
                config.as_ptr(),
                &mut connection,
            );
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(Connection { inner: connection })
        }
    }

    pub fn open_session(&mut self) -> Result<Session> {
        unsafe {
            let mut session: *mut WT_SESSION = std::ptr::null_mut();
            let Some(open_session) = (*self.inner).open_session else {
                return Err(Error::Placeholder(
                    "no open_session function pointer".to_owned(),
                ));
            };
            let ret = open_session(
                self.inner,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut session,
            );
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(Session { inner: session })
        }
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            let Some(close) = (*self.inner).close else {
                return;
            };
            let _ = close(self.inner, std::ptr::null());
        }
    }
}

/// All data operations are performed in the context of a WT_SESSION.
/// This encapsulates the thread and transactional context of the operation.
/// Thread safety: A WT_SESSION handle is not usually shared between threads, see Multithreading for more information.
pub struct Session {
    inner: *mut WT_SESSION,
}

impl Session {
    pub fn create_table(&mut self, table_name: &str) -> Result<()> {
        unsafe {
            let cmd = CString::new(format!("table:{table_name}"))?;
            let schema = c"key_format=S,value_format=S";
            let Some(create) = (*self.inner).create else {
                return Err(Error::Placeholder("no create function pointer".to_owned()));
            };
            let ret = create(self.inner, cmd.as_ptr(), schema.as_ptr());
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(())
        }
    }
    pub fn open_cursor(&mut self, table_name: &str) -> Result<Cursor> {
        unsafe {
            let mut cursor: *mut WT_CURSOR = std::ptr::null_mut();
            let cursor_uid = CString::new(format!("table:{table_name}"))?;
            let Some(open_cursor) = (*self.inner).open_cursor else {
                return Err(Error::Placeholder(
                    "no open_cursor function pointer".to_owned(),
                ));
            };
            let ret = open_cursor(
                self.inner,
                cursor_uid.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cursor,
            );
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(Cursor { inner: cursor })
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            let Some(close) = (*self.inner).close else {
                return;
            };
            let _ = close(self.inner, std::ptr::null());
        }
    }
}

/// Cursors allow data to be searched, iterated and modified, implementing the CRUD (create, read, update and delete) operations. Cursors are opened in the context of a session. If a transaction is started, cursors operate in the context of the transaction until the transaction is resolved.
/// Raw data is represented by key/value pairs of WT_ITEM structures, but cursors can also provide access to fields within the key and value if the formats are described in the WT_SESSION::create method.
/// In the common case, a cursor is used to access records in a table. However, cursors can be used on subsets of tables (such as a single column or a projection of multiple columns), as an interface to statistics, configuration data or application-specific data sources. See WT_SESSION::open_cursor for more information.
/// Thread safety: A WT_CURSOR handle is not usually shared between threads. See Multithreading for more information.
pub struct Cursor {
    inner: *mut WT_CURSOR,
}

impl Cursor {
    pub fn reset(&mut self) -> Result<()> {
        unsafe {
            let Some(reset) = (*self.inner).reset else {
                return Err(Error::Placeholder("no reset function pointer".to_owned()));
            };
            let ret = reset(self.inner);
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(())
        }
    }

    pub fn advance(&mut self) -> Result<()> {
        unsafe {
            let Some(next) = (*self.inner).next else {
                return Err(Error::Placeholder("no reset function pointer".to_owned()));
            };
            let ret = next(self.inner);
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(())
        }
    }

    /// # Safety
    /// Caller must ensure that the cursor has valid a key+value set.
    pub unsafe fn insert(&mut self) -> Result<()> {
        unsafe {
            let Some(insert) = (*self.inner).insert else {
                return Err(Error::Placeholder("no insert function pointer".to_owned()));
            };
            let ret = insert(self.inner);
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(())
        }
    }

    /// # Safety
    /// Caller must ensure that the cursor is in a valid state before invoking this method.
    /// Caller must ensure that the returned reference is dropped before mutating the cursor
    pub unsafe fn get_key(&mut self) -> Result<&CStr> {
        unsafe {
            let mut key = std::ptr::null();
            let Some(get_key) = (*self.inner).get_key else {
                return Err(Error::Placeholder("no get_key function pointer".to_owned()));
            };
            let ret = get_key(self.inner, &mut key);
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(CStr::from_ptr(key))
        }
    }

    /// # Safety
    /// Caller must ensure that the cursor is in a valid state before invoking this method.
    /// Caller must ensure that the returned reference is dropped before mutating the cursor
    pub unsafe fn get_value(&mut self) -> Result<&CStr> {
        unsafe {
            let mut value: *const i8 = std::ptr::null();
            let Some(get_value) = (*self.inner).get_value else {
                return Err(Error::Placeholder(
                    "no get_value function pointer".to_owned(),
                ));
            };
            let ret = get_value(self.inner, &mut value);
            if ret != 0 {
                return Err(Error::from_wt(ret));
            }
            Ok(CStr::from_ptr(value))
        }
    }

    /// # Safety
    /// Caller must ensure that `key` stays alive until the cursor is done using it
    pub unsafe fn set_key(&mut self, key: &CStr) -> Result<()> {
        unsafe {
            let Some(set_key) = (*self.inner).set_key else {
                return Err(Error::Placeholder("no set_key function pointer".to_owned()));
            };
            set_key(self.inner, key.as_ptr());
            Ok(())
        }
    }

    /// # Safety
    /// Caller must ensure that `value` stays alive until the cursor is done using it
    pub unsafe fn set_value(&mut self, value: &CStr) -> Result<()> {
        unsafe {
            let Some(set_value) = (*self.inner).set_value else {
                return Err(Error::Placeholder(
                    "no set_value function pointer".to_owned(),
                ));
            };
            set_value(self.inner, value.as_ptr());
            Ok(())
        }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe {
            let Some(close) = (*self.inner).close else {
                return;
            };
            let _ = close(self.inner);
        }
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
        assert!(!conn.inner.is_null());

        unsafe {
            let mut session = conn.open_session()?;
            session.create_table("foo")?;
            let mut cursor = session.open_cursor("foo")?;

            cursor.set_key(c"hello")?;
            cursor.set_value(c"world")?;
            cursor.insert()?;
        }

        unsafe {
            let mut session = conn.open_session()?;
            let mut cursor = session.open_cursor("foo")?;
            cursor.advance()?;
            assert_eq!(cursor.get_key()?, c"hello");
            assert_eq!(cursor.get_value()?, c"world");
        }
        Ok(())
    }
}
