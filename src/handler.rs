use crate::app::App;
use crate::ToChar;

pub fn handle_key_press(key_code: i32, app: &mut App) {
    match key_code.to_char() {
        'q' => {
            app.run = false;
        }
        'j' => app.move_cursor(false),
        'k' => app.move_cursor(true),
        'l' => app.toggle_list(),
        'h' => app.toggle_list(),
        ' ' => app.toggle_task(),
        _ => {}
    }
}
