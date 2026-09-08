use crate::boned::{Animation, Bone, Quaternion, Translation};

pub fn thug_to_thps4(thug1: &Animation) -> Animation {
    Animation {
        duration: thug1.duration,
        bones: vec![
            thug1.bone(0), // 0
            thug1.bone(1), // 1
            thug1.bone(2), // 2
            thug1 // 3
                .bone(4)
                .with_static_translation(Translation::new(0.0, 0.53125, 5.65625)),
            thug1 // 4
                .bone(29)
                .with_static_translation(Translation::new(0.0, 0.96875, 12.09375)),
            thug1 // 5
                .bone(30)
                .with_static_translation(Translation::new(0.0, -0.03125, 2.6875)),
            Bone::empty() // 6
                .with_static_translation(Translation::new(0.0, 1.96875, 4.3125))
                .with_static_quaternion(Quaternion::new(0., 0., 0., 1.)),
            thug1 // 7
                .bone(31)
                .with_static_translation(Translation::new(0.0, -1.375, 0.65625)),
            thug1 // 8
                .bone(36)
                .with_static_translation(Translation::new(0.0, -3.84375, 5.75)),
            thug1 // 9
                .bone(5)
                .with_static_translation(Translation::new(0.84375, -1.34375, 9.8125)),
            thug1 // 10
                .bone(6)
                .with_static_translation(Translation::new(6.28125, 2.28125, 0.34375)),
            thug1 // 11
                .bone(7)
                .with_static_translation(Translation::new(-0.8125, 0.8125, -11.625)),
            Bone::empty() // 12
                .with_static_translation(Translation::new(0.0, -0.8125, -6.5))
                .with_static_quaternion(Quaternion::new(0., 0., 0., 1.)),
            thug1 // 13
                .bone(8)
                .with_static_translation(Translation::new(0.125, 0.0, -4.5)),
            thug1 // 14
                .bone(13)
                .with_static_translation(Translation::new(-0.8125, -2.375, -1.84375)),
            thug1 // 15
                .bone(11)
                .with_static_translation(Translation::new(-0.09375, -1.3125, -3.59375)),
            thug1 // 16
                .bone(12)
                .with_static_translation(Translation::new(0.0, 0.0, -1.65625)),
            thug1 // 17
                .bone(9)
                .with_static_translation(Translation::new(-0.15625, 0.34375, -3.59375)),
            thug1 // 18
                .bone(10)
                .with_static_translation(Translation::new(0.0, 0.125, -1.65625)),
            Bone::empty() // 19
                .with_static_quaternion(Quaternion::new(0.0, -0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(-0.1875, -0.5625, -5.09375)),
            Bone::empty() // 20
                .with_static_quaternion(Quaternion::new(0.0, -0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(-1.28125, 0.96875, -5.40625)),
            thug1 // 21
                .bone(17)
                .with_static_translation(Translation::new(-0.84375, -1.34375, 9.8125)),
            thug1 // 22
                .bone(18)
                .with_static_translation(Translation::new(-6.28125, 2.28125, 0.34375)),
            thug1 // 23
                .bone(19)
                .with_static_translation(Translation::new(0.8125, 0.8125, -11.625)),
            Bone::empty() // 24
                .with_static_translation(Translation::new(0.0, -0.8125, -6.5))
                .with_static_quaternion(Quaternion::new(0., 0., 0., 1.)),
            thug1 // 25
                .bone(20)
                .with_static_translation(Translation::new(-0.125, 0.0, -4.5)),
            thug1 // 26
                .bone(25)
                .with_static_translation(Translation::new(0.8125, -2.375, -1.875)),
            thug1 // 27
                .bone(21)
                .with_static_translation(Translation::new(0.09375, -1.34375, -3.59375)),
            thug1 // 28
                .bone(22)
                .with_static_translation(Translation::new(0.0, 0.0, -1.65625)),
            thug1 // 29
                .bone(23)
                .with_static_translation(Translation::new(0.15625, 0.3125, -3.59375)),
            thug1 // 30
                .bone(24)
                .with_static_translation(Translation::new(0.0, 0.125, -1.65625)),
            Bone::empty() // 31
                .with_static_quaternion(Quaternion::new(0.0, 0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(0.1875, -0.5625, -5.09375)),
            Bone::empty() // 32
                .with_static_quaternion(Quaternion::new(0.0, 0.707, 0.0, 0.707))
                .with_static_translation(Translation::new(0.1875, -0.5625, -5.09375)),
            Bone::empty() // 33
                .with_static_translation(Translation::new(0.0, 4.21875, 10.65625))
                .with_static_quaternion(Quaternion::new(0., 0., 0., 1.)),
            thug1.bone(40), // 34
            thug1.bone(41), // 35
            thug1.bone(42), // 36
            thug1.bone(43), // 37
            thug1.bone(44), // 38
            Bone::empty() // 39
                .with_static_translation(Translation::new(0.0625, 0.5, -9.8125))
                .with_static_quaternion(Quaternion::new(0., 0., 0., 1.)),
            thug1.bone(38), // 40
            thug1.bone(45), // 41
            Bone::empty() // 42
                .with_static_translation(Translation::new(-0.0625, 0.5, -9.8125))
                .with_static_quaternion(Quaternion::new(0., 0., 0., 1.)),
            thug1.bone(46), // 43
            thug1.bone(48), // 44
            thug1.bone(49), // 45
            thug1.bone(47), // 46
            thug1.bone(50), // 47
            thug1.bone(51), // 48
            thug1.bone(53), // 49
        ],
    }
}
