#![allow(improper_ctypes_definitions)] //Addresses `extern` fn uses type `str`, which is not FFI-safe
use super::*;

//Checks if the user got damaged
pub unsafe fn is_damaged(boma: *mut BattleObjectModuleAccessor) -> bool {
    let status = StatusModule::status_kind(boma);
    if FighterStopModuleImpl::is_damage_stop(boma)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    || (*FIGHTER_STATUS_KIND_CAPTURE_PULLED..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status)
    || (*FIGHTER_STATUS_KIND_DOWN..=*FIGHTER_STATUS_KIND_LAY_DOWN).contains(&status)
    || (*FIGHTER_STATUS_KIND_DOWN_DAMAGE..=*FIGHTER_STATUS_KIND_BIND).contains(&status)
    || (*FIGHTER_STATUS_KIND_SLIP..=*FIGHTER_STATUS_KIND_SLIP_WAIT).contains(&status)
    || (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_ICE_JUMP).contains(&status)
    || (*FIGHTER_STATUS_KIND_LINK_FINAL..=*FIGHTER_STATUS_KIND_PIT_FALL).contains(&status)
    || (*FIGHTER_STATUS_KIND_SWALLOWED..=*FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI).contains(&status)
    || (*FIGHTER_STATUS_KIND_CATCHED_REFLET..=*FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND).contains(&status)
    || status == *FIGHTER_STATUS_KIND_GIMMICK_EATEN
    || (*FIGHTER_STATUS_KIND_CAPTURE_ITEM..=*FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP).contains(&status)
    || (*FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER..=*FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_CATCHED_RIDLEY..=*FIGHTER_STATUS_KIND_STABBED_DAMAGE).contains(&status)
    || (*FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED..=*FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE).contains(&status)
    || (*FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE..=*FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS).contains(&status)
    || (*FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START..=*FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_SUICIDE_BOMB..=*FIGHTER_STATUS_KIND_TANTAN_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD..=*FIGHTER_STATUS_KIND_EDGE_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_CAPTURE_TRAIL_KEYHOLE..=*FIGHTER_STATUS_KIND_TRAIL_FINAL_TARGET_END).contains(&status) {
        return true;
    }
    false
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

//Gets the necessary grab animation for throws
pub unsafe extern "C" fn grabbed_anim_selector(fighter: &mut L2CFighterCommon, anim_name: &str, set_frame: f32, mot_rate: f32) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if capture_id as i32 != *BATTLE_OBJECT_ID_INVALID {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let motion_share = WorkModule::get_param_int(capture_boma, hash40("param_motion"), hash40("motion_share"));
        let mut motion = hash40(anim_name);
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_DX);
        }
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_GIRL);
        }
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_BIG);
        }
        MotionModule::change_motion(capture_boma, Hash40::new_raw(motion), set_frame, mot_rate, false, 0.0, false, false);
    }
}

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

//Used to get the pointer for a vtable function within a specific module.
pub unsafe fn get_module_vtable_func(boma: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
    let module = (boma as *mut u64).add(module_offset/0x8);
    let vtable = *module as *const u64;
    *((*vtable + func_offset) as *const u64)
}

//Full Smash Attack Check
pub unsafe fn attack_4_hold(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if frame >= 59.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
    physics!(fighter, MA_MSC_CMD_PHYSICS_STOP_CHARGE);
    fighter.pop_lua_stack(1);
}

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash2::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32, //x24 - x27
    pub opponent_object_category: u8, //x28
    pub x29: u8,
    pub x2a: u8,
    pub x2b: u8,
    pub x2c: u8,
    pub x2d: u8,
    pub x2e: u8,
    pub collision_kind: u8, //x2f
    pub receiver_part_id: u8, //x30
    pub collider_part_id: u8, //x31
    pub receiver_id: u8, //x32
    pub collider_id: u8, //x33
    pub x35: bool
}

/*
Credited to IncrediblePlays for the original logic of the function (Of which this is a modified form of). Used to get either the address or pointer address of an agent (fighter/weapon/item) virtual function. 

kind: The respective fighter/weapon kind, takes the dereferenced lua_const value (E.G. *FIGHTER_KIND_SNAKE)
entry: The entry of the virtual function you want to hook. Documentation can be found here: https://github.com/theincredibleplayer/smash-vtables/blob/main/Vtable/fighter_vtable_documentation.txt
is_weapon: Indicates if the agent is a weapon
is_pointer: Determines whether or not it should get the address hook itself, or the pointer to the address hook. The pointer can be seen as the "index" of the agent's virtual table.

An example of its usage would be: get_agent_virtual_function(*FIGHTER_KIND_SNAKE, 13, false, false); This would get the 13th (0-Indexed) virtual function of Snake, which is his OPFF.

This function also has additional checks that will forcibly close the game if the function params are out of bounds
*/
pub fn get_agent_virtual_function(kind: i32, entry: usize, is_weapon: bool, is_pointer: bool) -> usize {
    if kind < 0 {
        std::process::abort()
    }
    unsafe {
        if is_weapon {
            if kind >= 0x267 || entry >= 104 {
                std::process::abort();
            }
        }
        else{
            if kind >= 0x5E || entry >= 146 {
                std::process::abort();
            }
        }
        let vtable = if is_weapon {get_weapon_vtable(kind as u32)} else {get_fighter_vtable(kind as u32)};
        let first_entry_ptr = *(vtable as *const u64) as *const usize;
        let main = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text);
        if is_pointer {
            return first_entry_ptr.add(entry) as usize-main as usize;
        }
        else {
            return *first_entry_ptr.add(entry)-main as usize;
        }
    }
}