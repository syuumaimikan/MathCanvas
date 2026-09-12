use crate::document::Document;

pub trait Command {
    fn execute(&mut self, document: &mut Document);
    fn undo(&mut self, document: &mut Document);
}

#[derive(Default)]
pub struct CommandStack {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl CommandStack {
    pub fn execute_command(&mut self, mut command: Box<dyn Command>, document: &mut Document) {
        command.execute(document);
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, document: &mut Document) {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(document);
            self.redo_stack.push(command);
        }
    }

    pub fn redo(&mut self, document: &mut Document) {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute(document);
            self.undo_stack.push(command);
        }
    }
}
