use crate::co;
use crate::kernel::decl::{HIWORD, HLOCAL, LOWORD, WinResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{EDITWORDBREAKPROC, POINT, RECT, SIZE};
use crate::user::privs::{zero_as_err, zero_as_none};

/// [`EN_CANUNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/em-canundo)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct CanUndo {}

impl MsgSend for CanUndo {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CANUNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_CHARFROMPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-charfrompos)
/// message parameters.
///
/// Return type: `(u16, u16)`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct CharFromPos {
	pub coords: POINT,
}

impl MsgSend for CharFromPos {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CHARFROMPOS.into(),
			wparam: 0,
			lparam: self.coords.into_u32() as _,
		}
	}
}

pub_struct_msg_empty! { EmptyUndoBuffer: co::EM::EMPTYUNDOBUFFER.into(); "user";
	/// [`EM_EMPTYUNDOBUFFER`](https://docs.microsoft.com/en-us/windows/win32/controls/em-emptyundobuffer)
}

/// [`EM_FMTLINES`](https://docs.microsoft.com/en-us/windows/win32/controls/em-fmtlines)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct FmtLines {
	pub insert_soft_line_breaks: bool,
}

impl MsgSend for FmtLines {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::FMTLINES.into(),
			wparam: self.insert_soft_line_breaks as _,
			lparam: 0,
		}
	}
}

/// [`EM_GETFIRSTVISIBLELINE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getfirstvisibleline)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetFirstVisibleLine {}

impl MsgSend for GetFirstVisibleLine {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETFIRSTVISIBLELINE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETHANDLE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-gethandle)
/// message, which has no parameters.
///
/// Return type: `HLOCAL`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetHandle {}

impl MsgSend for GetHandle {
	type RetType = HLOCAL;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		HLOCAL(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETHANDLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETIMESTATUS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getimestatus)
/// message, which has no parameters.
///
/// Return type: `co::EIMES`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetImeStatus {}

impl MsgSend for GetImeStatus {
	type RetType = co::EIMES;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::EIMES(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: 0,
		}
	}
}

/// [`EM_GETLIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getlimittext)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLimitText {}

impl MsgSend for GetLimitText {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLIMITTEXT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETLINE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getline)
/// message parameters.
///
/// The message will retrieve at most `buffer.len() - 1` characters for the
/// line, because there must be room for a terminating null.
///
/// Returns the number of chars copied to `buffer`, not counting the terminating
/// null, or `None` if no chars were copied. There is no documented way to
/// differentiate between an error and an empty line.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLine<'a> {
	pub index: u16,
	pub buffer: &'a mut WString,
}

impl<'a> MsgSend for GetLine<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.buffer.fill_with_zero();
		let buf_len = self.buffer.buffer_size() - 1; // leave room for terminating null
		self.buffer.as_mut_slice()
			.iter_mut()
			.next()
			.map(|wchar| *wchar = buf_len as _); // leave room for terminating null

		WndMsg {
			msg_id: co::EM::GETLINE.into(),
			wparam: self.index as _,
			lparam: unsafe { self.buffer.as_mut_ptr() } as _,
		}
	}
}

/// [`EM_GETLINECOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getlinecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLineCount {}

impl MsgSend for GetLineCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLINECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETMARGINS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getmargins)
/// message, which has no parameters.
///
/// Return type: `SIZE`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetMargins {}

impl MsgSend for GetMargins {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from_u32(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETMARGINS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETMODIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getmodify)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetModify {}

impl MsgSend for GetModify {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETMODIFY.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETPASSWORDCHAR`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getpasswordchar)
/// message, which has no parameters.
///
/// Return type: `Option<char>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetPasswordChar {}

impl MsgSend for GetPasswordChar {
	type RetType = Option<char>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|c| unsafe { std::char::from_u32_unchecked(c as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETPASSWORDCHAR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`EM_GETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getsel)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetSel<'a, 'b> {
	pub first_index: Option<&'a mut u32>,
	pub past_last_index: Option<&'b mut u32>,
}

impl<'a, 'b> MsgSend for GetSel<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETSEL.into(),
			wparam: self.first_index.as_mut().map_or(0, |r| r as *mut _ as _),
			lparam: self.past_last_index.as_mut().map_or(0, |r| r as *mut _ as _),
		}
	}
}

/// [`EM_GETTHUMB`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getthumb)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetThumb {}

impl MsgSend for GetThumb {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETWORDBREAKPROC`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getwordbreakproc)
/// message, which has no parameters.
///
/// Return type: `Option<EDITWORDBREAKPROC>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetWordBreakProc {}

impl MsgSend for GetWordBreakProc {
	type RetType = Option<EDITWORDBREAKPROC>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { std::mem::transmute(p) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_LIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-limittext)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct LimitText {
	pub max: Option<u32>,
}

impl MsgSend for LimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LIMITTEXT.into(),
			wparam: self.max.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINEFROMCHAR`](https://docs.microsoft.com/en-us/windows/win32/controls/em-linefromchar)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct LineFromChar {
	pub char_index: Option<u32>,
}

impl MsgSend for LineFromChar {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINEFROMCHAR.into(),
			wparam: self.char_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINEINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/em-lineindex)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct LineIndex {
	pub line_index: Option<u32>,
}

impl MsgSend for LineIndex {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINEINDEX.into(),
			wparam: self.line_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINELENGTH`](https://docs.microsoft.com/en-us/windows/win32/controls/em-linelength)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct LineLength {
	pub char_index: Option<u32>,
}

impl MsgSend for LineLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINELENGTH.into(),
			wparam: self.char_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINESCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-linescroll)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct LineScroll {
	pub num_chars: u32,
	pub num_lines: u32,
}

impl MsgSend for LineScroll {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINESCROLL.into(),
			wparam: self.num_chars as _,
			lparam: self.num_lines as _,
		}
	}
}

/// [`EM_POSFROMCHAR`](https://docs.microsoft.com/en-us/windows/win32/controls/em-posfromchar)
/// message parameters.
///
/// Return type: `POINT`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct PosFromChar {
	pub char_index: u32,
}

impl MsgSend for PosFromChar {
	type RetType = POINT;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		POINT::from_u32(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::POSFROMCHAR.into(),
			wparam: self.char_index as _,
			lparam: 0,
		}
	}
}

/// [`EM_REPLACESEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-replacesel)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ReplaceSel {
	pub can_be_undone: bool,
	pub replacement_text: WString,
}

impl MsgSend for ReplaceSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::REPLACESEL.into(),
			wparam: self.can_be_undone as _,
			lparam: unsafe { self.replacement_text.as_ptr() } as _,
		}
	}
}

/// [`EM_SCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-scroll)
/// message parameters.
///
/// Return type: `WinResult<u16>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Scroll {
	pub action: co::SB_EM,
}

impl MsgSend for Scroll {
	type RetType = WinResult<u16>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|num_lines| num_lines as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SCROLL.into(),
			wparam: self.action.0 as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { ScrollCaret: co::EM::SCROLLCARET.into(); "user";
	/// [`EM_SCROLLCARET`](https://docs.microsoft.com/en-us/windows/win32/controls/em-scrollcaret)
}

/// [`EM_SETHANDLE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-sethandle)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetHandle {
	pub handle: HLOCAL,
}

impl MsgSend for SetHandle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETHANDLE.into(),
			wparam: self.handle.0 as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETIMESTATUS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setimestatus)
/// message parameters.
///
/// Return type: `co::EIMES`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetImeStatus {
	pub status: co::EIMES,
}

impl MsgSend for SetImeStatus {
	type RetType = co::EIMES;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::EIMES(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: self.status.0 as _,
		}
	}
}

/// [`EM_SETLIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setlimittext)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetLimitText {
	pub max_chars: Option<u32>,
}

impl MsgSend for SetLimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETLIMITTEXT.into(),
			wparam: self.max_chars.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETMARGINS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setmargins)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetMargins {
	pub margins: co::EC,
	pub size: SIZE,
}

impl MsgSend for SetMargins {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETMARGINS.into(),
			wparam: self.margins.0 as _,
			lparam: self.size.into_u32() as _,
		}
	}
}

/// [`EM_SETMODIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setmodify)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetModify {
	pub flag: bool,
}

impl MsgSend for SetModify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETMODIFY.into(),
			wparam: self.flag as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETPASSWORDCHAR`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setpasswordchar)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetPasswordChar {
	pub character: Option<char>,
}

impl MsgSend for SetPasswordChar {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETPASSWORDCHAR.into(),
			wparam: self.character.map(|ch| ch as u32).unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETREADONLY`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setreadonly)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetReadOnly {
	pub read_only: bool,
}

impl MsgSend for SetReadOnly {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETREADONLY.into(),
			wparam: self.read_only as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setrect)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetRect<'a> {
	pub is_absolute_coords: bool,
	pub rect: Option<&'a RECT>,
}

impl<'a> MsgSend for SetRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETRECT.into(),
			wparam: self.is_absolute_coords as _,
			lparam: self.rect.map_or(0, |rect| rect as *const _ as _),
		}
	}
}

/// [`EM_SETRECTNP`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setrectnp)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetRectNp<'a> {
	pub is_absolute_coords: bool,
	pub rect: Option<&'a RECT>,
}

impl<'a> MsgSend for SetRectNp<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETRECTNP.into(),
			wparam: self.is_absolute_coords as _,
			lparam: self.rect.map_or(0, |rect| rect as *const _ as _),
		}
	}
}

/// [`EM_SETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setsel)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetSel {
	pub start: Option<u32>,
	pub end: Option<u32>,
}

impl MsgSend for SetSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETSEL.into(),
			wparam: self.start.map_or(-1, |n| n as i32) as _,
			lparam: self.end.map_or(-1, |n| n as i32) as _,
		}
	}
}

/// [`EM_SETTABSTOPS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-settabstops)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetTabStops<'a> {
	pub tab_stops: Option<&'a [i32]>,
}

impl<'a> MsgSend for SetTabStops<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETTABSTOPS.into(),
			wparam: self.tab_stops.map_or(0, |ts| ts.len() as _),
			lparam: self.tab_stops.map_or(0, |ts| ts.as_ptr() as _),
		}
	}
}

/// [`EM_SETWORDBREAKPROC`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setwordbreakproc)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetWordBreakProc {
	pub proc: Option<EDITWORDBREAKPROC>,
}

impl MsgSend for SetWordBreakProc {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETWORDBREAKPROC.into(),
			wparam: 0,
			lparam: self.proc.map_or(0, |proc| proc as _),
		}
	}
}

/// [`EM_UNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/em-undo)
/// message, which has no parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Undo {}

impl MsgSend for Undo {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::UNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}