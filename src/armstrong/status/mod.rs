use {
    crate::{
        common::{
            armstrong_func::*,
            armstrong_var::*,
            common_func::*,
            globals::*,
        },
        MARKED_COLORS
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

mod attack_air;
mod attack_hi3;
mod attack_hi4_hold;
mod attack_lw3;
mod attack_lw4_hold;
mod attack_s3;
mod attack_s4_hold;
mod catched_air_ganon;
mod final_smash;
mod final_throw;
mod firepillar_burst;
mod special_air_s_end;
mod special_hi_throw;
mod special_hi;
mod special_lw;
mod special_n_attack;
mod special_n_charge;
mod special_n;
mod special_s_catch;
mod special_s_end;
mod special_s_fall;
mod special_s_run;
mod special_s;

pub fn install() {
    attack_air::install();
    attack_hi3::install();
    attack_hi4_hold::install();
    attack_lw3::install();
    attack_lw4_hold::install();
    attack_s3::install();
    attack_s4_hold::install();
    catched_air_ganon::install();
    final_smash::install();
    final_throw::install();
    firepillar_burst::install();
    special_air_s_end::install();
    special_hi_throw::install();
    special_hi::install();
    special_lw::install();
    special_n_attack::install();
    special_n_charge::install();
    special_n::install();
    special_s_catch::install();
    special_s_end::install();
    special_s_fall::install();
    special_s_run::install();
    special_s::install();
}