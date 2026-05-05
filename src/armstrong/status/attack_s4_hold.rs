use super::*;

unsafe extern "C" fn armstrong_attack_hi4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, armstrong_attack_hi4_hold_end_status)
    .install()
    ;
}