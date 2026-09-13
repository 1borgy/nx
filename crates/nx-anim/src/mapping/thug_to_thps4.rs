use std::io;

use nx_bone::{Quaternion, Translation};
use nx_common::{Readable, Writable};
use nx_skel::Skeleton;

use crate::boned::{AnimBone, Animation};

pub const THPS4_SKEL_BYTES: &[u8] =
    include_bytes!("../../../../assets/skel/thug/THPS4_human.ske.xbx");
pub const THUG_SKEL_BYTES: &[u8] =
    include_bytes!("../../../../assets/skel/thug/THPS5_human.ske.xbx");

pub fn thug_to_thps4_test() -> Result<(), nx_skel::Error> {
    let thps4_skel = Skeleton::read(&mut io::Cursor::new(THPS4_SKEL_BYTES), &mut ())?.to_boned()?;
    let thug_skel = Skeleton::read(&mut io::Cursor::new(THUG_SKEL_BYTES), &mut ())?.to_boned()?;

    for (i, bone) in thps4_skel.bones.iter().enumerate() {
        log::info!("bone_{}.magnitude={}", i, bone.translation.magnitude());
    }

    for (i, bone) in thug_skel.bones.iter().enumerate() {
        log::info!("bone_{}.magnitude={}", i, bone.translation.magnitude());
    }

    Ok(())
}

pub fn thug_to_thps4(thug: &Animation) -> Result<Animation, nx_skel::Error> {
    Ok(Animation {
        duration: thug.duration,
        // thps4 <- thug
        bones: vec![
            // 0 (dummy_scale_zz) <- 0 (Control_Root)
            thug.bone(0),
            // 1 (pelvis) <- 1 (Bone_Pelvis)
            thug.bone(1),
            // 2 (stomach) <- 2 (Bone_Stomach_Lower)
            thug.bone(2),
            // 3 (chest) <- 4 (Bone_Chest)
            thug.bone(4)
                .with_static_translation(Translation::new(-0.004, 0.532, 5.659)),
            // 4 (neck) <- 29 (Bone_Neck)
            thug.bone(29)
                .with_static_translation(Translation::new(0.016, 0.997, 12.119)),
            // 5 (head) <- 30 (Bone_Head)
            thug.bone(30)
                .with_static_translation(Translation::new(0.0, -0.054, 2.710)),
            // 6 (mullet_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::identity())
                .with_static_translation(Translation::new(-0.012, 1.992, 4.320)),
            // 7 (jaw) <- 31 (Bone_Jaw)
            thug.bone(31)
                .with_static_translation(Translation::new(0.0, -1.375, 0.65625)),
            // 8 (breast_cloth_zz) <- 36 (Cloth_Breast)
            thug.bone(36)
                .with_static_translation(Translation::new(0.0, -3.84375, 5.75)),
            // 9 (left_collar) <- 5 (Bone_Collar_L)
            thug.bone(5)
                .with_static_translation(Translation::new(0.84375, -1.34375, 9.8125)),
            // 10 (left_bicep) <- 6 (Bone_Bicep_L)
            thug.bone(6)
                .with_static_translation(Translation::new(6.28125, 2.28125, 0.34375)),
            // 11 (left_forearm) <- 7 (Bone_Forearm_L)
            thug.bone(7)
                .with_static_translation(Translation::new(-0.8125, 0.8125, -11.625)),
            // 12 (left_wrist)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::identity())
                .with_static_translation(Translation::new(0.0, -0.8125, -6.5)),
            // 13 (left_palm) <- 8 (Bone_Palm_L)
            thug.bone(8)
                .with_static_translation(Translation::new(0.125, 0.0, -4.5)),
            // 14 (left_thumb_base) <- 13 (Bone_Thumb_L)
            thug.bone(13)
                .with_static_translation(Translation::new(-0.8125, -2.375, -1.84375)),
            // 14 (left_forefinger_base) <- 11 (Bone_Forefinger_Base_L)
            thug.bone(11)
                .with_static_translation(Translation::new(-0.09375, -1.3125, -3.59375)),
            // 16 (left_forefinger_tip) <- 12 (Bone_Forefinger_Tip_L)
            thug.bone(12)
                .with_static_translation(Translation::new(0.0, 0.0, -1.65625)),
            // 17 (left_fingers_base) <- 9 (Bone_Fingers_Base_L)
            thug.bone(9)
                .with_static_translation(Translation::new(-0.15625, 0.34375, -3.59375)),
            // 18 (left_fingers_tip) <- 10 (Bone_Fingers_Tip_L)
            thug.bone(10)
                .with_static_translation(Translation::new(0.0, 0.125, -1.65625)),
            // 19 (left_low_sleeve_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::new(0.0, -0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(-0.1875, -0.5625, -5.09375)),
            // 20 (left_top_sleeve_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::new(0.0, -0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(-1.28125, 0.96875, -5.40625)),
            // 21 (right_collar) <- 17 (Bone_Collar_R)
            thug.bone(17)
                .with_static_translation(Translation::new(-0.84375, -1.34375, 9.8125)),
            // 22 (right_bicep) <- 18 (Bone_Bicep_R)
            thug.bone(18)
                .with_static_translation(Translation::new(-6.28125, 2.28125, 0.34375)),
            // 23 (right_forearm) <- 19 (Bone_Forearm_R)
            thug.bone(19)
                .with_static_translation(Translation::new(0.8125, 0.8125, -11.625)),
            // 24 (right_wrist)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::identity())
                .with_static_translation(Translation::new(0.0, -0.8125, -6.5)),
            // 25 (right_palm) <- 20 (Bone_Palm_R)
            thug.bone(20)
                .with_static_translation(Translation::new(-0.125, 0.0, -4.5)),
            // 26 (right_thumb_base) <- 25 (Bone_Thumb_R)
            thug.bone(25)
                .with_static_translation(Translation::new(0.8125, -2.375, -1.875)),
            // 27 (right_forefinger_base) <- 21 (Bone_Forefinger_Base_R)
            thug.bone(21)
                .with_static_translation(Translation::new(0.09375, -1.34375, -3.59375)),
            // 28 (right_forefinger_tip) <- 22 (Bone_Forefinger_Tip_R)
            thug.bone(22)
                .with_static_translation(Translation::new(0.0, 0.0, -1.65625)),
            // 29 (right_fingers_base) <- 23 (Bone_Fingers_Base_R)
            thug.bone(23)
                .with_static_translation(Translation::new(0.15625, 0.3125, -3.59375)),
            // 30 (right_fingers_tip) <- 24 (Bone_Fingers_Tip_R)
            thug.bone(24)
                .with_static_translation(Translation::new(0.0, 0.125, -1.65625)),
            // 31 (right_low_sleeve_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::new(0.0, 0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(0.1875, -0.5625, -5.09375)),
            // 32 (right_top_sleeve_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::new(0.0, 0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(0.1875, -0.5625, -5.09375)),
            // 33 (hood_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::identity())
                .with_static_translation(Translation::new(0.0, 4.21875, 10.65625)),
            // 34 (right_hip) <- 40 (Bone_Thigh_R)
            thug.bone(40),
            // 35 (right_knee) <- 41 (Bone_Knee_R)
            thug.bone(41),
            // 36 (right_ankle) <- 42 (Bone_Ankle_R)
            thug.bone(42),
            // 37 (right_toes) <- 43 (Bone_Toe_R)
            thug.bone(43),
            // 38 (right_low_trouser_cloth_zz) <- 44 (Cloth_Trouser_R)
            thug.bone(44),
            // 38 (right_low_trouser_cloth_zz)
            // Bone::empty()
            //     .with_static_quaternion(Quaternion::identity())
            //     .with_static_translation(Translation::new(0.184, 0.357, -8.781)),
            // 39 (right_top_trouser_cloth_zz)
            AnimBone::empty()
                .with_static_quaternion(Quaternion::identity())
                .with_static_translation(Translation::new(0.0625, 0.5, -9.8125)),
            // 40 (shirt_tail_cloth_zz) <- 38 (Cloth_Shirt_C)
            thug.bone(38),
            // 41 (left_hip) <- 45 (Bone_Thigh_L)
            thug.bone(45),
            // 42 (left_top_trouser_cloth_zz)
            AnimBone::empty()
                .with_static_translation(Translation::new(-0.0625, 0.5, -9.8125))
                .with_static_quaternion(Quaternion::identity()),
            // 43 (left_knee) <- 46 (Bone_Knee_L)
            thug.bone(46),
            // 44 (left_ankle) <- 48 (Bone_Ankle_L)
            thug.bone(48),
            // 45 (left_toes) <- 49 (Bone_Toe_L)
            thug.bone(49),
            // 46 (left_low_trouser_cloth_zz) <- 47 (Cloth_Trouser_L)
            thug.bone(47),
            // 46 (left_low_trouser_cloth_zz)
            // Bone::empty()
            //     .with_static_quaternion(Quaternion::new(0., 0., 0., 1.))
            //     .with_static_translation(Translation::new(-0.184, 0.357, -8.781)),
            // 47 (burnq_board) <- 50 (Bone_Board_Root)
            thug.bone(50),
            // 48 (burnq_front_wheel) <- 51 (Bone_Board_Nose)
            thug.bone(51),
            // 49 (burnq_back_wheel) <- 53 (Bone_Board_Tail)
            thug.bone(53),
        ],
    })
}
