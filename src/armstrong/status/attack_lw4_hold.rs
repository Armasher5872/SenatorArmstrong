use super::*;

unsafe extern "C" fn armstrong_attack_hi4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, armstrong_attack_hi4_hold_end_status)
    .install()
    ;
}