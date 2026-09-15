use std::io;

use nx_bone::Quaternion;
use nx_common::Readable;
use nx_skel::Skeleton;
use quaternion_core as quat;

use crate::boned::{AnimBone, Animation};

pub const THPS4_SKEL_BYTES: &[u8] =
    include_bytes!("../../../../assets/skel/thug/THPS4_human.ske.xbx");
pub const THUG_SKEL_BYTES: &[u8] =
    include_bytes!("../../../../assets/skel/thug/THPS5_human.ske.xbx");

pub fn thug_to_thps4_test() -> Result<(), nx_skel::Error> {
    let thps4_skel = Skeleton::read(&mut io::Cursor::new(THPS4_SKEL_BYTES), &mut ())?.to_boned()?;
    let thug_skel = Skeleton::read(&mut io::Cursor::new(THUG_SKEL_BYTES), &mut ())?.to_boned()?;

    let r2d = 57.29578;

    for (i, bone) in thps4_skel.bones.iter().enumerate() {
        let q = bone.quaternion;
        let [tx, ty, tz] = quat::to_euler_angles(
            quat::RotationType::Intrinsic,
            quat::RotationSequence::XYZ,
            (q.w(), [q.x(), q.y(), q.z()]),
        );
        let (dx, dy, dz) = (tx * r2d, ty * r2d, tz * r2d);
        log::info!("bone_{} angles=({}, {}, {})", i, dx, dy, dz);
    }

    for (i, bone) in thug_skel.bones.iter().enumerate() {
        let q = bone.quaternion;
        let [tx, ty, tz] = quat::to_euler_angles(
            quat::RotationType::Intrinsic,
            quat::RotationSequence::XYZ,
            (q.w(), [q.x(), q.y(), q.z()]),
        );
        let (dx, dy, dz) = (tx * r2d, ty * r2d, tz * r2d);
        log::info!("bone_{} angles=({}, {}, {})", i, dx, dy, dz);
    }

    Ok(())
}

pub fn thug_to_thps4(thug: &Animation) -> Result<Animation, nx_skel::Error> {
    let thps4_skel = Skeleton::read(&mut io::Cursor::new(THPS4_SKEL_BYTES), &mut ())?.to_boned()?;
    let thug_skel = Skeleton::read(&mut io::Cursor::new(THUG_SKEL_BYTES), &mut ())?.to_boned()?;

    Ok(Animation {
        duration: thug.duration,
        // thps4 <- thug
        bones: vec![
            // 0 (dummy_scale_zz) <- 0 (Control_Root)
            thug.bone(0),
            // .translate(thps4_skel.bone(0).translation - thug_skel.bone(0).translation),
            // 1 (pelvis) <- 1 (Bone_Pelvis)
            thug.bone(1),
            // .translate(thps4_skel.bone(1).translation - thug_skel.bone(1).translation)
            // .translate(Translation::new(0.0, 0.0, -2.0)),
            // 2 (stomach) <- 2 (Bone_Stomach_Lower)
            thug.bone(2).baseline(thug_skel.bone(2), thps4_skel.bone(2)),
            // 3 (chest) <- 4 (Bone_Chest)
            thug.bone(4).baseline(thug_skel.bone(4), thps4_skel.bone(3)),
            // 4 (neck) <- 29 (Bone_Neck)
            thug.bone(29)
                .baseline(thug_skel.bone(29), thps4_skel.bone(4)),
            // 5 (head) <- 30 (Bone_Head)
            thug.bone(30)
                .baseline(thug_skel.bone(30), thps4_skel.bone(5)),
            // 6 (mullet_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(6)),
            // 7 (jaw) <- 31 (Bone_Jaw)
            thug.bone(31)
                .baseline(thug_skel.bone(31), thps4_skel.bone(7)),
            // 8 (breast_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(8)),
            // 9 (left_collar) <- 5 (Bone_Collar_L)
            thug.bone(5).baseline(thug_skel.bone(5), thps4_skel.bone(9)),
            // 10 (left_bicep) <- 6 (Bone_Bicep_L)
            thug.bone(6)
                .baseline(thug_skel.bone(6), thps4_skel.bone(10)),
            // 11 (left_forearm) <- 7 (Bone_Forearm_L)
            thug.bone(7)
                .baseline(thug_skel.bone(7), thps4_skel.bone(11)),
            // 12 (left_wrist)
            thug.bone(8).scale_translations(0.6),
            // 13 (left_palm) <- 8 (Bone_Palm_L)
            thug.bone(8)
                .scale_translations(0.4)
                .with_static_quaternion(Quaternion::identity()),
            // 14 (left_thumb_base) <- 13 (Bone_Thumb_L)
            thug.bone(13)
                .baseline(thug_skel.bone(13), thps4_skel.bone(14)),
            // 15 (left_forefinger_base) <- 11 (Bone_Forefinger_Base_L)
            thug.bone(11)
                .baseline(thug_skel.bone(11), thps4_skel.bone(15)),
            // 16 (left_forefinger_tip) <- 12 (Bone_Forefinger_Tip_L)
            thug.bone(12)
                .baseline(thug_skel.bone(12), thps4_skel.bone(16)),
            // 17 (left_fingers_base) <- 9 (Bone_Fingers_Base_L)
            thug.bone(9)
                .baseline(thug_skel.bone(9), thps4_skel.bone(17)),
            // 18 (left_fingers_tip) <- 10 (Bone_Fingers_Tip_L)
            thug.bone(10)
                .baseline(thug_skel.bone(10), thps4_skel.bone(18)),
            // 19 (left_low_sleeve_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(19)),
            // 20 (left_top_sleeve_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(20)),
            // 21 (right_collar) <- 17 (Bone_Collar_R)
            thug.bone(17)
                .baseline(thug_skel.bone(17), thps4_skel.bone(21)),
            // 22 (right_bicep) <- 18 (Bone_Bicep_R)
            thug.bone(18)
                .baseline(thug_skel.bone(18), thps4_skel.bone(22)),
            // 23 (right_forearm) <- 19 (Bone_Forearm_R)
            thug.bone(19)
                .baseline(thug_skel.bone(19), thps4_skel.bone(23)),
            // 24 (right_wrist)
            thug.bone(20).scale_translations(0.6),
            // 25 (right_palm) <- 20 (Bone_Palm_R)
            thug.bone(20)
                .scale_translations(0.4)
                .with_static_quaternion(Quaternion::identity()),
            // 26 (right_thumb_base) <- 25 (Bone_Thumb_R)
            thug.bone(25)
                .baseline(thug_skel.bone(25), thps4_skel.bone(26)),
            // 27 (right_forefinger_base) <- 21 (Bone_Forefinger_Base_R)
            thug.bone(21)
                .baseline(thug_skel.bone(21), thps4_skel.bone(27)),
            // 28 (right_forefinger_tip) <- 22 (Bone_Forefinger_Tip_R)
            thug.bone(22)
                .baseline(thug_skel.bone(22), thps4_skel.bone(28)),
            // 29 (right_fingers_base) <- 23 (Bone_Fingers_Base_R)
            thug.bone(23)
                .baseline(thug_skel.bone(23), thps4_skel.bone(29)),
            // 30 (right_fingers_tip) <- 24 (Bone_Fingers_Tip_R)
            thug.bone(24)
                .baseline(thug_skel.bone(24), thps4_skel.bone(30)),
            // 31 (right_low_sleeve_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(31)),
            // 32 (right_top_sleeve_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(32)),
            // 33 (hood_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(33)),
            // 34 (right_hip) <- 40 (Bone_Thigh_R)
            thug.bone(40)
                .baseline(thug_skel.bone(40), thps4_skel.bone(34)),
            // 35 (right_knee) <- 41 (Bone_Knee_R)
            thug.bone(41)
                .baseline(thug_skel.bone(41), thps4_skel.bone(35)),
            // 36 (right_ankle) <- 42 (Bone_Ankle_R)
            thug.bone(42)
                .baseline(thug_skel.bone(42), thps4_skel.bone(36)),
            // 37 (right_toes) <- 43 (Bone_Toe_R)
            thug.bone(43)
                .baseline(thug_skel.bone(43), thps4_skel.bone(37)),
            // 38 (right_low_trouser_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(38)),
            // 39 (right_top_trouser_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(39)),
            // 40 (shirt_tail_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(40)),
            // 41 (left_hip) <- 45 (Bone_Thigh_L)
            thug.bone(45)
                .baseline(thug_skel.bone(45), thps4_skel.bone(41)),
            // 42 (left_top_trouser_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(42)),
            // 43 (left_knee) <- 46 (Bone_Knee_L)
            thug.bone(46)
                .baseline(thug_skel.bone(46), thps4_skel.bone(43)),
            // 44 (left_ankle) <- 48 (Bone_Ankle_L)
            thug.bone(48)
                .baseline(thug_skel.bone(48), thps4_skel.bone(44)),
            // 45 (left_toes) <- 49 (Bone_Toe_L)
            thug.bone(49)
                .baseline(thug_skel.bone(49), thps4_skel.bone(45)),
            // 46 (left_low_trouser_cloth_zz)
            AnimBone::static_bone(thps4_skel.bone(46)),
            // 47 (burnq_board) <- 50 (Bone_Board_Root)
            thug.bone(50),
            // .translate(thps4_skel.bone(47).translation - thug_skel.bone(50).translation),
            // 48 (burnq_front_wheel) <- 51 (Bone_Board_Nose)
            AnimBone::static_bone(thps4_skel.bone(48)),
            // thug.bone(51)
            //     .baseline(thug_skel.bone(51), thps4_skel.bone(48)),
            // 49 (burnq_back_wheel) <- 53 (Bone_Board_Tail)
            AnimBone::static_bone(thps4_skel.bone(49)),
            // thug.bone(53)
            //     .baseline(thug_skel.bone(53), thps4_skel.bone(49)),
        ],
    })
}
