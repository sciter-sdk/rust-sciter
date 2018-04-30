//! C interface for behaviors support (a.k.a windowless controls).

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use capi::sctypes::*;
use capi::scdom::*;
use capi::scvalue::{VALUE};
use capi::scgraphics::{HGFX};

#[repr(C)]
pub struct BEHAVIOR_EVENT_PARAMS
{
	pub cmd: UINT,
	pub heTarget: HELEMENT,

	pub he: HELEMENT,
	pub reason: UINT_PTR,

	pub data:   VALUE,
}


#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum INITIALIZATION_EVENTS
{
	BEHAVIOR_DETACH = 0,
	BEHAVIOR_ATTACH = 1,
}

#[repr(C)]
pub struct INITIALIZATION_PARAMS
{
	pub cmd: INITIALIZATION_EVENTS,
}

#[repr(C)]
pub struct SCRIPTING_METHOD_PARAMS
{
	pub name: LPCSTR,
	pub argv: *const VALUE,
	pub argc: UINT,
	pub result: VALUE,
}

#[repr(C)]
pub struct TIMER_PARAMS
{
	pub timerId: UINT_PTR,
}

#[repr(C)]
pub struct DRAW_PARAMS {
	/// Element layer to draw.
	pub layer: DRAW_EVENTS,

	/// Graphics context.
	pub gfx: HGFX,

	/// Element area.
	pub area: RECT,

	/// Zero at the moment.
	pub reserved: UINT,
}

/// Layer to draw.
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
pub enum DRAW_EVENTS {
	DRAW_BACKGROUND = 0,
	DRAW_CONTENT,
	DRAW_FOREGROUND,
}


/// Event groups for subscription.
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum EVENT_GROUPS
{ /** attached/detached */
	HANDLE_INITIALIZATION = 0x0000,
	/** mouse events */
	HANDLE_MOUSE = 0x0001,
	/** key events */
	HANDLE_KEY = 0x0002,
	/** focus events, if this flag is set it also means that element it attached to is focusable */
	HANDLE_FOCUS = 0x0004,
	/** scroll events */
	HANDLE_SCROLL = 0x0008,
	/** timer event */
	HANDLE_TIMER = 0x0010,
	/** size changed event */
	HANDLE_SIZE = 0x0020,
	/** drawing request (event) */
	HANDLE_DRAW = 0x0040,
	/** requested data () has been delivered */
	HANDLE_DATA_ARRIVED = 0x080,

	 /** logical, synthetic events:
	                                           BUTTON_CLICK, HYPERLINK_CLICK, etc.,
	                                           a.k.a. notifications from intrinsic behaviors */
	HANDLE_BEHAVIOR_EVENT        = 0x0100,
	 /** behavior specific methods */
	HANDLE_METHOD_CALL           = 0x0200,
	/** behavior specific methods */
	HANDLE_SCRIPTING_METHOD_CALL = 0x0400,
	/** behavior specific methods using direct tiscript::value's */
	HANDLE_TISCRIPT_METHOD_CALL  = 0x0800,

	/** system drag-n-drop */
	HANDLE_EXCHANGE              = 0x1000,
	/** touch input events */
	HANDLE_GESTURE               = 0x2000,

	/** all of them */
	HANDLE_ALL                   = 0xFFFF,

	/** special value for getting subscription flags */
	SUBSCRIPTIONS_REQUEST        = -1,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Event propagation schema.
pub enum PHASE_MASK
{
	/// Bubbling phase – direction: from a child element to all its containers.
	BUBBLING 				= 0,
	/// Sinking phase – direction: from containers to target child element.
	SINKING  				= 0x0_8000,
	/// Bubbling event consumed by some child.
	BUBBLING_HANDLED= 0x1_0000,
	/// Sinking event consumed by some child.
	SINKING_HANDLED = 0x1_8000,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
#[allow(missing_docs)]
/// General event source triggers
pub enum CLICK_REASON
{
	BY_MOUSE_CLICK,
	BY_KEY_CLICK,
	SYNTHESIZED, // synthesized, programmatically generated.
	BY_MOUSE_ON_ICON,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Edit control change trigger.
pub enum EDIT_CHANGED_REASON
{
	/// Single char insertion.
	BY_INS_CHAR,
	/// Character range insertion, clipboard.
	BY_INS_CHARS,
	/// Single char deletion.
	BY_DEL_CHAR,
	/// character range deletion (selection).
	BY_DEL_CHARS,
	/// undo/redo
	BY_UNDO_REDO
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Behavior event codes.
pub enum BEHAVIOR_EVENTS
{
	/// click on button
	BUTTON_CLICK = 0,
	/// mouse down or key down in button
	BUTTON_PRESS,
	/// checkbox/radio/slider changed its state/value
	BUTTON_STATE_CHANGED,
	/// before text change
	EDIT_VALUE_CHANGING,
	/// after text change
	EDIT_VALUE_CHANGED,
	/// selection in `<select>` changed
	SELECT_SELECTION_CHANGED,
	/// node in select expanded/collapsed, heTarget is the node
	SELECT_STATE_CHANGED,

	/// request to show popup just received,
	///     here DOM of popup element can be modifed.
	POPUP_REQUEST,

	/// popup element has been measured and ready to be shown on screen,
	///     here you can use functions like `ScrollToView`.
	POPUP_READY,

	/// popup element is closed,
	///     here DOM of popup element can be modifed again - e.g. some items can be removed to free memory.
	POPUP_DISMISSED,

	/// menu item activated by mouse hover or by keyboard,
	MENU_ITEM_ACTIVE,

	/// menu item click,
	///   BEHAVIOR_EVENT_PARAMS structure layout
	///   BEHAVIOR_EVENT_PARAMS.cmd - MENU_ITEM_CLICK/MENU_ITEM_ACTIVE
	///   BEHAVIOR_EVENT_PARAMS.heTarget - owner(anchor) of the menu
	///   BEHAVIOR_EVENT_PARAMS.he - the menu item, presumably `<li>` element
	///   BEHAVIOR_EVENT_PARAMS.reason - BY_MOUSE_CLICK | BY_KEY_CLICK
	MENU_ITEM_CLICK,







	/// "right-click", BEHAVIOR_EVENT_PARAMS::he is current popup menu `HELEMENT` being processed or `NULL`.
	/// application can provide its own `HELEMENT` here (if it is `NULL`) or modify current menu element.
	CONTEXT_MENU_REQUEST = 0x10,


	/// broadcast notification, sent to all elements of some container being shown or hidden
	VISIUAL_STATUS_CHANGED,
	/// broadcast notification, sent to all elements of some container that got new value of `:disabled` state
	DISABLED_STATUS_CHANGED,

	/// popup is about to be closed
	POPUP_DISMISSING,

	/// content has been changed, is posted to the element that gets content changed,  reason is combination of `CONTENT_CHANGE_BITS`.
	/// `target == NULL` means the window got new document and this event is dispatched only to the window.
	CONTENT_CHANGED = 0x15,


	/// generic click
	CLICK = 0x16,
	/// generic change
	CHANGE = 0x17,

	// "grey" event codes  - notfications from behaviors from this SDK
	/// hyperlink click
	HYPERLINK_CLICK = 0x80,

	/// element was collapsed, so far only behavior:tabs is sending these two to the panels
	ELEMENT_COLLAPSED = 0x90,
	/// element was expanded,
	ELEMENT_EXPANDED,

	/// activate (select) child,
	/// used for example by accesskeys behaviors to send activation request, e.g. tab on `behavior:tabs`.
	ACTIVATE_CHILD,

	/// ui state changed, observers shall update their visual states.
	/// is sent for example by `behavior:richtext` when caret position/selection has changed.
	UI_STATE_CHANGED = 0x95,


	/// `behavior:form` detected submission event. `BEHAVIOR_EVENT_PARAMS::data` field contains data to be posted.
	/// `BEHAVIOR_EVENT_PARAMS::data` is of type `T_MAP` in this case key/value pairs of data that is about
	/// to be submitted. You can modify the data or discard submission by returning true from the handler.
	FORM_SUBMIT,


	/// `behavior:form` detected reset event (from `button type=reset`). `BEHAVIOR_EVENT_PARAMS::data` field contains data to be reset.
	/// `BEHAVIOR_EVENT_PARAMS::data` is of type `T_MAP` in this case key/value pairs of data that is about
	/// to be rest. You can modify the data or discard reset by returning true from the handler.
	FORM_RESET,



	/// document in `behavior:frame` or root document is complete.
	DOCUMENT_COMPLETE,

	/// requests to `behavior:history` (commands)
	HISTORY_PUSH,
	HISTORY_DROP,
	HISTORY_PRIOR,
	HISTORY_NEXT,
	/// `behavior:history` notification - history stack has changed
	HISTORY_STATE_CHANGED,

	/// close popup request,
	CLOSE_POPUP,
	/// request tooltip, `evt.source` <- is the tooltip element.
	REQUEST_TOOLTIP,

	/// animation started (`reason=1`) or ended(`reason=0`) on the element.
	ANIMATION         = 0xA0,

	/// document created, script namespace initialized. `target` -> the document
	DOCUMENT_CREATED  = 0xC0,
	/// document is about to be closed, to cancel closing do: `evt.data = sciter::value("cancel")`;
	DOCUMENT_CLOSE_REQUEST,
	/// last notification before document removal from the DOM
	DOCUMENT_CLOSE,
	/// document has got DOM structure, styles and behaviors of DOM elements. Script loading run is complete at this moment.
	DOCUMENT_READY,
	/// document just finished parsing - has got DOM structure. This event is generated before the `DOCUMENT_READY`.
	/// Since 4.0.3.
	DOCUMENT_PARSED   = 0xC4,

	/// `<video>` "ready" notification
	VIDEO_INITIALIZED = 0xD1,
	/// `<video>` playback started notification
	VIDEO_STARTED,
	/// `<video>` playback stoped/paused notification
	VIDEO_STOPPED,
	/// `<video>` request for frame source binding,
	///   If you want to provide your own video frames source for the given target `<video>` element do the following:
	///
	///   1. Handle and consume this `VIDEO_BIND_RQ` request
	///   2. You will receive second `VIDEO_BIND_RQ` request/event for the same `<video>` element
	///      but this time with the `reason` field set to an instance of `sciter::video_destination` interface.
	///   3. `add_ref()` it and store it for example in worker thread producing video frames.
	///   4. call `sciter::video_destination::start_streaming(...)` providing needed parameters
	///      call `sciter::video_destination::render_frame(...)` as soon as they are available
	///      call `sciter::video_destination::stop_streaming()` to stop the rendering (a.k.a. end of movie reached)
	VIDEO_BIND_RQ,


	/// `behavior:pager` starts pagination
	PAGINATION_STARTS  = 0xE0,
	/// `behavior:pager` paginated page no, reason -> page no
	PAGINATION_PAGE,
	/// `behavior:pager` end pagination, reason -> total pages
	PAGINATION_ENDS,

	/// all custom event codes shall be greater than this number. All codes below this will be used
	/// solely by application - Sciter will not intrepret it and will do just dispatching.
	/// To send event notifications with  these codes use `SciterSend`/`PostEvent` API.
	FIRST_APPLICATION_EVENT_CODE = 0x100,

}


impl ::std::ops::BitOr for EVENT_GROUPS {
  type Output = EVENT_GROUPS;
  fn bitor(self, rhs: Self::Output) -> Self::Output {
    let rn = (self as UINT) | (rhs as UINT);
    unsafe { ::std::mem::transmute(rn) }
  }
}
