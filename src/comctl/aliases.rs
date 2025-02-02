use crate::co;
use crate::user::decl::HWND;

/// Type alias to
/// [`PFTASKDIALOGCALLBACK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pftaskdialogcallback)
/// calback function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub type PFTASKDIALOGCALLBACK =
	extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		wParam: usize,
		lParam: isize,
		lpRefData: isize,
	);

/// Type alias to
/// [`PFNLVCOMPARE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-listview_sortitems)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub type PFNLVCOMPARE =
	extern "system" fn(
		lParam1: isize,
		lParam2: isize,
		lParamSort: isize,
	) -> i32;

/// Type alias to
/// [`PFNLVGROUPCOMPARE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pfnlvgroupcompare)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub type PFNLVGROUPCOMPARE =
	extern "system" fn(
		groupId1: i32,
		groupId2: i32,
		lpRefData: isize,
	) -> i32;

/// Type alias to
/// [`SUBCLASSPROC`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-subclassproc)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub type SUBCLASSPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
		uIdSubclass: usize,
		dwRefData: usize,
	) -> isize;
