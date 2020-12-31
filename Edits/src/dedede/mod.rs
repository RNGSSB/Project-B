use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use skyline::nn::ro::LookupSymbol;
use smash::app;

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_DEDEDE,
    animation = "attack_air_b",
    animcmd = "game_attackairb"
    )]
fn dedede_attack_air_b_game(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(4)
        if (is_execute) {
            WorkModule::on_flag( /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(6)
        if (is_execute) {
            ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -6.0, /*X2*/ LUA_VOID, /*Y2*/ LUA_VOID, /*Z2*/ LUA_VOID, /*Hitlag*/ 7.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -12.0, /*X2*/ LUA_VOID, /*Y2*/ LUA_VOID, /*Z2*/ LUA_VOID, /*Hitlag*/ 7.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        frame(9)
        if (is_execute) {
            ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -6.0, /*X2*/ LUA_VOID, /*Y2*/ LUA_VOID, /*Z2*/ LUA_VOID, /*Hitlag*/ 6.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -12.0, /*X2*/ LUA_VOID, /*Y2*/ LUA_VOID, /*Z2*/ LUA_VOID, /*Hitlag*/ 6.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        frame(21)
        if (is_execute) {
            MotionModule::set_rate( 0.8);
            AttackModule::clear_all();
        }
        frame(26)
        if (is_execute) {
            WorkModule::off_flag( /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    });
}

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_DEDEDE,
    animation = "attack_air_b",
    animcmd = "effect_attackairb"
    )]
fn dedede_attack_air_b_effect(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(6)
        if(is_execute){
            EFFECT_FOLLOW_ALPHA(0x1156ac182a as u64, hash40("top"), 0.0, 10.0, -12.0, 0, 0, 0, 1.5, true, 1.0)
            LAST_EFFECT_SET_RATE(2)
        }
    });
}


pub fn install() {
    acmd::add_hooks!(
dedede_attack_air_b_game, dedede_attack_air_b_effect
);    
}