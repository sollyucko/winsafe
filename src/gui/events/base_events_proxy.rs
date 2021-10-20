use std::ptr::NonNull;

use crate::aliases::ErrResult;
use crate::co;
use crate::gui::base::Base;
use crate::gui::events::events_wm_nfy::sealed_events_wm_nfy::SealedEventsWmNfy;
use crate::gui::traits::ParentEvents;
use crate::msg::wm;

/// Base to all native control event proxies.
pub(in crate::gui) struct BaseEventsProxy {
	parent_ptr: NonNull<Base>,
	ctrl_id: u16,
}

impl BaseEventsProxy {
	pub(in crate::gui) fn new(parent_base_ref: &Base, ctrl_id: u16) -> Self {
		Self {
			parent_ptr: NonNull::from(parent_base_ref),
			ctrl_id
		}
	}

	pub(in crate::gui) fn wm_command<F>(&self, code: co::CMD, func: F)
		where F: Fn() -> ErrResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().wm_command(code, self.ctrl_id, func);
	}

	pub(in crate::gui) fn add_nfy<F>(&self, code: co::NM, func: F)
		where F: Fn(wm::Notify) -> ErrResult<Option<isize>> + 'static
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().add_nfy(self.ctrl_id as _, code, func);
	}
}
