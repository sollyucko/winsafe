#![allow(non_snake_case)]

use crate::aliases::WinResult;

use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;
use crate::structs::WIN32_FIND_DATA;
use crate::various::{Path, WString};

pub_struct_handle! {
	/// Handle to a
	/// [file search](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew).
	/// Originally just a `HANDLE`.
	HFINDFILE
}

impl HFINDFILE {
	/// [`FindClose`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// method.
	pub fn FindClose(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::FindClose(self.ptr) })
	}

	/// [`FindFirstFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFINDFILE::FindClose`](crate::HFINDFILE::FindClose) call.
	///
	/// This method is rather tricky, consider using
	/// [`HFINDFILE::iter`](crate::HFINDFILE::iter).
	pub fn FindFirstFile(
		file_name: &str,
		wfd: &mut WIN32_FIND_DATA) -> WinResult<(HFINDFILE, bool)>
	{
		match unsafe {
			kernel32::FindFirstFileW(
				WString::from_str(file_name).as_ptr(),
				wfd as *mut _ as _,
			).as_mut()
		} {
			Some(ptr) => Ok((Self { ptr }, true)), // first file found
			None => match GetLastError() {
				co::ERROR::FILE_NOT_FOUND => Ok((Self::NULL, false)), // not an error, first file not found
				err => Err(err),
			},
		}
	}

	/// [`FindNextFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findnextfilew)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`HFINDFILE::iter`](crate::HFINDFILE::iter).
	pub fn FindNextFile(self, wfd: &mut WIN32_FIND_DATA) -> WinResult<bool> {
		match unsafe {
			kernel32::FindNextFileW(self.ptr, wfd as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false), // not an error, no further files found
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// Returns an iterator over the found items, by calling
	/// [`FindFirstFile`](crate::HFINDFILE::FindFirstFile), then subsequent
	/// [`FindNextFile`](crate::HFINDFILE::FindNextFile), and finally freeing
	/// the resource by calling [`FindClose`](crate::HFINDFILE::FindClose).
	///
	/// # Examples
	///
	/// Enumerating all TXT files in a directory:
	///
	/// ```rust,ignore
	/// use winsafe::HFINDFILE;
	///
	/// for txt_file in HFINDFILE::iter("C:\\Temp\\*.txt") {
	///     let file = txt_file?;
	///     println!("File: {}", file);
	/// }
	/// ```
	pub fn iter<'a>(
		path_and_pattern: &'a str) -> impl Iterator<Item = WinResult<String>> + 'a
	{
		HfindfileIter::new(path_and_pattern)
	}
}

//------------------------------------------------------------------------------

struct HfindfileIter<'a> {
	hfind: HFINDFILE,
	first_pass: bool,
	wfd: WIN32_FIND_DATA,
	path_and_pattern: &'a str,
	no_more: bool,
}

impl<'a> Drop for HfindfileIter<'a> {
	fn drop(&mut self) {
		self.hfind.FindClose().ok(); // ignore error
	}
}

impl<'a> Iterator for HfindfileIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.no_more {
			return None;
		}

		let found = if self.first_pass {
			self.first_pass = false;
			let (hfind, found) =
				match HFINDFILE::FindFirstFile(self.path_and_pattern, &mut self.wfd) {
					Err(e) => {
						self.no_more = true; // prevent further iterations
						return Some(Err(e))
					},
					Ok((hfind, found)) => (hfind, found),
				};
			self.hfind = hfind;
			found
		} else {
			match self.hfind.FindNextFile(&mut self.wfd) {
				Err(e) => {
					self.no_more = true; // prevent further iterations
					return Some(Err(e))
				},
				Ok(found) => found,
			}
		};

		if found {
			let path_only = Path::path_from(self.path_and_pattern)
				.unwrap_or("");
			Some(Ok(format!("{}\\{}", path_only, self.wfd.cFileName())))
		} else {
			None
		}
	}
}

impl<'a> HfindfileIter<'a> {
	fn new(path_and_pattern: &'a str) -> Self {
		Self {
			hfind: HFINDFILE::NULL,
			first_pass: true,
			wfd: WIN32_FIND_DATA::default(),
			path_and_pattern,
			no_more: false,
		}
	}
}
