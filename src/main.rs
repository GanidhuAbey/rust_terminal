extern crate sfml;

use sfml::graphics::{RenderWindow, Color, RenderTarget, Text, Font};
use sfml::window::{Style, Event, Key};

fn main() {
    //create terminal window
    let mut terminal = RenderWindow::new(
        (1200, 400),
        ">Terminal",
        Style::CLOSE,
        &Default::default()
    );
    let mut run_command = false;
    let mut command = String::from(">>");
    let font = Font::from_file("resources/sansation.ttf").unwrap();
    //define text
    let mut text = Text::new(&command, &font, 12);
    text.set_fill_color(&Color::MAGENTA);
    text.set_outline_color(&Color::BLACK);
    text.set_outline_thickness(5.0);
    //loop till terminal is closed
    while terminal.is_open() {
        let event = terminal.poll_event();
        terminal.set_active(true);
        terminal.clear(&Color::rgb(20, 20, 15));
        if event != None {
            match event.unwrap() {
                //close condition
                Event::KeyPressed {code, ..} => {
                    if code == Key::Escape {
                        terminal.close();
                    }
                }
                Event::TextEntered {unicode} => {
                    //check if character is a special input
                    if unicode == 0xD as char {
                        run_command = true;
                        println!("command has been inputed");
                    }
                    else if unicode == 0x08 as char && command.len() > 2 {
                        command = command[0..command.len() - 1].to_string();
                    }
                    //add new character to end command
                    else {
                        command.push(unicode);
                    }
                    text.set_string(&command);
                }
                _ => (),
            }
        }
        terminal.draw(&text);
        terminal.display()
    }
}
