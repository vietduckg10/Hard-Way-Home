use godot::prelude::*;
use godot::classes::{Area2D, IArea2D, Label, Node2D, Sprite2D};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct TreeTrap {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for TreeTrap {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world! TreeTrap"); // Prints to the Godot console
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("TreeTrap ready!");

        let tree_trap = self.base().clone();
        let mut sprite = self.base().clone().get_node_as::<Sprite2D>("Sprite2D");
        let mut label = self.base().clone().get_node_as::<Label>("Label");

        tree_trap.signals()
            .body_entered()
            .connect(move |body: Gd<Node2D>| {

                if body.is_in_group("player") {
                    godot_print!("Player entered the TreeTrap!");
                    
                    sprite.set_visible(true);
                    label.set_visible(true);
                }
                
            });
    }
}