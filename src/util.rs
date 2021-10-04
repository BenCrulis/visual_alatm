use std::error::Error;
use x11rb::protocol::xproto::{Depth, VisualClass, Visualtype};
use crate::display;

pub const TMP_FILE: &str = "/tmp/visual_alarm_description";

pub fn find_visual_with_depth(visuals: &[Depth], depth: u8, visual_class: VisualClass) -> Option<Visualtype> {
    for depth_object in visuals {
        if depth_object.depth == depth {
            for visualtype in &depth_object.visuals {
                if visualtype.class == visual_class {
                    return Some(visualtype.clone())
                }
            }
        }
    }
    None
}


pub fn slice_to_sequence_buffer(slice: &[u8]) -> [u8; 32] {
    let mut buffer = [0; 32];
    for (i, v) in slice.iter().enumerate() {
        buffer[i] = *v;
    }
    buffer
}


pub fn set_remainder(nb_seconds: u64, nb_pulses: u8) -> Result<(), Box<dyn Error>> {
    std::thread::sleep(std::time::Duration::from_secs(nb_seconds));

    let mut display_obj = display::Display::create_and_connect()?;
    display_obj.screen_pulse_effect(nb_pulses, (1.0, 0.0, 0.0), 0.5);
    Ok(())
}