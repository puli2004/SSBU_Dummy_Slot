#![feature(proc_macro_hygiene)]
use skyline::{install_hook};

#[derive(Debug)]
#[repr(C)]
pub struct PlayerInfo {
    unk1: [u8; 80],
    ice_climber_going_first: u32,
    unk2: u32,
    fighter_id: u32,
    redirected_fighter_id: u32,
    unk3: u32,
    fighter_slot: u32,
    unk4: [u8; 127],
    FIGHTER_NAME: u64,
    unk5: [u8;9],
}

//(*param_1).fighter_slot = 7; // Slots model

#[skyline::hook(offset = 0x17e44c0)]
unsafe fn set_fighter_color(param_1: *mut PlayerInfo) {
    original!()(param_1);
    let slot = (*param_1).fighter_slot as u8;
    let fighter_id = (*param_1).fighter_id as i32;
        if (*param_1).fighter_id == 15 && hash40(0x0edd2afecc) == true{
            println!("Passed test 1");
            if (*param_1).fighter_slot == 7{
            (*param_1).fighter_slot = 2;
            println!("Switching");
        }
    }
  }

#[skyline::main(name = "color-switch")]
pub fn main() {
    install_hook!(set_fighter_color);
    /*install_hook!(set_chara_color);*/
}
