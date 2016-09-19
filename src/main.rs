#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

mod note_manager;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent};


widget_ids! {
    struct Ids { master, header, body, footer, text_edit, save_button, reset_button }
}

struct AppState {
    count: u32,
    notes: note_manager::Manager
}

fn main() {
    const WIDTH: u32 = 360;
    const HEIGHT: u32 = 720;

    //get the notes
    let mut my_notes = note_manager::Manager::new();

    if my_notes.notes.len() == 0 {

        let edit_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
            Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
            finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
            fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
            Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
            Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
            magna est, efficitur suscipit dolor eu, consectetur consectetur urna.";

        my_notes.notes.push(edit_text.to_string());
    }

    // Construct the window.
    let mut window: PistonWindow = piston_window::WindowSettings::new("Notes", [WIDTH, HEIGHT])
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_ups(60);

    // Construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();


    let mut state = AppState { count : 0 , notes: my_notes };

    // Poll events from the window.
    while let Some(event) = window.next() {

        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }



        event.update(|_| set_ui(ui.set_widgets(), &ids, &mut state));

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }

}

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids, state: &mut AppState) {
    use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget, Labelable};

    widget::Canvas::new()
        .flow_down(&[
            (ids.header, widget::Canvas::new().color(color::BLUE).length(50.0).pad_bottom(20.0)),
            (ids.body, widget::Canvas::new()),
            (ids.footer, widget::Canvas::new().color(color::BLUE).length(50.0).pad(5.0))
        ])
        .color(color::DARK_CHARCOAL)
        .set(ids.master, ui);

    for _click in widget::Button::new()
        .mid_left_of(ids.footer)
        .w_h(80.0, 30.0)
        .label(&state.count.to_string())
        .set(ids.save_button, ui)
    {
        state.count +=1;
    }

    for _click in widget::Button::new()
        .mid_left_of(ids.footer)
        .right_from(ids.save_button, 5.0)
        .w_h(80.0, 30.0)
        .label("Save")
        .set(ids.reset_button, ui)
    {
        &state.notes.save();
    }

    for edit in widget::TextEdit::new(&state.notes.notes[0])
        .color(color::LIGHT_BLUE)
        .padded_wh_of(ids.body, 20.0)
        .mid_bottom_of(ids.body)
        .line_spacing(2.5)
        .set(ids.text_edit, ui)
    {
        state.notes.notes[0] = edit.to_string();
    }
}
