use godot::prelude::*;
use godot::classes::{
    AnimatedSprite2D, 
    Area2D, 
    CharacterBody2D, 
    ICharacterBody2D, 
    Node2D, 
    RayCast2D};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Enemy {
    base: Base<CharacterBody2D>,
    speed: f32,
    direction: f32,
    gravity: f32,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            speed: 100.0,
            direction: -1.0,
            gravity: 1200.0,
        }
    }

    fn ready(&mut self) {
        godot_print!("Enemy ready!");

        let damage_area =
            self.base()
                .clone()
                .get_node_as::<Area2D>("KillZone");

        damage_area
            .signals()
            .body_entered()
            .connect(|body: Gd<Node2D>| {
                if body.is_in_group("player") {
                    godot_print!("Enemy hit the player!");
                }
            });
    }

    fn physics_process(&mut self, delta: f64) {
        if 144.0 < self.base().get_position().y {
            self.base_mut().queue_free();
        }
        else {
            let on_floor = self.base().is_on_floor();
            let raycast_right = self.base().clone().get_node_as::<RayCast2D>("RayCast2DRight");
            let raycast_left = self.base().clone().get_node_as::<RayCast2D>("RayCast2DLeft");

            // Horizontal movement
            let mut velocity = self.base().get_velocity();

            if raycast_right.is_colliding() {
                self.direction = -1.0;
            }
            if raycast_left.is_colliding() {
                self.direction = 1.0;
            }
            
            velocity.x = self.speed * self.direction;

            // Gravity
            if !on_floor {
                velocity.y += self.gravity * delta as f32;
            }

            let mut sprite = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

            // Flip sprite
            if velocity.x < 0.0 {
                sprite.set_flip_h(true);
            } else if velocity.x > 0.0 {
                sprite.set_flip_h(false);
            }

            // Apply movement
            self.base_mut().set_velocity(velocity);
            self.base_mut().move_and_slide();
        }
    }
}