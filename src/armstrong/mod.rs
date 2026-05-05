#![allow(static_mut_refs, unused_mut)] //Addresses variable does not need to be mutable and creating a shared reference to mutable static
use {
    crate::{
        common::{
            armstrong_func::*,
            armstrong_var::*,
            common_func::*,
            globals::*,
            hook::*,
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
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash2::app::{
        LinkEvent,
        LinkEventCapture
    },
    smash_script::macros::*,
    smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
    unsafe {
        FIGHTER_ARMSTRONG_GENERATE_ARTICLE_FIREPILLAR += clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "ganon", "firepillar", false);
    }
}