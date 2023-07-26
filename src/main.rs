use macroquad::prelude::*;
use std::time::Instant;

mod state;
use state::{game::Signal, State};

fn window_conf() -> Conf {
    Conf {
        window_title: "Five-tris".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();
    let mut new_game = true;

    let mut last_frame = Instant::now();
    loop {
        let delta = last_frame.elapsed();

        if !new_game {
            match state.game.update(delta) {
                None => {}
                Some(signal) => match signal {
                    Signal::Quit => {
                        println!("Exiting");
                        break;
                    }
                    Signal::Restart => {
                        println!("Restarting game");
                        state = State::new();
                        new_game = true;
                        continue;
                    }
                },
            };
        } else {
            new_game = false;
        }

        state.game.render();

        last_frame = Instant::now();
        next_frame().await
    }
}
