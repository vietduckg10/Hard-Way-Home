use godot::prelude::*;
use godot::classes::{Area2D, IArea2D, Node2D, Timer, CollisionShape2D};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct KillZone {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for KillZone {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world! KillZone"); // Prints to the Godot console
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("KillZone ready!");

        let area = self.base().clone();
        let timer = self.base().clone()
        .get_node_as::<Timer>("Timer");
        let mut timer_for_body = timer.clone();
        let mut tree = self.base().get_tree();

        area.signals()
            .body_entered()
            .connect(move |mut body: Gd<Node2D>| {

                if body.is_in_group("player") {
                    godot_print!("Player entered the KillZone!");
                    timer_for_body.start();
                    body.call("die", &[]);
                    body.cast::<Player>().get_node_as::<CollisionShape2D>("CollisionShape2D").queue_free();
                }
                
            });

        timer.signals()
            .timeout()
            .connect(move || {
                godot_print!("Timeout! Resetting level!");

                // We need the SceneTree to reload the scene.
                tree.reload_current_scene();
            });
        
    }
}
