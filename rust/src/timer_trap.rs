use godot::prelude::*;
use godot::classes::{Area2D, IArea2D, Node2D, Timer, CollisionShape2D};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct TimerTrap {
    #[base]
    base: Base<Area2D>,
    #[export]
    vector: Vector2
}

#[godot_api]
impl IArea2D for TimerTrap {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world! TimerTrap"); // Prints to the Godot console
        Self {
            base,
            vector: Vector2::new(0.0, 0.0)
        }
    }

    fn ready(&mut self) {
        godot_print!("TimerTrap ready!");
        let mut timer_trap = self.base().clone();
        let timer = self.base().clone()
        .get_node_as::<Timer>("Timer");
        let mut timer_for_body = timer.clone();
        let mut timer_killer = timer.clone();
        let mut trigger_area = self.base().clone().get_node_as::<CollisionShape2D>("CollisionShape2D");
        let vector = self.vector;

        timer_trap.signals()
            .body_entered()
            .connect(move |body: Gd<Node2D>| {

                if body.is_in_group("player") {
                    trigger_area.queue_free();
                    godot_print!("Player entered the TimerTrap!");
                    if timer_for_body.is_stopped() {
                        timer_for_body.start();
                    }
                }
                
            });

        timer.signals()
            .timeout()
            .connect(move || {
                godot_print!("Timeout! Trap active!");
                
                let mut tween = timer_trap.create_tween();

                tween
                    .tween_property(
                        &timer_trap,
                        "position:y",
                        &vector.y.to_variant(),
                        0.7,
                    )
                    .as_relative();

                tween
                    .tween_property(
                        &timer_trap,
                        "position:x",
                        &vector.x.to_variant(),
                        0.7,
                    )
                    .as_relative();
                timer_killer.queue_free();
            });
    }
}
