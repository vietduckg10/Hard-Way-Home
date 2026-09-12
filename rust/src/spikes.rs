use godot::prelude::*;
use godot::classes::{Node2D, INode2D, Area2D};

use crate::killzone::KillZone;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Spikes {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Spikes {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Spikes ready!");

        let mut sprite = self
            .base()
            .clone()
            .get_node_as::<Node2D>("Sprite2D");
        let trigger_area = self.base().clone().get_node_as::<Area2D>("TriggerZone");
        let mut killzone = self.base().clone().get_node_as::<KillZone>("KillZone");
        // let mut tween = self.base_mut().create_tween();

        // let target_y = (-32.0_f32).to_variant();

        trigger_area.signals()
            .body_entered()
            .connect(move |body: Gd<Node2D>| {
                godot_print!("Trigger zone detected: {}", body.get_name());

                if body.is_in_group("player") {
                    sprite.set_visible(true);
                    killzone.set_monitoring(true);

                    // tween.tween_property(
                    //     &sprite,
                    //     "position:y",
                    //     &target_y,
                    //     5.0,
                    // );

                    godot_print!("Player entered the Trigger zone!");
                }
                
            });
    }
}