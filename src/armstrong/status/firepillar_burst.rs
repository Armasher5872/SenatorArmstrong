use super::*;

unsafe extern "C" fn armstrong_firepillar_burst_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn armstrong_firepillar_burst_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let founder_boma = get_founder_boma(weapon);
    let life = WorkModule::get_param_int(boma, hash40("param_firepillar"), hash40("life"));
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(founder_boma, weapon.battle_object_id as i32, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_FLAME_PILLAR_ID);
    0.into()
}

unsafe extern "C" fn armstrong_firepillar_burst_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(armstrong_firepillar_burst_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_firepillar_burst_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let boma = weapon.module_accessor;
    let founder_boma = get_founder_boma(weapon);
    let founder_lr = PostureModule::lr(founder_boma);
    let founder_pos_x = PostureModule::pos_x(founder_boma);
    let founder_pos_y = PostureModule::pos_y(founder_boma);
    let founder_pos_z = PostureModule::pos_z(founder_boma);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos_x = WorkModule::get_float(boma, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
    let pos_y = WorkModule::get_float(boma, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if frame < 3.0 {
        PostureModule::set_pos(boma, &Vector3f{x: founder_pos_x+(pos_x*founder_lr), y: founder_pos_y+pos_y, z: founder_pos_z});
    }
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new("wn_msc_cmd_self_wn_remove"));
    }
    0.into()
}

unsafe extern "C" fn armstrong_firepillar_burst_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let founder_boma = get_founder_boma(weapon);
    WorkModule::set_int(founder_boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_FLAME_PILLAR_ID);
    WorkModule::set_int(boma, 0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
    WorkModule::set_float(boma, 0.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
    WorkModule::set_float(boma, 0.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_CHARGE);
    0.into()
}

pub fn install() {
    Agent::new("ganon_firepillar")
    .set_costume(get_armstrong_costumes_acmd())
    .status(Pre, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_pre_status)
    .status(Init, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_init_status)
    .status(Main, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_main_status)
    .status(End, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_end_status)
    .install()
    ;
}