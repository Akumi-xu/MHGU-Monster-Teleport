#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use forge::input::{Button, Input};
use forge::log::prelude::*;

static BASE: AtomicU32 = AtomicU32::new(0);
static HELD: AtomicBool = AtomicBool::new(false);

fn valid(p: u32) -> bool { (0x08000000..0xa8000000).contains(&p) && p & 3 == 0 }
unsafe fn read<T: Copy>(a: u32) -> T { core::ptr::read_volatile(a as *const T) }
unsafe fn write<T>(a: u32, value: T) { core::ptr::write_volatile(a as *mut T, value); }

// Main-relative guest offsets, not host process addresses.
unsafe fn teleport(base: u32) {
    let net = read::<u32>(base + 0x18843f0);
    let players = read::<u32>(base + 0x1885b58);
    let area = read::<u32>(base + 0x188ca48);
    let stage = read::<u32>(base + 0x18977a0);
    let quest = read::<u32>(base + 0x188ad90);
    if ![net, players, area, stage, quest].iter().all(|&p| valid(p)) { return; }
    // Offline-solo gate shared with the companion mods.
    if read::<u8>(net + 1598) != 0 || read::<u8>(players + 40028) > 1
        || (0..4).filter(|i| valid(read::<u32>(players + 28 + i * 4))).count() != 1
        || read::<u32>(quest) != base + 0x1753b30 { return; }
    let in_quest: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(base + 0x3f8634);
    let finished: unsafe extern "C" fn(u32, u32, u32) -> u32 = core::mem::transmute(base + 0x3a5ed0);
    if in_quest(area) != 1 || finished(quest, 5, 1) != 0 || read::<u8>(quest + 0x55) > 12 { return; }
    let slot = read::<u32>(area + 0x30);
    if slot >= 4 { return; }
    let player = read::<u32>(players + 28 + 4 * slot);
    if !valid(player) || read::<i16>(player + 0x13d6) <= 0 { return; }
    let current = read::<u8>(player + 0x1054);
    let pending = read::<u8>(area + 0x4a);
    if current > 12 || (pending != 255 && pending != current) || read::<u32>(stage + 420) & 2 != 0 { return; }
    // Ground movement only: do not interrupt carts, climbing or attacks.
    if !matches!(read::<u16>(player + 0x2506), 1 | 2) { return; }
    let enemies = read::<u32>(base + 0x1851fe0);
    if !valid(enemies) { return; }
    for slot in 0..10 {
        let enemy = read::<u32>(enemies + 28 + 4 * slot);
        if !valid(enemy) || read::<i32>(enemy + 0x17b0) <= 0
            || read::<u32>(enemy + 0x17b4) <= 100 || read::<u8>(enemy + 0x15ea) == 0
            || !matches!(read::<u8>(enemy + 0xd), 0x44 | 0x4c) { continue; }
        let target = read::<u8>(enemy + 0x1054);
        if target > 12 { continue; }
        if target == current {
            info!("[teleport] already in monster slot {} area {}", slot, current);
            return;
        }
        let data = read::<u32>(stage + 904 + 4 * target as u32);
        if !valid(data) || read::<u32>(data + 0xcc) == 0 { continue; }
        let points = read::<u32>(data + 0xd8);
        if !valid(points) { continue; }
        let point = read::<u32>(points);
        if !valid(point) { continue; }
        let spawn = read::<u8>(point + 0x20);
        // Native checkAreaMove request; the game loads the area and authored entrance.
        write(area + 0x4a, target);
        write(area + 0x51, spawn);
        write(stage + 420, read::<u32>(stage + 420) | 2);
        info!("[teleport] request monster slot {}: area {} -> {}, spawn {}", slot, current, target, spawn);
        return;
    }
    info!("[teleport] no living large monster with a loaded destination");
}

#[forge::entry]
fn main() {
    BASE.store(forge::mem::text_addr(), Relaxed);
    info!("[teleport] v0.1.0 offline solo; L+R+UP to first living large monster area");
}

#[unsafe(no_mangle)]
pub extern "C" fn forge_onUpdate(_dt: f32) {
    let down = Input::is_button_down(Button::L) && Input::is_button_down(Button::R)
        && Input::is_button_down(Button::Up);
    let was = HELD.swap(down, Relaxed);
    let base = BASE.load(Relaxed);
    if base != 0 && down && !was { unsafe { teleport(base); } }
}
