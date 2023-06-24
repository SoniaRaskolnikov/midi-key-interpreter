use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug)]
pub enum Layout {
    OnlinePiano,     // just the letters and numbers
    FullOnlinePiano, // letters and numbers but with shift for accidentals
    GameLayout,      // same as FullOnlinePiano but using inputbot instead of enigo
    FullGameLayout   // similar logic as FullOnlinePiano and GameLayout but with additional notes triggered with control
}

#[derive(Debug)]
pub enum ModKeys {
    Shift,
    Control
}

// if the note is not within the range of the keyboard layout chosen,
// the note will be transposed by octave so that it is in range of the chosen keyboard layout
fn transpose_octave(note_no: &u8, min: u8, max: u8) -> u8 {
    if note_no < &min {
        return note_no + ((min - note_no) as f64 / 12.).ceil() as u8 * 12
    }
    else if note_no > &max {
        return note_no - ((note_no - max) as f64 / 12.).ceil() as u8 * 12
    }
    note_no.clone()
}

lazy_static! {
    static ref ONLINE_PIANO_MAP: HashMap<u8, (Option<ModKeys>, char)> = {
        HashMap::from([
            (48, (None, 'q')),
            (49, (None, '2')),
            (50, (None, 'w')),
            (51, (None, '3')),
            (52, (None, 'e')),
            (53, (None, 'r')),
            (54, (None, '5')),
            (55, (None, 't')),
            (56, (None, '6')),
            (57, (None, 'y')),
            (58, (None, '7')),
            (59, (None, 'u')),
            (60, (None, 'i')),
            (61, (None, '9')),
            (62, (None, 'o')),
            (63, (None, '0')),
            (64, (None, 'p')),
            (65, (None, 'z')),
            (66, (None, 's')),
            (67, (None, 'x')),
            (68, (None, 'd')),
            (69, (None, 'c')),
            (70, (None, 'f')),
            (71, (None, 'v')),
            (72, (None, 'b')),
            (73, (None, 'h')),
            (74, (None, 'n')),
            (75, (None, 'j')),
            (76, (None, 'm')),
            (77, (None, ',')),
            (78, (None, 'l')),
            (79, (None, '.')),
            (80, (None, ';')),
            (81, (None, '/')),
            (82, (None, '\u{0027}')) // (single quote)
        ])
    };
    static ref SHIFT_PIANO_MAP: HashMap<u8, (Option<ModKeys>, char)> = {
        HashMap::from([
            (36, (None, '1')),
            (37, (Some(ModKeys::Shift), '1')),
            (38, (None, '2')),
            (39, (Some(ModKeys::Shift), '2')),
            (40, (None, '3')),
            (41, (None, '4')),
            (42, (Some(ModKeys::Shift), '4')),
            (43, (None, '5')),
            (44, (Some(ModKeys::Shift), '5')),
            (45, (None, '6')),
            (46, (Some(ModKeys::Shift), '6')),
            (47, (None, '7')),
            (48, (None, '8')),
            (49, (Some(ModKeys::Shift), '8')),
            (50, (None, '9')),
            (51, (Some(ModKeys::Shift), '9')),
            (52, (None, '0')),
            (53, (None, 'q')),
            (54, (Some(ModKeys::Shift), 'q')),
            (55, (None, 'w')),
            (56, (Some(ModKeys::Shift), 'w')),
            (57, (None, 'e')),
            (58, (Some(ModKeys::Shift), 'e')),
            (59, (None, 'r')),
            (60, (None, 't')),
            (61, (Some(ModKeys::Shift), 't')),
            (62, (None, 'y')),
            (63, (Some(ModKeys::Shift), 'y')),
            (64, (None, 'u')),
            (65, (None, 'i')),
            (66, (Some(ModKeys::Shift), 'i')),
            (67, (None, 'o')),
            (68, (Some(ModKeys::Shift), 'o')),
            (69, (None, 'p')),
            (70, (Some(ModKeys::Shift), 'p')),
            (71, (None, 'a')),
            (72, (None, 's')),
            (73, (Some(ModKeys::Shift), 's')),
            (74, (None, 'd')),
            (75, (Some(ModKeys::Shift), 'd')),
            (76, (None, 'f')),
            (77, (None, 'g')),
            (78, (Some(ModKeys::Shift), 'g')),
            (79, (None, 'h')),
            (80, (Some(ModKeys::Shift), 'h')),
            (81, (None, 'j')),
            (82, (Some(ModKeys::Shift), 'j')),
            (83, (None, 'k')),
            (84, (None, 'l')),
            (85, (Some(ModKeys::Shift), 'l')),
            (86, (None, 'z')),
            (87, (Some(ModKeys::Shift), 'z')),
            (88, (None, 'x')),
            (89, (None, 'c')),
            (90, (Some(ModKeys::Shift), 'c')),
            (91, (None, 'v')),
            (92, (Some(ModKeys::Shift), 'v')),
            (93, (None, 'b')),
            (94, (Some(ModKeys::Shift), 'b')),
            (95, (None, 'n')),
            (96, (None, 'm'))
        ])
    };
    static ref FULL_GAME_MAP : HashMap<u8, (Option<ModKeys>, char)> = {
        HashMap::from([
            (21, (Some(ModKeys::Control), '1')),
            (22, (Some(ModKeys::Control), '2')),
            (23, (Some(ModKeys::Control), '3')),
            (24, (Some(ModKeys::Control), '4')),
            (25, (Some(ModKeys::Control), '5')),
            (26, (Some(ModKeys::Control), '6')),
            (27, (Some(ModKeys::Control), '7')),
            (28, (Some(ModKeys::Control), '8')),
            (29, (Some(ModKeys::Control), '9')),
            (30, (Some(ModKeys::Control), '0')),
            (31, (Some(ModKeys::Control), 'q')),
            (32, (Some(ModKeys::Control), 'w')),
            (33, (Some(ModKeys::Control), 'e')),
            (34, (Some(ModKeys::Control), 'r')),
            (35, (Some(ModKeys::Control), 't')),
            (36, (None, '1')),
            (37, (Some(ModKeys::Shift), '1')),
            (38, (None, '2')),
            (39, (Some(ModKeys::Shift), '2')),
            (40, (None, '3')),
            (41, (None, '4')),
            (42, (Some(ModKeys::Shift), '4')),
            (43, (None, '5')),
            (44, (Some(ModKeys::Shift), '5')),
            (45, (None, '6')),
            (46, (Some(ModKeys::Shift), '6')),
            (47, (None, '7')),
            (48, (None, '8')),
            (49, (Some(ModKeys::Shift), '8')),
            (50, (None, '9')),
            (51, (Some(ModKeys::Shift), '9')),
            (52, (None, '0')),
            (53, (None, 'q')),
            (54, (Some(ModKeys::Shift), 'q')),
            (55, (None, 'w')),
            (56, (Some(ModKeys::Shift), 'w')),
            (57, (None, 'e')),
            (58, (Some(ModKeys::Shift), 'e')),
            (59, (None, 'r')),
            (60, (None, 't')),
            (61, (Some(ModKeys::Shift), 't')),
            (62, (None, 'y')),
            (63, (Some(ModKeys::Shift), 'y')),
            (64, (None, 'u')),
            (65, (None, 'i')),
            (66, (Some(ModKeys::Shift), 'i')),
            (67, (None, 'o')),
            (68, (Some(ModKeys::Shift), 'o')),
            (69, (None, 'p')),
            (70, (Some(ModKeys::Shift), 'p')),
            (71, (None, 'a')),
            (72, (None, 's')),
            (73, (Some(ModKeys::Shift), 's')),
            (74, (None, 'd')),
            (75, (Some(ModKeys::Shift), 'd')),
            (76, (None, 'f')),
            (77, (None, 'g')),
            (78, (Some(ModKeys::Shift), 'g')),
            (79, (None, 'h')),
            (80, (Some(ModKeys::Shift), 'h')),
            (81, (None, 'j')),
            (82, (Some(ModKeys::Shift), 'j')),
            (83, (None, 'k')),
            (84, (None, 'l')),
            (85, (Some(ModKeys::Shift), 'l')),
            (86, (None, 'z')),
            (87, (Some(ModKeys::Shift), 'z')),
            (88, (None, 'x')),
            (89, (None, 'c')),
            (90, (Some(ModKeys::Shift), 'c')),
            (91, (None, 'v')),
            (92, (Some(ModKeys::Shift), 'v')),
            (93, (None, 'b')),
            (94, (Some(ModKeys::Shift), 'b')),
            (95, (None, 'n')),
            (96, (None, 'm')),
            (97, (Some(ModKeys::Control), 'v')),
            (98, (Some(ModKeys::Control), 'u')),
            (99, (Some(ModKeys::Control), 'i')),
            (100, (Some(ModKeys::Control), 'o')),
            (101, (Some(ModKeys::Control), 'p')),
            (102, (Some(ModKeys::Control), 'a')),
            (103, (Some(ModKeys::Control), 's')),
            (104, (Some(ModKeys::Control), 'd')),
            (105, (Some(ModKeys::Control), 'f')),
            (106, (Some(ModKeys::Control), 'g')),
            (107, (Some(ModKeys::Control), 'h')),
            (108, (Some(ModKeys::Control), 'j'))
        ])
    };
}

pub fn get_keys(note_no: &u8, layout: &Layout) -> Option<&'static (Option<ModKeys>, char)> {
    match layout {
        Layout::OnlinePiano => {
            let transposed_note = transpose_octave(&note_no, 48, 82);
            let note_no = &transposed_note;
            match ONLINE_PIANO_MAP.get(note_no) {
                Some(keys) => return Some(keys),
                None => {
                    println!("unable to find note {} for {:?} layout", note_no, layout);
                    return None
                }
            }
        }
        Layout::FullOnlinePiano | Layout::GameLayout => {
            
            let transposed_note = transpose_octave(&note_no, 36, 96);
            let note_no = &transposed_note;
            match SHIFT_PIANO_MAP.get(note_no) {
                Some(keys) => return Some(keys),
                None => {
                    println!("unable to find note {} for {:?} layout", note_no, layout);
                    return None
                }
            }
        }
        Layout::FullGameLayout => {
            let transposed_note = transpose_octave(&note_no, 21, 108);
            let note_no = &transposed_note;
            match FULL_GAME_MAP.get(note_no) {
                Some(keys) => return Some(keys),
                None => {
                    println!("unable to find note {} for {:?} layout", note_no, layout);
                    return None
                }
            }
        }
    }
}