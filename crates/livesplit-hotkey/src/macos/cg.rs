#![allow(dead_code)]

use std::ffi::c_void;

use super::cf::MachPortRef;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum EventTapLocation {
    Hid,
    Session,
    AnnotatedSession,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum EventTapPlacement {
    HeadInsertEventTap,
    TailAppendEventTap,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum EventTapOptions {
    DefaultTap,
    ListenOnly,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct EventMask: u64 {
        /// Specifies a null event.
        const NULL = 1 << 0;
        /// Specifies a mouse down event with the left button.
        const LEFT_MOUSE_DOWN = 1 << 1;
        /// Specifies a mouse up event with the left button.
        const LEFT_MOUSE_UP = 1 << 2;
        /// Specifies a mouse down event with the right button.
        const RIGHT_MOUSE_DOWN = 1 << 3;
        /// Specifies a mouse up event with the right button.
        const RIGHT_MOUSE_UP = 1 << 4;
        /// Specifies a mouse moved event.
        const MOUSE_MOVED = 1 << 5;
        /// Specifies a mouse drag event with the left button down.
        const LEFT_MOUSE_DRAGGED = 1 << 6;
        /// Specifies a mouse drag event with the right button down.
        const RIGHT_MOUSE_DRAGGED = 1 << 7;
        /// Specifies a key down event.
        const KEY_DOWN = 1 << 10;
        /// Specifies a key up event.
        const KEY_UP = 1 << 11;
        /// Specifies a key changed event for a modifier or status key.
        const FLAGS_CHANGED = 1 << 12;
        /// Specifies a scroll wheel moved event.
        const SCROLL_WHEEL = 1 << 22;
        /// Specifies a tablet pointer event.
        const TABLET_POINTER = 1 << 23;
        /// Specifies a tablet proximity event.
        const TABLET_PROXIMITY = 1 << 24;
        /// Specifies a mouse down event with one of buttons 2-31.
        const OTHER_MOUSE_DOWN = 1 << 25;
        /// Specifies a mouse up event with one of buttons 2-31.
        const OTHER_MOUSE_UP = 1 << 26;
        /// Specifies a mouse drag event with one of buttons 2-31 down.
        const OTHER_MOUSE_DRAGGED = 1 << 27;

        const ALL_EVENTS = (1 << 28) - 1;
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum EventType {
    Null = 0,

    // Mouse events.
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    MouseMoved = 5,
    LeftMouseDragged = 6,
    RightMouseDragged = 7,

    // Keyboard events.
    KeyDown = 10,
    KeyUp = 11,
    FlagsChanged = 12,

    // Specialized control devices.
    ScrollWheel = 22,
    TabletPointer = 23,
    TabletProximity = 24,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    OtherMouseDragged = 27,

    // Out of band event types. These are delivered to the event tap callback
    // to notify it of unusual conditions that disable the event tap.
    TapDisabledByTimeout = 0xFFFFFFFE,
    TapDisabledByUserInput = 0xFFFFFFFF,
}

#[repr(u32)]
#[non_exhaustive]
pub enum EventField {
    MouseEventNumber = 0,
    MouseEventClickState = 1,
    MouseEventPressure = 2,
    MouseEventButtonNumber = 3,
    MouseEventDeltaX = 4,
    MouseEventDeltaY = 5,
    MouseEventInstantMouser = 6,
    MouseEventSubtype = 7,
    KeyboardEventAutorepeat = 8,
    KeyboardEventKeycode = 9,
    KeyboardEventKeyboardType = 10,
    ScrollWheelEventDeltaAxis1 = 11,
    ScrollWheelEventDeltaAxis2 = 12,
    ScrollWheelEventDeltaAxis3 = 13,
    ScrollWheelEventFixedPtDeltaAxis1 = 93,
    ScrollWheelEventFixedPtDeltaAxis2 = 94,
    ScrollWheelEventFixedPtDeltaAxis3 = 95,
    ScrollWheelEventPointDeltaAxis1 = 96,
    ScrollWheelEventPointDeltaAxis2 = 97,
    ScrollWheelEventPointDeltaAxis3 = 98,
    ScrollWheelEventScrollPhase = 99,
    ScrollWheelEventScrollCount = 100,
    ScrollWheelEventMomentumPhase = 123,
    ScrollWheelEventInstantMouser = 14,
    TabletEventPointX = 15,
    TabletEventPointY = 16,
    TabletEventPointZ = 17,
    TabletEventPointButtons = 18,
    TabletEventPointPressure = 19,
    TabletEventTiltX = 20,
    TabletEventTiltY = 21,
    TabletEventRotation = 22,
    TabletEventTangentialPressure = 23,
    TabletEventDeviceId = 24,
    TabletEventVendor1 = 25,
    TabletEventVendor2 = 26,
    TabletEventVendor3 = 27,
    TabletProximityEventVendorId = 28,
    TabletProximityEventTabletId = 29,
    TabletProximityEventPointerId = 30,
    TabletProximityEventDeviceId = 31,
    TabletProximityEventSystemTabletId = 32,
    TabletProximityEventVendorPointerType = 33,
    TabletProximityEventVendorPointerSerialNumber = 34,
    TabletProximityEventVendorUniqueId = 35,
    TabletProximityEventCapabilityMask = 36,
    TabletProximityEventPointerType = 37,
    TabletProximityEventEnterProximity = 38,
    EventTargetProcessSerialNumber = 39,
    EventTargetUnixProcessId = 40,
    EventSourceUnixProcessId = 41,
    EventSourceUserData = 42,
    EventSourceUserId = 43,
    EventSourceGroupId = 44,
    EventSourceStateId = 45,
    ScrollWheelEventIsContinuous = 88,
    MouseEventWindowUnderMousePointer = 91,
    MouseEventWindowUnderMousePointerThatCanHandleThisEvent = 92,
}

mod opaque {
    pub enum Event {}
    pub enum EventTapProxy {}
}

pub type EventRef = *mut opaque::Event;
pub type EventTapProxy = *mut opaque::EventTapProxy;

pub type EventTapCallBack = Option<
    unsafe extern "C" fn(
        proxy: EventTapProxy,
        ty: EventType,
        event: EventRef,
        userInfo: *mut c_void,
    ) -> EventRef,
>;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGEventTapCreate(
        tap: EventTapLocation,
        place: EventTapPlacement,
        options: EventTapOptions,
        events_of_interest: EventMask,
        callback: EventTapCallBack,
        userInfo: *mut c_void,
    ) -> MachPortRef;

    pub fn CGEventGetIntegerValueField(event: EventRef, field: EventField) -> i64;
}
