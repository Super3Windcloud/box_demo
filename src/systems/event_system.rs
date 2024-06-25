use crate::{
    audio::AudioStore,
    components::*,
    events::{BoxPlacedOnSpot, EntityMoved, Event},  // 引入事件类型
    resources::EventQueue,// 事件队列
};
use specs::{Entities, Join, ReadStorage, System, Write};
// 引入系统所需的资源
use std::collections::HashMap;

pub struct EventSystem<'a> {
    pub context: &'a mut ggez::Context,
}

// System implementation
impl<'a> System<'a> for EventSystem<'a> {
    // Data
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut audio_store, entities, boxes, box_spots, positions) = data;

        let mut new_events = Vec::new();

        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);

            match event {
                Event::PlayerHitObstacle => {
                    // play sound here
                    audio_store.play_sound(self.context, &"wall".to_string());
                }
                Event::EntityMoved(EntityMoved { id }) => {
                    // An entity was just moved, check if it was a box and fire
                    // more events if it's been moved on a spot.
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_with_positions: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|t| ((t.1.x, t.1.y), t.0))
                                .collect::<HashMap<_, _>>();

                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            // Check if there is a spot on this position, and if there
                            // is if it's the correct or incorrect type
                            if let Some(box_spot) =
                                box_spots_with_positions.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: (box_spot.colour == the_box.colour),
                                })); // 产生新的事件
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    // play sound here
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };

                    audio_store.play_sound(self.context, &sound.to_string()) // 调用音效播放器播放音效
                }
            }
        }

        event_queue.events.append(&mut new_events);// 将新产生的事件添加到事件队列中
    }
}
