use std::io::{stdin, stdout, Write};
use std::error::Error;

use bimap::BiMap;
use enigo::{Enigo, Key, KeyboardControllable};
// on windows it appears that, games ignore enigo's key_down for shift
// but games do seem to respond to input bot
use inputbot::KeybdKey::{LShiftKey, LControlKey};
use midir::{MidiInput, Ignore};

mod keys;

fn press_keys(key_presses: &(Option<keys::ModKeys>, char), layout: &keys::Layout, enigo: & mut Enigo) {
    match layout {
        keys::Layout::OnlinePiano => enigo.key_down(Key::Layout(key_presses.1)),
        keys::Layout::FullOnlinePiano => {
            match &key_presses.0 {
                Some(_mod_key) => {
                    enigo.key_down(Key::LShift);
                    enigo.key_down(Key::Layout(key_presses.1));
                    enigo.key_up(Key::LShift);
                }
                None => enigo.key_down(Key::Layout(key_presses.1),)
            }
        },
        keys::Layout::GameLayout => {
            match &key_presses.0 {
                Some(_) => {
                    LShiftKey.press();
                    enigo.key_down(Key::Layout(key_presses.1));
                    LShiftKey.release();
                }
                None => enigo.key_down(Key::Layout(key_presses.1),)
            }
        }, 
        keys::Layout::FullGameLayout => {
            match &key_presses.0 {
                Some(mod_key) => {
                    match mod_key {
                        keys::ModKeys::Shift => {
                            LShiftKey.press();
                            enigo.key_down(Key::Layout(key_presses.1));
                            LShiftKey.release();
                        },
                        keys::ModKeys::Control => {
                            LControlKey.press();
                            enigo.key_down(Key::Layout(key_presses.1));
                            LControlKey.release();
                        }
                    }
                }
                None => enigo.key_down(Key::Layout(key_presses.1),)
            }
        }, 
    }
}

fn release_key(key: &char, enigo: & mut Enigo) {
    enigo.key_up(Key::Layout(key.clone()))
}

fn ask_for_layout() -> Result<keys::Layout, Box<dyn Error>> {
    let layout_names = ["OnlinePiano", "FullOnlinePiano", "GameLayout", "FullGameLayout"];

    println!("Layout Options:");
    for (i, l) in layout_names.iter().enumerate() {
        println!("{}: {}", (i + 1), l);
    };

    print!("Please select keyboard layout: ");
    stdout().flush()?;
    let mut layout_input = String::new();
    stdin().read_line(&mut layout_input)?;
    layout_input = layout_input.trim().parse()?;

    if layout_input == "1" {
        Ok(keys::Layout::OnlinePiano)
    }
    else if layout_input == "2" {
        Ok(keys::Layout::FullOnlinePiano)
    }
    else if layout_input == "3" {
        Ok(keys::Layout::GameLayout)
    }
    else if layout_input == "4" {
        Ok(keys::Layout::FullGameLayout)
    }
    else {
        Err("input is not a valid layout option".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    // various midi constants 
    let note_down_code: u8 = 144;
    let note_up_code: u8 = 128;
    let pedal_code: u8 = 176;        
    let sustain_pedal_no: u8 = 64;  
    let sustain_vel_gate: u8 = 127; // how far the pedal needs to be pressed down to activate (max 127)

    let layout = ask_for_layout()?;

    let mut enigo = Enigo::new();
    let mut pressed_notes = BiMap::<u8, char>::new();
    let mut sustained_notes = BiMap::<u8, char>::new();
    let mut sustain_down = false;

    // setup midi input conection
    // code shamelessly copied from midir doc's examples
    let mut input = String::new();
    
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::All);
    
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!("Choosing the only available input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?)
                     .ok_or("invalid input port selected")?
        }
    };
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |_stamp, message, _| {
        let message_type = message[0];

        if (message_type == pedal_code) & (message[1] == sustain_pedal_no) {
            let velo = message[2];
            if velo >= sustain_vel_gate {
                sustain_down = true;
            }
            else {
                sustain_down = false;
                for (sustained_note_no, sustained_key) in &sustained_notes.clone() {
                    release_key(sustained_key, &mut enigo);
                    pressed_notes.remove_by_left(sustained_note_no);
                    sustained_notes.remove_by_left(sustained_note_no);
                }
            }
        }
        else if message_type == note_down_code {
            let note_no = message[1];

            let key_presses = keys::get_keys(&note_no, &layout).unwrap();
            let held_key = key_presses.1;
            if pressed_notes.contains_right(&held_key) {
                if sustain_down {
                    sustained_notes.remove_by_right(&held_key);
                }
                release_key(&held_key, &mut enigo);
            }
            else {
                pressed_notes.insert(note_no, held_key);
            }
            press_keys(key_presses, &layout, &mut enigo);
        }
        else if message_type == note_up_code {
            let note_no = message[1];

            let key_presses = keys::get_keys(&note_no, &layout).unwrap();
            let held_key = key_presses.1;
            if sustain_down {
                sustained_notes.insert(note_no, held_key);
            }
            else {
                pressed_notes.remove_by_left(&note_no);
                release_key(&held_key, &mut enigo);
            }
        }
    }, ())?;
    
    println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
