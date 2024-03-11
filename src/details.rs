use crate::ast::{Block, Reporter};

#[rustfmt::skip]
pub fn block_details(
    block: Block,
) -> (&'static str, &'static [&'static str], Option<&'static str>) {
    use Block as B;
    match block {
        B::Move => ("motion_movesteps", &["STEPS"], None),
        B::TurnLeft  => ("motion_turnright", &["DEGREES"], None),
        B::TurnRight => ("motion_turnleft",  &["DEGREES"], None),
        B::GotoRandomPosition => unreachable!(),
        B::GotoMousePointer => unreachable!(),
        B::GotoSprite => unreachable!(),
        B::Goto => ("motion_gotoxy", &["X", "Y"], None),
        B::GlideToRandomPosition => unreachable!(),
        B::GlideToMousePointer => unreachable!(),
        B::GlideToSprite => unreachable!(),
        B::Glide => ("motion_glidesecstoxy", &["X", "Y", "SECS"], None),
        B::PointInDirection => ("motion_pointindirection", &["DIRECTION"], None),
        B::PointTowardsMousePointer => unreachable!(),
        B::PointTowardsRandomDirection => unreachable!(),
        B::PointTowards => unreachable!(),
        B::ChangeX => ("motion_changexby", &["DX"], None),
        B::SetX => ("motion_setx", &["X"], None),
        B::ChangeY => ("motion_changeyby", &["DY"], None),
        B::SetY => ("motion_sety", &["Y"], None),
        B::IfOnEdgeBounce => ("motion_ifonedgebounce", &[], None),
        B::SetRotationStyleLeftRight   => ("motion_setrotationstyle", &[], Some(r#""STYLE":["left-right",null]"#)),
        B::SetRotationStyleDoNotRotate => ("motion_setrotationstyle", &[], Some(r#""STYLE":["don't rotate",null]"#)),
        B::SetRotationStyleAllAround   => ("motion_setrotationstyle", &[], Some(r#""STYLE":["all around",null]"#)),
        B::SayForSeconds => ("looks_sayforsecs", &["MESSAGE", "SECS"], None),
        B::Say => ("looks_say", &["MESSAGE"], None),
        B::ThinkForSeconds => ("looks_thinkforsecs", &["MESSAGE", "SECS"], None),
        B::Think => ("looks_think", &["MESSAGE"], None),
        B::SwitchCostume => unreachable!(),
        B::NextCostume => ("looks_nextcostume", &[], None),
        B::SwitchBackdrop => unreachable!(),
        B::NextBackdrop => ("looks_nextbackdrop", &[], None),
        B::ChangeSize => ("looks_changesizeby", &["CHANGE"], None),
        B::SetSize => ("looks_setsizeto", &["SIZE"], None),
        B::ChangeColorEffect      => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["COLOR",null]"#)),
        B::ChangeFisheyeEffect    => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["FISHEYE",null]"#)),
        B::ChangeWhirlEffect      => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["WHIRL",null]"#)),
        B::ChangePixelateEffect   => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["PIXELATE",null]"#)),
        B::ChangeMosaicEffect     => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["MOSAIC",null]"#)),
        B::ChangeBrightnessEffect => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["BRIGHTNESS",null]"#)),
        B::ChangeGhostEffect      => ("looks_changeeffectby", &["CHANGE"], Some(r#""EFFECT":["GHOST",null]"#)),
        B::SetColorEffect         => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["COLOR",null]"#)),
        B::SetFisheyeEffect       => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["FISHEYE",null]"#)),
        B::SetWhirlEffect         => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["WHIRL",null]"#)),
        B::SetPixelateEffect      => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["PIXELATE",null]"#)),
        B::SetMosaicEffect        => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["MOSAIC",null]"#)),
        B::SetBrightnessEffect    => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["BRIGHTNESS",null]"#)),
        B::SetGhostEffect         => ("looks_seteffectto",    &["VALUE"],  Some(r#""EFFECT":["GHOST",null]"#)),
        B::ClearGraphicEffects => ("looks_cleargraphiceffects", &[], None),
        B::Show => ("looks_show", &[], None),
        B::Hide => ("looks_hide", &[], None),
        B::GotoFront => ("looks_gotofrontback", &[], Some(r#""FRONT_BACK":["front",null]"#)),
        B::GotoBack  => ("looks_gotofrontback", &[], Some(r#""FRONT_BACK":["back",null]"#)),
        B::GoForward  => ("looks_goforwardbackwardlayers", &["NUM"], Some(r#""FORWARD_BACKWARD":["forward",null]"#)),
        B::GoBackward => ("looks_goforwardbackwardlayers", &["NUM"], Some(r#""FORWARD_BACKWARD":["backward",null]"#)),
        B::PlaySoundUntilDone => todo!(),
        B::StartSound => todo!(),
        B::StopAllSounds => ("sound_stopallsounds", &[], None),
        B::ChangePitchEffect => ("sound_changeeffectby", &["VALUE"], Some(r#""EFFECT":["PITCH",null]"#)),
        B::ChangePanEffect   => ("sound_changeeffectby", &["VALUE"], Some(r#""EFFECT":["PAN",null]"#)),
        B::SetPitchEffect    => ("sound_seteffectto",    &["VALUE"], Some(r#""EFFECT":["PITCH",null]"#)),
        B::SetPanEffect      => ("sound_seteffectto",    &["VALUE"], Some(r#""EFFECT":["PAN",null]"#)),
        B::ClearSoundEffects => ("sound_cleareffects", &[], None),
        B::ChangeVolume => ("sound_changevolumeby", &["VOLUME"], None),
        B::SetVolume => ("sound_setvolumeto", &["VOLUME"], None),
        B::Broadcast => ("event_broadcast", &["BROADCAST_INPUT"], None),
        B::BroadcastAndWait => ("event_broadcastandwait", &["BROADCAST_INPUT"], None),
        B::Wait => ("control_wait", &["DURATION"], None),
        B::WaitUntil => ("control_wait_until", &["CONDITION"], None),
        B::StopAll          => ("control_stop", &[], Some(r#""STOP_OPTION":["all",null]"#)),
        B::StopThisScript   => ("control_stop", &[], Some(r#""STOP_OPTION":["this script",null]"#)),
        B::StopOtherScripts => ("control_stop", &[], Some(r#""STOP_OPTION":["other scripts in sprite",null]"#)),
        B::CloneSprite => todo!(),
        B::Clone => todo!(),
        B::DeleteThisClone => ("control_delete_this_clone", &[], None),
        B::Ask => ("sensing_askandwait", &["QUESTION"], None),
        B::SetDraggable    => ("sensing_setdragmode", &[], Some(r#""DRAG_MODE":["draggable",null]"#)),
        B::SetNotDraggable => ("sensing_setdragmode", &[], Some(r#""DRAG_MODE":["not draggable",null]"#)),
        B::ResetTimer => ("sensing_resettimer", &[], None),
        B::EraseAll => ("pen_clear", &[], None),
        B::Stamp => ("pen_stamp", &[], None),
        B::PenDown => ("pen_penDown", &[], None),
        B::PenUp => ("pen_penUp", &[], None),
        B::SetPenColor => ("pen_setPenColorToColor", &["COLOR"], None),
        B::ChangePenHue          => ("pen_changePenColorParamBy", &["VALUE"], Some(r#""COLOR_PARAM":["color",null]"#)),
        B::ChangePenSaturation   => ("pen_changePenColorParamBy", &["VALUE"], Some(r#""COLOR_PARAM":["saturation",null]"#)),
        B::ChangePenBrightness   => ("pen_changePenColorParamBy", &["VALUE"], Some(r#""COLOR_PARAM":["brightness",null]"#)),
        B::ChangePenTransparency => ("pen_changePenColorParamBy", &["VALUE"], Some(r#""COLOR_PARAM":["transparency",null]"#)),
        B::SetPenHue             => ("pen_setPenColorParamTo",    &["VALUE"], Some(r#""COLOR_PARAM":["color",null]"#)),
        B::SetPenSaturation      => ("pen_setPenColorParamTo",    &["VALUE"], Some(r#""COLOR_PARAM":["saturation",null]"#)),
        B::SetPenBrightness      => ("pen_setPenColorParamTo",    &["VALUE"], Some(r#""COLOR_PARAM":["brightness",null]"#)),
        B::SetPenTransparency    => ("pen_setPenColorParamTo",    &["VALUE"], Some(r#""COLOR_PARAM":["transparency",null]"#)),
        B::ChangePenSize => ("pen_changePenSizeBy", &["SIZE"], None),
        B::SetPenSize => ("pen_setPenSizeTo", &["SIZE"], None),
        B::PlayDrum => todo!(),
        B::Rest => ("music_restForBeats", &["BEATS"], None),
        B::PlayNote => ("music_playNoteForBeats", &["NOTE", "BEATS"], None),
        B::SetInstrument => todo!(),
        B::SetTempo => ("music_setTempo", &["TEMPO"], None),
        B::ChangeTempo => ("music_changeTempo", &["TEMPO"], None),
        B::Breakpoint => todo!(),
        B::Log => todo!(),
        B::Warn => todo!(),
        B::Error => todo!(),
    }
}

#[rustfmt::skip]
pub fn reporter_details(
    reporter: Reporter,
) -> (&'static str, &'static [&'static str], Option<&'static str>) {
    use Reporter as R;
    match reporter {
        R::XPosition => ("motion_xposition", &[], None),
        R::YPosition => ("motion_yposition", &[], None),
        R::Direction => ("motion_direction", &[], None),
        R::Size => ("looks_size", &[], None),
        R::Volume => ("sound_volume", &[], None),
        R::MouseDown => ("sensing_mousedown", &[], None),
        R::MouseX => ("sensing_mousex", &[], None),
        R::MouseY => ("sensing_mousey", &[], None),
        R::Loudness => ("sensing_loudness", &[], None),
        R::Timer => ("sensing_timer", &[], None),
        R::CurrentYear      => ("sensing_current", &[], Some(r#""CURRENTMENU":["YEAR",null]"#)),
        R::CurrentMonth     => ("sensing_current", &[], Some(r#""CURRENTMENU":["MONTH",null]"#)),
        R::CurrentDate      => ("sensing_current", &[], Some(r#""CURRENTMENU":["DATE",null]"#)),
        R::CurrentDayOfWeek => ("sensing_current", &[], Some(r#""CURRENTMENU":["DAYOFWEEK",null]"#)),
        R::CurrentHour      => ("sensing_current", &[], Some(r#""CURRENTMENU":["HOUR",null]"#)),
        R::CurrentMinute    => ("sensing_current", &[], Some(r#""CURRENTMENU":["MINUTE",null]"#)),
        R::CurrentSecond    => ("sensing_current", &[], Some(r#""CURRENTMENU":["SECOND",null]"#)),
        R::DaysSince2000 => ("sensing_dayssince2000", &[], None),
        R::Username => ("sensing_username", &[], None),
        R::Random => ("operator_random", &["FROM", "TO"], None),
        R::Answer => ("sensing_answer", &[], None),
        R::Tempo => ("music_getTempo", &[], None),
        R::CostumeNumber  => ("looks_costumenumbername",  &[], Some(r#""NUMBER_NAME":["number",null]"#)),
        R::CostumeName    => ("looks_costumenumbername",  &[], Some(r#""NUMBER_NAME":["name",null]"#)),
        R::BackdropNumber => ("looks_backdropnumbername", &[], Some(r#""NUMBER_NAME":["number",null]"#)),
        R::BackdropName   => ("looks_backdropnumbername", &[], Some(r#""NUMBER_NAME":["name",null]"#)),
        R::TouchingMousePointer => unreachable!(),
        R::TouchingEdge => unreachable!(),
        R::Touching => unreachable!(),
        R::TouchingColor => ("sensing_touchingcolor", &["COLOR"], None),
        R::ColorIsTouchingColor => ("sensing_coloristouchingcolor", &["COLOR", "COLOR2"], None),
        R::DistanceToMousePointer => unreachable!(),
        R::DistanceTo => unreachable!(),
    }
}