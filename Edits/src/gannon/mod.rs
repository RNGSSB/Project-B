use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use skyline::nn::ro::LookupSymbol;
use smash::app;

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn ganon_attackhi4(fighter : &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
            FT_MOTION_RATE(FSM=0.8)
        frame(Frame=8)
            FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=105, FKB=0, BKB=40, Size=4.4, X=0.0, Y=12.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=5.0, X=0.0, Y=12.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=105, FKB=0, BKB=40, Size=3.5, X=0.0, Y=12.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
            FT_MOTION_RATE(FSM=0.9)
            Parser commit 29c9d9a
        }
    });
}

frame(Frame=10)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=13.0, Angle=22, KBG=82, FKB=0, BKB=31, Size=4.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=13.0, Angle=22, KBG=82, FKB=0, BKB=31, Size=5.0, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=14.0, Angle=22, KBG=82, FKB=0, BKB=31, Size=5.5, X=5.3, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
}
wait(Frames=3)
if(is_excute){
AttackModule::clear_all()
}

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn ganon_attackhi4(fighter : &mut L2CFighterCommon) {
    acmd!({
        frame(11);
        if (is_excute) {
        ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 24.0, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ -6.0, /*Z*/ 0.0, LUA_VOID, LUA_VOID, LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 24.0, /*Angle*/ 78, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ -3.2, /*Z*/ 0.0, LUA_VOID, LUA_VOID, LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 19.0, /*Angle*/ 75, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, LUA_VOID, LUA_VOID, LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        wait(3);
        if (is_excute) {
        AttackModule::clear_all();
        }
        frame(46);
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true);
        }
    });
}

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_hi4",
    animcmd = "effect_attackhi4")]
pub fn ganon_attackhi4_effect(fighter : &mut L2CFighterCommon) {
    acmd!({
        frame(11);
        if (is_excute) {
        EFFECT_FOLLOW(/*Effect*/ 0x0e27bc68a2u64, /*Bone*/ hash40("kneel"), /*X*/ -1.0, /*Y*/ -3.0, /*Z*/ 0.0, /*XRot*/ 180, /*YRot*/ 330, /*ZRot*/ 0, /*Size*/ 1.2, true, 0);
        }
        
        frame(14);
        if (is_excute) {
        EFFECT_OFF_KIND(0x0e27bc68a2u64, false, false);
        }
    });
}