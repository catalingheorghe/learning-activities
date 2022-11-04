use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    // sfx is one-time; music loops, plus it has a volume (full volume 1.0)
    //game.game_state_mut().audio_manager.play_sfx(SfxPreset::Jingle1);
    //game.game_state_mut().audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    // "actor" - something you can see, you can interact with
    let actor = game
        .game_state_mut()
        .add_actor("player", ActorPreset::RacingCarGreen);
    actor.translation = Vec2::new(-100.0, -100.0);
    actor.rotation = std::f32::consts::FRAC_PI_2;
    actor.scale = 3.0;
    actor.layer = 0.0; // note: default

    let actor2 = game
        .game_state_mut()
        .add_actor("player2", ActorPreset::RacingCarBlue);
    actor2.translation = Vec2::new(-50.0, -100.0);
    actor2.rotation = std::f32::consts::FRAC_PI_2;
    actor2.scale = 3.0;
    actor2.layer = 1.0;

    let text = game
        .game_state_mut()
        .add_text_actor("message", "This is a text actor");
    text.layer = 100.0;
    text.translation.y = 200.0;

    // last thing in main, takes a closure
    game.run(logic);
}

// a structure, GameState, holds all the information
fn logic(game_state: &mut GameState) {
    let player = game_state.actors.get_mut("player").unwrap();
    // we have a mutable reference to an actor
    // we know it's there, so just unwrap the result, don't handle the error
    // actors is a hash map with the labels as keys

    for event in game_state.keyboard_events.drain(..) {
        // the keyboard_events is in game_sate, we don't want to take ownership
        // of it, aka move it, only take its content - drain(..) -> draining
        // iterator
        
        if let KeyboardInput {
            scan_code: _,
            key_code: Some(key_code),
            state: ElementState::Pressed,
        } = event
        {
            match key_code {
                KeyCode::A | KeyCode::Left => player.translation.x -= 10.0,
                KeyCode::D | KeyCode::Right => player.translation.x += 10.0,
                _ => {}
            }
        }
    }

    for _event in game_state.mouse_button_events.iter() {
        // iterator over immutable references; leaves them there if you want
        // to look at them again (in the next frame they are flushed)
        player.scale *= 1.1;
    }

    let player2 = game_state.actors.get_mut("player2").unwrap();
    for event in game_state.cursor_moved_events.iter() {
        player2.translation = event.position;
    }
}

// collisions
//
// actor 'collision' must be set to true (default fals)
// in logic - 'collision_events'
//   State - begin or end
//   Pair - actors involved
// see example collisions
//
// cargo run --release --example collisions
//

// level creator
//
// place sprites - see instructions
// press Z and it will spit out the code for all the stuff
//



