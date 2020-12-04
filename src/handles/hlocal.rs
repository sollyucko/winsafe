#![allow(non_snake_case)]

use crate::co;
use crate::ffi::{kernel32, Void};

ty_handle!(HLOCAL,
	"Handle to a
	[local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).");

impl<T> From<*const T> for HLOCAL {
	fn from(p: *const T) -> Self {
		Self(p as *mut Void) // create from a *const T
	}
}

impl<T> From<*mut T> for HLOCAL {
	fn from(p: *mut T) -> Self {
		Self(p as *mut Void) // create from a *mut T
	}
}

impl HLOCAL {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	pub fn LocalFree(self) {
		if !unsafe { kernel32::LocalFree(self.0) }.is_null() {
			// We can't call FormatMessage() here because it uses LocalFree itself,
			// thus we may cause a stack overflow.
			panic!("LocalFree failed: error code {err} ({err:#010x}).",
				err = co::ERROR::GetLastError());
		}
	}
}