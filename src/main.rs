extern crate sfml;

use sfml::graphics::{RenderWindow, Color, RenderTarget, Text, Font};
use sfml::window::{Style, Event, Key};

use std::process::Command;
use std::io::{self, Write, ErrorKind};

fn main() {
    //create terminal window
    let mut terminal = RenderWindow::new(
        (1200, 400),
        ">Terminal",
        Style::CLOSE,
        &Default::default()
    );
    //-----
    let mut run_command = false;
    //define text
    let mut command = String::from(">>");
    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let mut text = Text::new(&command, &font, 12);
    text.set_fill_color(&Color::MAGENTA);
    text.set_outline_color(&Color::BLACK);
    text.set_outline_thickness(5.0);
    //-----
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
                    else if unicode != 0x08 as char {
                        command.push(unicode);
                    }
                    text.set_string(&command);
                }
                _ => (),
            }
        }
        let mut output = String::new();
        if run_command {
            output = command_run(&command);
            run_command = false;
            text.set_string(&output);
        }

        terminal.draw(&text);
        terminal.display()
    }
}

fn command_run(input: &String) -> String {
    let command = input[2..input.len()].to_string();
    println!("should print before error");
    let child = Command::new(command).output();
    println!("should not print");

    match child.is_err() {
        true => {
            
            let message = error_message(child.unwrap_err().kind());
            println!("ERROR: {}", message);
        },
        false => io::stdout().write_all(&child.unwrap().stdout).unwrap(),
    }
    
    String::from("ok")
}

fn error_message(kind: ErrorKind) -> String {
    match kind {
        ErrorKind::NotFound => return String::from("The command or entity searched for was not found"),
        _ => return String::from("An unexpected error has occured..."),
    }
}