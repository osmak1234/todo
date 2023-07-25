use crate::app::App;
use crate::Block;
use std::cmp::Ordering;

pub fn render_ui(app: &App) -> String {
    let uncompleted_tasks = app.uncompleted.clone();
    let completed_tasks = app.completed.clone();

    let mut uncompleted_block = Block::new(
        uncompleted_tasks
            .iter()
            .map(|item| format!("- [ ] {} ", item.name.clone()))
            .collect::<Vec<String>>()
            .join("\n"),
    );
    let mut completed_block = Block::new(
        completed_tasks
            .iter()
            .map(|item| format!("- [x] {} ", item.name.clone()))
            .collect::<Vec<String>>()
            .join("\n"),
    );

    match Ord::cmp(&uncompleted_tasks.len(), &completed_tasks.len()) {
        Ordering::Less => {
            let difference = completed_tasks.len() - uncompleted_tasks.len();
            uncompleted_block.pad_block_bottom(difference);
            uncompleted_block.pad_block_right(3);
            uncompleted_block.add_char_right('|');
            uncompleted_block.pad_block_right(3);
            uncompleted_block.join_blocks(completed_block);
        }
        Ordering::Equal => {
            uncompleted_block.pad_block_right(3);
            uncompleted_block.add_char_right('|');
            uncompleted_block.pad_block_right(3);
            uncompleted_block.join_blocks(completed_block);
        }
        Ordering::Greater => {
            let difference = uncompleted_tasks.len() - completed_tasks.len();
            completed_block.pad_block_bottom(difference);
            uncompleted_block.pad_block_right(3);
            uncompleted_block.add_char_right('|');
            uncompleted_block.pad_block_right(3);
            uncompleted_block.join_blocks(completed_block);
        }
    }
    uncompleted_block.render()
}
