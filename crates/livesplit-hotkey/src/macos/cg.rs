#![allow(dead_code)]

use std::ffi::c_void;

use super::cf::MachPortRef;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct EventTapLocation(u32);

impl EventTapLocation {
    pub const HID: Self = Self(0);
    pub const SESSION: Self = Self(1);
    pub const ANNOTATED_SESSION: Self = Self(2);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct EventTapPlacement(u32);

impl EventTapPlacement {
    pub const HEAD_INSERT_EVENT_TAP: Self = Self(0);
    pub const TAIL_APPEND_EVENT_TAP: Self = Self(1);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct EventTapOptions(u32);

impl EventTapOptions {
    pub const DEFAULT_TAP: Self = Self(0);
    pub const LISTEN_ONLY: Self = Self(1);
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventType(u32);

impl EventType {
    pub const NULL: Self = Self(0);

    // Mouse events.
    pub const LEFT_MOUSE_DOWN: Self = Self(1);
    pub const LEFT_MOUSE_UP: Self = Self(2);
    pub const RIGHT_MOUSE_DOWN: Self = Self(3);
    pub const RIGHT_MOUSE_UP: Self = Self(4);
    pub const MOUSE_MOVED: Self = Self(5);
    pub const LEFT_MOUSE_DRAGGED: Self = Self(6);
    pub const RIGHT_MOUSE_DRAGGED: Self = Self(7);

    // Keyboard events.
    pub const KEY_DOWN: Self = Self(10);
    pub const KEY_UP: Self = Self(11);
    pub const FLAGS_CHANGED: Self = Self(12);

    // Specialized control devices.
    pub const SCROLL_WHEEL: Self = Self(22);
    pub const TABLET_POINTER: Self = Self(23);
    pub const TABLET_PROXIMITY: Self = Self(24);
    pub const OTHER_MOUSE_DOWN: Self = Self(25);
    pub const OTHER_MOUSE_UP: Self = Self(26);
    pub const OTHER_MOUSE_DRAGGED: Self = Self(27);

    // Out of band event types. These are delivered to the event tap callback
    // to notify it of unusual conditions that disable the event tap.
    pub const TAP_DISABLED_BY_TIMEOUT: Self = Self(0xFFFFFFFE);
    pub const TAP_DISABLED_BY_USER_INPUT: Self = Self(0xFFFFFFFF);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventField(u32);

impl EventField {
    pub const MOUSE_EVENT_NUMBER: Self = Self(0);
    pub const MOUSE_EVENT_CLICK_STATE: Self = Self(1);
    pub const MOUSE_EVENT_PRESSURE: Self = Self(2);
    pub const MOUSE_EVENT_BUTTON_NUMBER: Self = Self(3);
    pub const MOUSE_EVENT_DELTA_X: Self = Self(4);
    pub const MOUSE_EVENT_DELTA_Y: Self = Self(5);
    pub const MOUSE_EVENT_INSTANT_MOUSER: Self = Self(6);
    pub const MOUSE_EVENT_SUBTYPE: Self = Self(7);
    pub const KEYBOARD_EVENT_AUTOREPEAT: Self = Self(8);
    pub const KEYBOARD_EVENT_KEYCODE: Self = Self(9);
    pub const KEYBOARD_EVENT_KEYBOARD_TYPE: Self = Self(10);
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS1: Self = Self(11);
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS2: Self = Self(12);
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS3: Self = Self(13);
    pub const SCROLL_WHEEL_EVENT_FIXED_PT_DELTA_AXIS1: Self = Self(93);
    pub const SCROLL_WHEEL_EVENT_FIXED_PT_DELTA_AXIS2: Self = Self(94);
    pub const SCROLL_WHEEL_EVENT_FIXED_PT_DELTA_AXIS3: Self = Self(95);
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS1: Self = Self(96);
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS2: Self = Self(97);
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS3: Self = Self(98);
    pub const SCROLL_WHEEL_EVENT_SCROLL_PHASE: Self = Self(99);
    pub const SCROLL_WHEEL_EVENT_SCROLL_COUNT: Self = Self(100);
    pub const SCROLL_WHEEL_EVENT_MOMENTUM_PHASE: Self = Self(123);
    pub const SCROLL_WHEEL_EVENT_INSTANT_MOUSER: Self = Self(14);
    pub const TABLET_EVENT_POINT_X: Self = Self(15);
    pub const TABLET_EVENT_POINT_Y: Self = Self(16);
    pub const TABLET_EVENT_POINT_Z: Self = Self(17);
    pub const TABLET_EVENT_POINT_BUTTONS: Self = Self(18);
    pub const TABLET_EVENT_POINT_PRESSURE: Self = Self(19);
    pub const TABLET_EVENT_TILT_X: Self = Self(20);
    pub const TABLET_EVENT_TILT_Y: Self = Self(21);
    pub const TABLET_EVENT_ROTATION: Self = Self(22);
    pub const TABLET_EVENT_TANGENTIAL_PRESSURE: Self = Self(23);
    pub const TABLET_EVENT_DEVICE_ID: Self = Self(24);
    pub const TABLET_EVENT_VENDOR1: Self = Self(25);
    pub const TABLET_EVENT_VENDOR2: Self = Self(26);
    pub const TABLET_EVENT_VENDOR3: Self = Self(27);
    pub const TABLET_PROXIMITY_EVENT_VENDOR_ID: Self = Self(28);
    pub const TABLET_PROXIMITY_EVENT_TABLET_ID: Self = Self(29);
    pub const TABLET_PROXIMITY_EVENT_POINTER_ID: Self = Self(30);
    pub const TABLET_PROXIMITY_EVENT_DEVICE_ID: Self = Self(31);
    pub const TABLET_PROXIMITY_EVENT_SYSTEM_TABLET_ID: Self = Self(32);
    pub const TABLET_PROXIMITY_EVENT_VENDOR_POINTER_TYPE: Self = Self(33);
    pub const TABLET_PROXIMITY_EVENT_VENDOR_POINTER_SERIAL_NUMBER: Self = Self(34);
    pub const TABLET_PROXIMITY_EVENT_VENDOR_UNIQUE_ID: Self = Self(35);
    pub const TABLET_PROXIMITY_EVENT_CAPABILITY_MASK: Self = Self(36);
    pub const TABLET_PROXIMITY_EVENT_POINTER_TYPE: Self = Self(37);
    pub const TABLET_PROXIMITY_EVENT_ENTER_PROXIMITY: Self = Self(38);
    pub const EVENT_TARGET_PROCESS_SERIAL_NUMBER: Self = Self(39);
    pub const EVENT_TARGET_UNIX_PROCESS_ID: Self = Self(40);
    pub const EVENT_SOURCE_UNIX_PROCESS_ID: Self = Self(41);
    pub const EVENT_SOURCE_USER_DATA: Self = Self(42);
    pub const EVENT_SOURCE_USER_ID: Self = Self(43);
    pub const EVENT_SOURCE_GROUP_ID: Self = Self(44);
    pub const EVENT_SOURCE_STATE_ID: Self = Self(45);
    pub const SCROLL_WHEEL_EVENT_IS_CONTINUOUS: Self = Self(88);
    pub const MOUSE_EVENT_WINDOW_UNDER_MOUSE_POINTER: Self = Self(91);
    pub const MOUSE_EVENT_WINDOW_UNDER_MOUSE_POINTER_THAT_CAN_HANDLE_THIS_EVENT: Self = Self(92);
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

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct EventFlags: u64 {
        const CAPS_LOCK = 1 << 16;
        const SHIFT = 1 << 17;
        const CONTROL = 1 << 18;
        const OPTION = 1 << 19;
        const COMMAND = 1 << 20;
        const NUMERIC_PAD = 1 << 21;
        const HELP = 1 << 22;
        const FUNCTION = 1 << 23;
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    pub fn CGEventGetFlags(event: EventRef) -> EventFlags;

    pub fn CGEventTapCreate(
        tap: EventTapLocation,
        place: EventTapPlacement,
        options: EventTapOptions,
        events_of_interest: EventMask,
        callback: EventTapCallBack,
        userInfo: *mut c_void,
    ) -> MachPortRef;

    pub fn CGEventTapEnable(tap: MachPortRef, enable: bool);

    pub fn CGEventGetIntegerValueField(event: EventRef, field: EventField) -> i64;
}
