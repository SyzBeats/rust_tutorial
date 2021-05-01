use std::{ error::Error, time::Duration};
use std::io;
use crossterm::{ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, LeaveAlternateScreen}};
use crossterm::terminal::EnterAlternateScreen;
use rusty_audio::Audio;


/**
 * Main function for the programm
 * 
 * is the type used for returning and propagating errors. 
 * It is an enum with the variants, Ok(T), representing success 
 * and containing a value, and Err(E), representing error and containing an error value.
 */
fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("win", "./sounds/explode.wav");
    audio.add("pew", "./sounds/explode.wav");
    audio.add("startup", "./sounds/startup.wav");

    // Execution continues while playback occurs in another thread.
    audio.play("startup");


    // ========== SETUP 
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;

    // this enters the alternative screen, so not the main terminal screen but another "layer" 
    stdout.execute(EnterAlternateScreen)?;

    stdout.execute(Hide)?;


    // ========= GAME LOOP

    // named loop - "gameloop" so it can be accessed

    'gameloop: loop {
        // Input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    
                    // quit the game early
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }

                    // shoot noise
                    KeyCode::Up => {
                        audio.play("pew");
                    }

                    // move noise
                    KeyCode::Left | KeyCode::Right => {
                        audio.play("move");
                    }

                    _ => {
                        // ignore any other case
                    }
                }
            }
        }
    }




    // ========== CLEANUP
    // Block until sounds finish playing
    audio.wait();

    stdout.execute(Show)?;

    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    return Ok(());
}
