use crate::text_box::TextBox;


pub struct TextBoxNode {
    tbox: TextBox
}

impl TextBoxNode {
    pub fn new(tbox: TextBox) -> Self {
        Self { tbox }
    }
}