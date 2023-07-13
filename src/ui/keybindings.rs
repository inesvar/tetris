use piston::Key;

pub fn key_to_string(key: Key) -> String {
    match key {
        Key::A => return "a".to_owned(),
        Key::B => return "b".to_owned(),
        Key::C => return "c".to_owned(),
        Key::D => return "d".to_owned(),
        Key::E => return "e".to_owned(),
        Key::F => return "f".to_owned(),
        Key::G => return "g".to_owned(),
        Key::H => return "h".to_owned(),
        Key::I => return "i".to_owned(),
        Key::J => return "j".to_owned(),
        Key::K => return "k".to_owned(),
        Key::L => return "l".to_owned(),
        Key::M => return "m".to_owned(),
        Key::N => return "n".to_owned(),
        Key::O => return "o".to_owned(),
        Key::P => return "p".to_owned(),
        Key::Q => return "q".to_owned(),
        Key::R => return "r".to_owned(),
        Key::S => return "s".to_owned(),
        Key::T => return "t".to_owned(),
        Key::U => return "u".to_owned(),
        Key::V => return "v".to_owned(),
        Key::W => return "w".to_owned(),
        Key::X => return "x".to_owned(),
        Key::Y => return "y".to_owned(),
        Key::Z => return "z".to_owned(),
        Key::NumPad0 => return "numpad0".to_owned(),
        Key::NumPad1 => return "numpad1".to_owned(),
        Key::NumPad2 => return "numpad2".to_owned(),
        Key::NumPad3 => return "numpad3".to_owned(),
        Key::NumPad4 => return "numpad4".to_owned(),
        Key::NumPad5 => return "numpad5".to_owned(),
        Key::NumPad6 => return "numpad6".to_owned(),
        Key::NumPad7 => return "numpad7".to_owned(),
        Key::NumPad8 => return "numpad8".to_owned(),
        Key::NumPad9 => return "numpad9".to_owned(),
        Key::Space => return "space".to_owned(),
        Key::Left => return "left".to_owned(),
        Key::Right => return "right".to_owned(),
        Key::Up => return "up".to_owned(),
        Key::Down => return "down".to_owned(),
        _ => unreachable!(),
    }
}

pub fn keys_to_string(keys: &[Key]) -> String {
    let mut s = String::new();
    for key in keys {
        s.push_str(&key_to_string(*key));
        s.push_str(", ");
    }
    s
}
