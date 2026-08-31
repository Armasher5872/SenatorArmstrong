use super::*;

const ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ARMSTRONG_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaa6520; //Armstrong only
const ARMSTRONG_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540; //Armstrong only
const ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Shared
const ARMSTRONG_VTABLE_LINK_EVENT_OFFSET: usize = 0xaa6990; //Shared

//Armstrong Reset Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        if is_armstrong_slots(boma) {
            armstrong_var(&mut *boma);
        }
    }
    original!()(vtable, fighter)
}

//Armstrong Death Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_death_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    if is_armstrong_slots(boma) {
        armstrong_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Armstrong On Attack
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn armstrong_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let opponent_object = get_battle_object_from_id(opponent_object_id);
    let opponent_battle_object_id = (*opponent_object).battle_object_id;
    let status_kind = StatusModule::status_kind(boma);
    if is_armstrong_slots(boma) {
        let charge = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
        if status_kind == *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_ATTACK && charge > 0.75 {
            call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
        }
        if collision_kind == 1 {
            if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
                    let opponent_boma = sv_battle_object::module_accessor(opponent_battle_object_id);
                    let opponent_situation_kind = StatusModule::situation_kind(opponent_boma);
                    if opponent_situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DOWN, false);
                    }
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Armstrong Link Event
#[skyline::hook(offset = ARMSTRONG_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn armstrong_link_event(vtable: u64, fighter: &mut Fighter, log: *mut u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(boma);
    let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
    let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
    let event: &mut LinkEvent = std::mem::transmute(log);
    let event_kind = event.link_event_kind.0;
    if is_armstrong_slots(boma) {
        if event_kind == hash40("capture") {
            let capture_event: &mut LinkEventCapture = std::mem::transmute(event);
            let captured_status = capture_event.status;
            if status == *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN {
                if captured_status == *FIGHTER_STATUS_KIND_CATCHED_GANON {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, false);
                    capture_event.result = true;
                    capture_event.node = smash2::phx::Hash40::new("throw");
                    return 0;
                }
                if captured_status == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, false);
                    capture_event.result = true;
                    capture_event.node = smash2::phx::Hash40::new("throw");
                    return 0;
                }
            }
            if captured_status == *FIGHTER_STATUS_KIND_THROWN {
                capture_event.node = smash2::phx::Hash40::new("throw");
                capture_event.result = true;
                if status == *FIGHTER_STATUS_KIND_FINAL {
                    capture_event.motion_offset = offset;
                    capture_event.motion_offset_lw = offset_lw;
                    StatusModule::change_status_request(boma, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, false);
                }
                if status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, false);
                    capture_event.motion_offset = offset;
                    capture_event.motion_offset_lw = offset_lw;
                    return 0;
                }
                return 0;
            }
        }
    }
    else {
        // Not an Armstrong-marked slot: don't reimplement vanilla behavior here.
        // Defer immediately to the next hook in the chain (another installed
        // Ganondorf-based moveset, e.g. Broly, or the true original function)
        // instead of matching on the shared status/event constants ourselves
        // and potentially swallowing events that belong to another mod.
        return original!()(vtable, fighter, log);
    }
    // Armstrong-marked slot, but this particular event/status combo wasn't
    // one we needed to intercept above: pass it down the chain too.
    original!()(vtable, fighter, log)
}

//Armstrong On Damage
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn armstrong_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        if is_armstrong_slots(fighter.battle_object.module_accessor) {
            let boma = fighter.battle_object.module_accessor;
            let agent = get_fighter_common_from_accessor(&mut *boma);
            let motion_kind = MotionModule::motion_kind(boma);
            let frame = MotionModule::frame(boma);
            let status_kind = StatusModule::status_kind(boma);
            let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
            let situation_kind = StatusModule::situation_kind(boma);
            let statuses = [
                *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4,
                *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4,
                *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_STATUS_KIND_SPECIAL_LW
            ];
            if status_kind == *FIGHTER_STATUS_KIND_APPEAL
            && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
            && (1.0..=79.0).contains(&frame) {
                HitModule::set_check_catch(boma, true, 0);
                DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
                MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
            }
            if statuses.contains(&status_kind) || statuses.contains(&prev_status_kind) {
                PLAY_SE(agent, Hash40::new("se_common_metal_step_l"));
                DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && situation_kind == *SITUATION_KIND_GROUND && WorkModule::is_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_COUNTER_ACTIVE) {
                    WorkModule::on_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
                }
            }
        }
    }
    original!()(vtable, fighter, on_damage)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0xaa6618).nop(); //Nops the original location where Neutral Special inflicts critical zoom, as I want Armstrong to have different places where they inflict critical zoom
	skyline::install_hooks!(
        armstrong_reset_initialization,
        armstrong_death_initialization,
        armstrong_on_attack,
        armstrong_link_event,
        armstrong_on_damage
    );
}
