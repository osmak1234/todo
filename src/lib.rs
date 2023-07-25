pub mod app;
pub mod event;
pub mod handler;
pub mod ui;

trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for i32 {
    fn to_char(&self) -> char {
        char::from_u32(*self as u32).unwrap()
    }
}
#[derive(Clone, Debug)]
pub struct Block {
    pub text: String,
}

#[allow(clippy::comparison_chain)]
impl Block {
    pub fn new(text: String) -> Block {
        let needed_width = text.lines().map(|ln| ln.len()).max().unwrap_or(0);

        let padded_text = text
            .lines()
            .map(|ln| format!("{:width$}", ln, width = needed_width))
            .collect::<Vec<String>>()
            .join("\n");

        Block { text: padded_text }
    }

    /// # WARNING
    /// this fuction thinks that the 2 blocks are valied i.e. all lines have the same width
    pub fn join_blocks(&mut self, block: Block) {
        self.text = match (self.text.is_empty(), block.text.is_empty()) {
            // edge cases
            (true, true) => "".to_string(),
            (true, false) => block.text,
            //good input
            (false, true) => self.text.to_string(),
            (false, false) => {
                let mut lines1: Vec<String> =
                    self.text.split('\n').map(|ln| ln.to_string()).collect();
                let mut lines2: Vec<String> =
                    block.text.split('\n').map(|ln| ln.to_string()).collect();

                let ln_1_count = lines1.len();
                let ln_2_count = lines2.len();
                let s3: Vec<String>;
                if ln_2_count == ln_1_count {
                    s3 = lines1
                        .into_iter()
                        .zip(lines2.into_iter())
                        .map(|(str1, str2)| str1 + " " + &str2)
                        .collect();
                } else if ln_2_count > ln_1_count {
                    let needed_width = lines2.clone().get(0).unwrap().chars().count();

                    while lines1.len() != lines2.len() {
                        lines1.push(format!("{:width$}", "", width = needed_width));
                    }

                    s3 = lines1
                        .into_iter()
                        .zip(lines2.into_iter())
                        .map(|(str1, str2)| str1 + " " + &str2)
                        .collect();
                } else {
                    let needed_width = lines1.clone().get(0).unwrap().chars().count();

                    while lines2.len() != lines1.len() {
                        let temp = format!("{:width$}", "", width = needed_width);
                        lines2.push(temp);
                    }

                    s3 = lines2
                        .into_iter()
                        .zip(lines1.into_iter())
                        .map(|(str1, str2)| str1 + " " + &str2)
                        .collect();
                }
                s3.join("\n")
            }
        };
    }

    pub fn pad_block_right(&mut self, width: usize) {
        let lines: Vec<&str> = self.text.split('\n').collect();
        let needed_width = lines.first().unwrap().chars().count();

        self.text = lines
            .iter()
            .map(|ln| format!("{:width$}", ln, width = needed_width + width))
            .collect::<Vec<_>>()
            .join("\n");
    }

    pub fn add_char_right(&mut self, ch: char) {
        let lines: Vec<&str> = self.text.split('\n').collect();
        self.text = lines
            .iter()
            .map(|ln| format!("{}{}", ln, ch))
            .collect::<Vec<_>>()
            .join("\n");
    }

    pub fn render(&self) -> String {
        self.text.clone()
    }

    pub fn pad_block_bottom(&mut self, height: usize) {
        let mut lines: Vec<String> = self.text.split('\n').map(|ln| ln.to_string()).collect();
        let needed_width = lines.first().unwrap().chars().count();

        for _ in 0..height {
            let temp = format!("{:width$}", "", width = needed_width);
            lines.push(temp);
        }
        self.text = lines.join("\n");
    }
}
