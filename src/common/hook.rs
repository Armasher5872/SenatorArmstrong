use super::*;

#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::from_offset(0x68d530)]
pub fn get_fighter_vtable(id: u32) -> *const usize;

//Calls the Special Zoom function
#[skyline::from_offset(0x696720)]
pub unsafe extern "C" fn call_special_zoom(boma: *mut BattleObjectModuleAccessor, collision_log: u64, fighter_kind: i32, vl_params: u64, param_5: i32, param_6: i32, param_7: i32, param_8: i32, param_9: i32) -> u64;

#[skyline::from_offset(0x33bed40)]
pub fn get_weapon_vtable(id: u32) -> *const usize;