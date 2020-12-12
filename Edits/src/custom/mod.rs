use smash::lib::{L2CValue, L2CAgent};
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smash::phx::*;
use acmd;
use skyline::nn::ro::LookupSymbol;

static mut LOCKED: [bool; 9] = [false; 9];
static mut CANAIRDODGE: [bool; 9] = [true; 9];

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let mut l2c_agent = L2CAgent::new(lua_state);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = get_kind(boma);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        let cat3 = ControlModule::get_command_flag_cat(boma, 2);
        let stick_value_y = ControlModule::get_stick_y(boma);
        let stick_value_x = ControlModule::get_stick_x(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let kinetic_type = KineticModule::get_kinetic_type(boma);
        let flick_x_dir = ControlModule::get_flick_x_dir(boma);
        
        
        dacus(boma, status_kind, cat1, stick_value_y);
        landCancels(boma, status_kind, situation_kind, fighter_kind);
        perfectPivots(boma, status_kind, stick_value_x);
        sm4shJabLocks(boma, status_kind);
        removeSHMacro(boma, status_kind);
        quickAttackCancels(boma, status_kind, situation_kind, fighter_kind, stick_value_y);  
        regainAirDodge(boma, status_kind, situation_kind);
        enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    }
}

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

//DACUS
pub unsafe fn dacus(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32, stick_value_y: f32) { //Dacus
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if MotionModule::frame(boma) < (10.0){
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (stick_value_y >= 0.7 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if MotionModule::frame(boma) > (2.0){
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
            }
        }
    }
}

//fox/falco land cancle blaster
pub unsafe fn landCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) { //Land Cancels
    if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

//perfect pivots
pub unsafe fn perfectPivots(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32) { //Perfect Pivots and microdashes
    if [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
        if MotionModule::frame(boma) < (3.0) && stick_value_x < 0.5 && stick_value_x > -0.5 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
}

//smash 4 jab locks (forces neutral getup)
pub unsafe fn sm4shJabLocks(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_DOWN_DAMAGE { //Sm4sh jab locks
        LOCKED[get_player_number(boma)] = true;
    }
    else if [*FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK].contains(&status_kind) {
        if LOCKED[get_player_number(boma)] {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN_STAND, true);
            LOCKED[get_player_number(boma)] = false;
        }
    }
    else if ![*FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE].contains(&status_kind) {
        LOCKED[get_player_number(boma)] = false;
    }
}

pub unsafe fn removeSHMacro(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
    || StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_JUMP {
        if MotionModule::frame(boma) < 2.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
    }
}

//pikachu quick attack cancles
pub unsafe fn quickAttackCancels (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_value_y: f32) {
    if [*FIGHTER_KIND_PICHU, *FIGHTER_KIND_PIKACHU].contains(&fighter_kind) {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if (GroundModule::is_passable_ground(boma) && stick_value_y <= -0.75) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) 
                || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
        }
    }
}

pub unsafe fn regainAirDodge(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if situation_kind != *SITUATION_KIND_AIR {
        CANAIRDODGE[get_player_number(boma)] = true;
    }
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        CANAIRDODGE[get_player_number(boma)] = true;
    }
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]

pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
 
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if !CANAIRDODGE[get_player_number(boma)] {
            if CancelModule::is_enable_cancel(boma) {
                return false;
            }
        }
    }

    original!()(boma, flag)
}

//random tripping
pub unsafe fn randomTripping (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN].contains(&status_kind) {
        let rng = smash::app::sv_math::rand(0, 100);
        if rng == 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SLIP, true);
        }
    }
} 

//removed short hop buffer
pub unsafe fn removeSHMacro(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
    || StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_JUMP {
        if MotionModule::frame(boma) < 2.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
    }
}

// Use this for general per-frame weapon-level hooks
pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}