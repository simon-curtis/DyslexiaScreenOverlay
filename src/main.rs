use std::collections::HashMap;

use gtk::prelude::*;
use gtk::{Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Dyslexia Overlay");
    window.set_fullscreen(true);
    window.set_override_redirect(true);

    // Create a drawing area to display the colors
    let drawing_area = gtk::DrawingArea::new();
    window.add(&drawing_area);

    // Connect the draw signal to the callback
    drawing_area.connect_draw(draw_callback);

    // Show the window and start the main GTK loop
    window.show_all();
    gtk::main();
}

fn draw_callback(drawing_area: &gtk::DrawingArea, cr: &cairo::Context) -> gtk::Inhibit {
    let mut colors: HashMap<String, [u8; 3]> = HashMap::new();

    // Add the white and green colors to the map
    colors.insert("white".to_string(), [255, 255, 255]);
    colors.insert("green".to_string(), [0, 170, 0]);

    // Replace all white pixels with green
    for color in colors.values_mut() {
        if color == colors["white"] {
            *color = colors["green"];
        }
    }

    // Draw the colors on the screen
    for color in colors.values() {
        cr.set_source_rgb(color[0] as f64 / 255.0, color[1] as f64 / 255.0, color[2] as f64 / 255.0);
        cr.paint();
    }

    gtk::Inhibit(false)
}
