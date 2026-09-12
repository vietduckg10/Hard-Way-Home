use godot::prelude::*;
use godot::classes::{Area2D, IStaticBody2D, Sprite2D, StaticBody2D, CollisionShape2D};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct InvisibleTile {
    base: Base<StaticBody2D>
}

#[godot_api]
impl IStaticBody2D for InvisibleTile {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let invisible_tile = self.base().clone();
        let trigger_area = self.base().clone().get_node_as::<Area2D>("TriggerArea");

        trigger_area.signals()
            .body_entered()
            .connect(move |body: Gd<Node2D>| {

                if body.is_in_group("player") {
                    let mut player = body.cast::<Player>();
                    let mut velocity = player.get_velocity();

                    if 0.0 > velocity.y {
                        let mut sprite = invisible_tile.get_node_as::<Sprite2D>("Sprite2D");
                        let mut collision_box = invisible_tile.get_node_as::<CollisionShape2D>("CollisionShape2D");
                        
                        sprite.set_visible(true);
                        collision_box.set_deferred("disabled", &false.to_variant());
                        godot_print!("is disabled: {}", collision_box.is_disabled());
                        velocity.y = 0.0;
                        player.set_velocity(velocity);
                    }
                }
                
            });
    }
}