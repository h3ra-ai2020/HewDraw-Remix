// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status_one_of(&[*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_B, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_F, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_B,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_B,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_F,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_F])
    && frame > 12.0
    && boma.is_button_on(Buttons::Guard) {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

unsafe fn uspecial_cancels(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status_one_of(&[*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT]) {
            if boma.is_button_on(Buttons::Guard) || boma.is_button_on(Buttons::Attack) {
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
            }
    } else if boma.is_status_one_of(&[*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END]) && frame > 7.0 {
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            ControlModule::reset_trigger(boma);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dspecial_cancels(boma, situation_kind, frame);
    uspecial_cancels(boma, situation_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_MURABITO )]
pub fn murabito_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		murabito_frame(fighter);
    }
}

pub unsafe fn murabito_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[weapon_frame( agent = WEAPON_KIND_MURABITO_FLOWERPOT )]
fn flowerpot_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if weapon.is_status( *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED ) && AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
            weapon.change_status(WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_BURST.into(), false.into());
        }
    }
}