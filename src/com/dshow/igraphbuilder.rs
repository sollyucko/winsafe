#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::dshow::{IBaseFilter, IFilterGraph};
use crate::com::dshow::vt::{IBaseFilterVT, IFilterGraphVT, IGraphBuilderVT};
use crate::com::funcs::{hr_to_winresult, hr_to_winresult_bool};
use crate::com::PPComVT;
use crate::handles::HFILE;
use crate::WString;

/// [`IGraphBuilder`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-igraphbuilder)
/// COM interface. Backed by
/// [`IGraphBuilderVT`](crate::dshow::vt::IGraphBuilderVT) virtual table.
///
/// Inherits from:
/// * [`IFilterGraph`](crate::dshow::IFilterGraph);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, dshow};
///
/// let obj: dshow::IGraphBuilder = CoCreateInstance(
///     &dshow::clsid::FilterGraph,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
#[derive(Clone)]
pub struct IGraphBuilder {
	/// Methods of base interface [`IFilterGraph`](crate::dshow::IFilterGraph).
	pub IFilterGraph: IFilterGraph,
}

impl From<PPComVT<IGraphBuilderVT>> for IGraphBuilder {
	fn from(ppv: PPComVT<IGraphBuilderVT>) -> Self {
		Self {
			IFilterGraph: IFilterGraph::from(ppv as PPComVT<IFilterGraphVT>)
		}
	}
}

impl IGraphBuilder {
	unsafe fn ppv(&self) -> PPComVT<IGraphBuilderVT> {
		self.IFilterGraph.IUnknown.ppv::<IGraphBuilderVT>()
	}

	/// [`IGraphBuilder::Abort`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
	/// method.
	pub fn Abort(&self) -> WinResult<()> {
		hr_to_winresult(unsafe { ((**self.ppv()).Abort)(self.ppv()) })
	}

	/// [`IGraphBuilder::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
	/// method.
	pub fn AddSourceFilter(&self,
		fileName: &str, filterName: &str) -> WinResult<IBaseFilter>
	{
		let mut ppvQueried: PPComVT<IBaseFilterVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).AddSourceFilter)(
					self.ppv(),
					WString::from_str(fileName).as_ptr(),
					WString::from_str(filterName).as_ptr(),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IBaseFilter::from(ppvQueried))
	}

	/// [`IGraphBuilder::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
	/// method.
	pub fn RenderFile(&self, file: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).RenderFile)(
					self.ppv(),
					WString::from_str(file).as_ptr(),
					std::ptr::null(),
				)
			},
		)
	}

	/// [`IGraphBuilder::SetLogFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-setlogfile)
	/// method.
	pub fn SetLogFile(&self, hFile: Option<HFILE>) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetLogFile)(
					self.ppv(),
					match hFile {
						Some(hFile) => hFile.ptr,
						None => std::ptr::null_mut(),
					},
				)
			},
		)
	}

	/// [`IGraphBuilder::ShouldOperationContinue`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
	/// method.
	pub fn ShouldOperationContinue(&self) -> WinResult<bool> {
		hr_to_winresult_bool(
			unsafe { ((**self.ppv()).ShouldOperationContinue)(self.ppv()) },
		)
	}
}
