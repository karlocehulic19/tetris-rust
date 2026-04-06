pub struct Board {
    blocks: [[u8; 10]; 10],
}

impl Board {
    pub fn new() -> Self {
        return Self {
            blocks: [[0; 10]; 10],
        };
    }

    pub fn to_vec_strings(self) -> [String; 10] {
        let mut strings: [String; 10] = Default::default();

        for (row_i, row) in self.blocks.iter().enumerate() {
            let mut string_row: String = "".to_string();
            for cell in row {
                string_row += &cell.to_string();
            }
            strings[row_i] = string_row;
        }

        return strings;
    }

    pub fn place_block(&mut self) {
        self.blocks[3][3] = 1;
    }
}
