use core::cell::RefCell;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;

use crate::animations::AnimationSet;

pub const NUM_LEDS: usize = 64;

pub static STATE: Mutex<CriticalSectionRawMutex, RefCell<AnimationSet>> =
    Mutex::new(RefCell::new(AnimationSet::new()));
