use godot::prelude::*;
use godot::classes::{Area2D, IArea2D, AnimationPlayer};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Coin {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world! Coin"); // Prints to the Godot console
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Coin ready!");

        let coin = self.base().clone();
        self.base().signals()
        .body_entered()
        .connect(move |body: Gd<Node2D>| {

            if body.is_in_group("player") {
                godot_print!("Player entered the coin!");
                let mut animation_player = coin.get_node_as::<AnimationPlayer>("AnimationPlayer");
                animation_player.set_current_animation(&godot::prelude::StringName::from("pickup"));
                animation_player.play();
            }
            
        });
    }
}