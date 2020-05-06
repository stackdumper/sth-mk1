extern crate nalgebra as na;

use na::{Isometry3, Point3, Vector3};
use nalgebra::Unit;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::joint::{FreeJoint, RevoluteJoint};
use nphysics3d::material::{BasicMaterial, MaterialHandle};
use nphysics3d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, MultibodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use nphysics_testbed3d::Testbed;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // create worlds
    let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let geometrical_world = DefaultGeometricalWorld::new();

    // add parts
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let constraints = DefaultJointConstraintSet::new();
    let forces = DefaultForceGeneratorSet::new();

    // add ground
    let ground_thickness = 0.2;
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector3::new(10.0, ground_thickness, 10.0)));
    let ground_handle = bodies.insert(Ground::new());
    let ground_collider = ColliderDesc::new(ground_shape)
        .translation(Vector3::y() * -ground_thickness)
        .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.5)))
        .build(BodyPartHandle(ground_handle, 0));

    colliders.insert(ground_collider);

    //// JOINTS
    // base
    // let base_joint = FixedJoint::new(Isometry3::translation(0.0, -5.0, 0.0));
    let base_joint = FreeJoint::new(Isometry3::translation(0.0, 5.0, 0.0));
    let mut base_body = MultibodyDesc::new(base_joint);

    // add legs
    for side in vec![-1.0, 1.0] {
        for leg_i in 0..3 {
            // leg joint 1
            let mut leg_joint_1 = RevoluteJoint::new(
                Unit::new_unchecked(Vector3::new(0.0, side, 0.0)),
                0f32.to_radians(),
            );
            leg_joint_1.enable_min_angle(-60f32.to_radians());
            leg_joint_1.enable_max_angle(60f32.to_radians());
            leg_joint_1.enable_angular_motor();
            // leg_joint_1.set_max_angular_motor_torque(107.0); // 11 kg = 107N
            leg_joint_1.set_desired_angular_motor_velocity(0.0);
            let leg_joint_1_handle = base_body
                .add_child(leg_joint_1)
                .set_body_shift(Vector3::new(0.0, 0.0, 0.0))
                .set_parent_shift(Vector3::new(leg_i as f32 * 1.2 - 1.2, 0.0, 0.7 * side));

            // leg joint 2
            let mut leg_joint_2 = RevoluteJoint::new(
                Unit::new_unchecked(Vector3::new(side, 0.0, 0.0)),
                -20f32.to_radians(),
            );
            leg_joint_2.enable_min_angle(-80f32.to_radians());
            leg_joint_2.enable_max_angle(40f32.to_radians());
            leg_joint_2.enable_angular_motor();
            // leg_joint_2.set_max_angular_motor_torque(107.0); // 11 kg = 107N
            leg_joint_2.set_desired_angular_motor_velocity(0.0);
            let leg_joint_2_handle = leg_joint_1_handle
                .add_child(leg_joint_2)
                .set_body_shift(Vector3::new(0.0, 0.0, -0.4 * side))
                .set_parent_shift(Vector3::new(0.0, 0.0, 0.0 * side));

            // leg joint 3
            let mut leg_joint_3 = RevoluteJoint::new(
                Unit::new_unchecked(Vector3::new(side, 0.0, 0.0)),
                60f32.to_radians(),
            );
            leg_joint_3.enable_min_angle(20f32.to_radians());
            leg_joint_3.enable_max_angle(140f32.to_radians());
            leg_joint_3.enable_angular_motor();
            // leg_joint_3.set_max_angular_motor_torque(107.0); // 11 kg = 107N
            leg_joint_3.set_desired_angular_motor_velocity(0.0);
            let _leg_joint_3_handle = leg_joint_2_handle
                .add_child(leg_joint_3)
                .set_body_shift(Vector3::new(0.0, 0.0, -0.3 * side))
                .set_parent_shift(Vector3::new(0.0, 0.0, 0.4 * side));
        }
    }

    // insert
    let base_body_handle = bodies.insert(base_body.build());

    //// SHAPES
    // common material
    let material_handle = MaterialHandle::new(BasicMaterial::new(0.0, 0.5));

    // base
    let base_collider_shape = ShapeHandle::new(Cuboid::new(Vector3::new(1.2, 0.1, 0.5)));
    let base_collider = ColliderDesc::new(base_collider_shape)
        .density(0.3)
        .material(material_handle.clone())
        .build(BodyPartHandle(base_body_handle, 0));
    colliders.insert(base_collider);

    for leg_i in 0..6 {
        let offset = leg_i * 3;

        // leg joint 1
        let leg_joint_1_shape = ShapeHandle::new(Cuboid::new(Vector3::new(0.1, 0.1, 0.1)));
        let leg_joint_1_collider = ColliderDesc::new(leg_joint_1_shape)
            .density(0.3)
            .material(material_handle.clone())
            .build(BodyPartHandle(base_body_handle, offset + 1));
        colliders.insert(leg_joint_1_collider);

        // leg joint 2
        let leg_joint_2_shape = ShapeHandle::new(Cuboid::new(Vector3::new(0.05, 0.05, 0.2)));
        let leg_joint_2_collider = ColliderDesc::new(leg_joint_2_shape)
            .density(0.3)
            .material(material_handle.clone())
            .build(BodyPartHandle(base_body_handle, offset + 2));
        colliders.insert(leg_joint_2_collider);

        // leg joint 3
        let leg_joint_3_shape = ShapeHandle::new(Cuboid::new(Vector3::new(0.05, 0.05, 0.55)));
        let leg_joint_3_collider = ColliderDesc::new(leg_joint_3_shape)
            .density(0.3)
            .material(material_handle.clone())
            .build(BodyPartHandle(base_body_handle, offset + 3));
        colliders.insert(leg_joint_3_collider);
    }

    //// MOTORS
    // leg_joint_2.set_desired_angular_motor_velocity(0.01);

    let angles = Arc::new(Mutex::new(vec![60f32; 18]));

    // create controller
    {
        let angles = Arc::clone(&angles);

        thread::spawn(move || {
            let sock = UdpSocket::bind("0.0.0.0:5546").expect("Failed to bind socket");
            sock.set_nonblocking(true)
                .expect("Failed to enter non-blocking mode");

            // Poll for data every 5 milliseconds for 5 seconds.
            let mut buf = [0u8; 1024];
            loop {
                let result = sock.recv(&mut buf);

                match result {
                    Ok(num_bytes) => {
                        let msg = std::str::from_utf8(&buf[0..num_bytes])
                            .expect("failed to parse message");

                        let angs = msg
                            .split(",")
                            .into_iter()
                            .map(|t| t.parse().unwrap())
                            .collect::<Vec<f32>>();

                        println!("new angles: {:?}", angs);

                        *angles.lock().unwrap() = angs;
                    }
                    Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                        println!("Something went wrong: {}", err)
                    }
                    _ => {}
                }

                thread::sleep(Duration::from_millis(5));
            }
        });
    }

    // // create controller
    // {
    //     let angles = Arc::clone(&angles);
    //     thread::spawn(move || loop {
    //         for leg_i in 0..6 {
    //             let offset = leg_i * 3;

    //             // (*angles_clone.lock().unwrap())[offset + 0] = 0.0;
    //             (*angles.lock().unwrap())[offset + 1] = -60.0;
    //             (*angles.lock().unwrap())[offset + 2] = 140.0;
    //         }
    //         thread::sleep_ms(2000);

    //         for leg_i in 0..6 {
    //             let offset = leg_i * 3;

    //             // (*angles_clone.lock().unwrap())[offset + 0] = 0.0;
    //             (*angles.lock().unwrap())[offset + 1] = -10.0;
    //             (*angles.lock().unwrap())[offset + 2] = 90.0;
    //         }
    //         thread::sleep_ms(2000);
    //     });
    // }

    // create testbed
    let mut testbed = Testbed::new(
        mechanical_world,
        geometrical_world,
        bodies,
        colliders,
        constraints,
        forces,
    );
    testbed.set_body_color(ground_handle, Point3::new(2.5, 2.5, 2.5));
    // testbed
    //     .mechanical_world_mut()
    //     .integration_parameters
    //     .max_velocity_iterations = 12;
    testbed.add_callback(move |_, _, bodies, _, _, _| {
        let angles = angles.lock().unwrap().clone();
        // println!("target: {}", angles[0]);

        let body = bodies.multibody_mut(base_body_handle).unwrap();
        body.activate();

        for (index, target_angle) in angles.iter().enumerate() {
            // get joint
            let joint = body
                .link_mut(index + 1)
                .unwrap()
                .joint_mut()
                .downcast_mut::<RevoluteJoint<f32>>()
                .unwrap();

            // get current joint angle
            let angle = joint.angle().to_degrees();
            // println!("angle: {}", angle);

            // get diff with target angle
            let diff = (joint.min_angle().unwrap().to_degrees() + target_angle) - angle;

            // if diff is not zero
            if diff.abs() > 0.01 {
                // prevent over-reaching by limiting speed
                // when next pos is bigger than dif
                let speed = 50f32.min(diff.abs() * 60.0);
                let vel = if diff > 0.0 { speed } else { -speed };
                // set velocity
                joint.set_desired_angular_motor_velocity(vel.to_radians());
            } else {
                // set velocity to zero
                joint.set_desired_angular_motor_velocity(0.0);
            }
        }
    });
    testbed.run();
}
