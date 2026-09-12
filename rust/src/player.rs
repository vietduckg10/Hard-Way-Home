use godot::prelude::*;
use godot::classes::CharacterBody2D;


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    speed: f32,
    jump_velocity: f32,
    gravity: f32,
    jump_hold_time: f32,
    max_jump_hold_time: f32,
    current_animation: StringName,
    is_dead: bool,
}

use godot::classes::ICharacterBody2D;
use godot::classes::{Input, AnimatedSprite2D, Camera2D, AudioStreamPlayer2D};

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self { 
            base,
            speed: 150.0,
            jump_velocity: -300.0,
            gravity: 1200.0,
            jump_hold_time: 0.0,
            max_jump_hold_time: 0.25,
            current_animation: StringName::from("idle"),
            is_dead: false,
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group("player");
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
            let input = Input::singleton();

        if self.is_dead {
            // Make the player fly upward
            velocity.y += self.gravity * delta as f32;

            // Rotate the player
            self.base_mut().rotate(5.0 * delta as f32);
        }
        else {
            let on_floor = self.base().is_on_floor();

            // Horizontal movement
            if input.is_action_pressed("move_left") {
                velocity.x = -self.speed;
            } else if input.is_action_pressed("move_right") {
                velocity.x = self.speed;
            } else {
                velocity.x = 0.0;
            }

            let mut sprite = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

            // Animation
            if velocity.x != 0.0 {
                if self.current_animation != StringName::from("run") {
                    sprite.play_ex().name("run").done();
                    self.current_animation = StringName::from("run");
                }
            } else {
                if self.current_animation != StringName::from("idle") {
                    sprite.play_ex().name("idle").done();
                    self.current_animation = StringName::from("idle");
                }
            }

            // Flip sprite
            if velocity.x < 0.0 {
                sprite.set_flip_h(true);
            } else if velocity.x > 0.0 {
                sprite.set_flip_h(false);
            }

            // Gravity
            if !on_floor {
                let mut current_gravity = self.gravity;

                // Holding jump makes the player rise longer
                if input.is_action_pressed("jump")
                    && velocity.y < 0.0
                    && self.jump_hold_time < self.max_jump_hold_time
                {
                    current_gravity *= 0.4;
                }

                velocity.y += current_gravity * delta as f32;
            }

            // Jump
            if input.is_action_pressed("jump") && on_floor {
                velocity.y = self.jump_velocity;
                self.jump_hold_time = 0.0;
            }

            if input.is_action_pressed("jump")
                && velocity.y < 0.0
                && self.jump_hold_time < self.max_jump_hold_time
            {
                self.jump_hold_time += delta as f32;
            }
        }

        // Apply movement
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        // godot_print!("x: {}, y: {}", self.base().get_position().x, self.base().get_position().y);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn die(&mut self) {
        if self.is_dead {
            return;
        }

        self.is_dead = true;

        self.base().clone().get_node_as::<AudioStreamPlayer2D>("AudioStreamPlayer2D").play();

        // Stop camera from following the player
        let mut camera = self
            .base()
            .clone()
            .get_node_as::<Camera2D>("Camera2D");

        // Remember the camera's current world position
        let camera_position = camera.get_global_position();

        // Remove camera from Player
        camera.set_as_top_level(true);

        // Restore the world position
        camera.set_global_position(camera_position);

        let mut velocity = self.base().get_velocity();

        // Launch player upward
        velocity.y = -500.0;

        // Stop horizontal movement
        velocity.x = 0.0;

        self.base_mut().set_velocity(velocity);

        godot_print!("Player died!");
    }
}