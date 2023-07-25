use std::error::Error;

extern crate ncurses;

use ncurses::*;
use todo::app::App;
use todo::event::{Event, EventSink};
use todo::handler::handle_key_press;
use todo::ui::render_ui;

fn main() -> Result<(), Box<dyn Error>> {
    /* Setup ncurses. */
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    let event_sink = EventSink {
        que: vec![Event::Draw],
    };
    let mut app = App::new(event_sink);

    while app.run {
        if let Some(val) = app.event_sink.next_q() {
            match val {
                Event::Draw => {
                    clear();
                    let to_render = render_ui(&app);

                    let lines = to_render
                        .split('\n')
                        .map(|ln| ln.to_string())
                        .collect::<Vec<String>>();
                    let mut tasks: Vec<String> = vec![];

                    lines.iter().for_each(|ln| {
                        ln.to_owned().split('|').for_each(|item| {
                            tasks.push(item.to_string());
                        })
                    });

                    for (i, task) in tasks.iter().enumerate() {
                        if i % 2 == 0 {
                            if !app.list && app.item * 2 == i as i32 {
                                attron(A_BOLD());
                                addstr(task);
                                attroff(A_BOLD());
                                addstr(" |");
                            } else {
                                addstr(task);
                                addstr(" |");
                            }
                        } else if app.list && app.item * 2 - 1 == i as i32 {
                            attron(A_BOLD());
                            addstr(&format!("{} \n", task));
                            attroff(A_BOLD());
                        } else {
                            addstr(&format!("{} \n", task));
                        }
                    }

                    refresh();
                }
                Event::KeyPress(char) => handle_key_press(char, &mut app),
                Event::EmptyTick => todo!(),
            }
        } else {
            timeout(100); // Sets the delay for getch()
            let ch = getch();
            if ch != -1 {
                app.event_sink.push(Event::KeyPress(ch))
            }
        }
    }

    endwin();
    Ok(())
}
