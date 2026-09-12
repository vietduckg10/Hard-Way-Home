use godot::prelude::*;
use godot::classes::{
    AnimatableBody2D, 
    IAnimatableBody2D, 
    Area2D,
    CollisionShape2D};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
pub struct MovingTrap {
    base: Base<AnimatableBody2D>,
    #[export]
    vector: Vector2
}

#[godot_api]
impl IAnimatableBody2D for MovingTrap {
    fn init(base: Base<AnimatableBody2D>) -> Self {
        Self {
            base,
            vector: Vector2::new(0.0, 0.0)
        }
    }

    fn ready(&mut self) {
        godot_print!("MovingTrap ready!");
        let mut moving_trap = self.base_mut().clone();
        let trigger_area = self
            .base()
            .clone()
            .get_node_as::<Area2D>("TriggerArea");
        let vector = self.vector;

        trigger_area.signals()
            .body_entered()
            .connect(move |body: Gd<Node2D>| {
                godot_print!("Moving Trap detected: {}", body.get_name());

                if body.is_in_group("player") {
                    let mut tween = moving_trap.create_tween();

                    body.cast::<Player>().set_velocity(Vector2 { x: (0.0), y: (1.0) });
                    moving_trap.get_node_as::<CollisionShape2D>("CollisionShape2D").queue_free();

                    tween
                        .tween_property(
                            &moving_trap,
                            "position:y",
                            &vector.y.to_variant(),
                            0.5,
                        )
                        .as_relative();

                    tween
                        .tween_property(
                            &moving_trap,
                            "position:x",
                            &vector.x.to_variant(),
                            0.5,
                        )
                        .as_relative();
                }
                
            });
    }
}