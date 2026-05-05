#![allow(internal_features)]
use {
    crate::{
        common::{
            armstrong_var::*,
            common_func::*,
            globals::*,
        },
        MARKED_COLORS,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            LuaConst,
            lua_const::*,
        },
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        *,
        macros::*
    }
};

pub mod armstrong_func;
pub mod armstrong_var;
pub mod common_func;
pub mod globals;
pub mod hook;