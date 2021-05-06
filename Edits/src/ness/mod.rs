use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use skyline::nn::ro::LookupSymbol;
use smash::app;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_NESS, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn ness_attack_uair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=2)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=8)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=13.0, Angle=85, KBG=109, FKB=0, BKB=13, Size=7.2, X=0.0, Y=3.408, Z=0.0, X2=2.4, Y2=-1.0, Z2=-3.5, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
        }
        frame(Frame=4)
        if(is_excute){
        AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.87)
        frame(Frame=40)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        FT_MOTION_RATE(FSM=1)        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_NESS, 
    animation = "attack_air_hi",
    animcmd = "effect_attackairhi")]
pub fn ness_effect_uair(fighter: &mut L2CFighterCommon) {
    acmd!({

    });
}

pub fn install() {
    acmd::add_hooks!(
        ness_attack_uair, ness_effect_uair
    );
}