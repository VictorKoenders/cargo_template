#[derive(Default, Debug)]
pub struct Transaction {
    actions: Vec<TransactionAction>,
}

impl Transaction {
    pub fn replace_in_files<T: Into<String>, U: Into<String>>(&mut self, key: T, value: U) {
        self.actions.push(TransactionAction::ReplaceInFiles {
            key: key.into(),
            value: value.into(),
        });
    }

    pub fn replace_in_file<T: Into<String>, U: Into<String>, V: Into<String>>(&mut self, file: T, key: U, value: V) {
        self.actions.push(TransactionAction::ReplaceInFile {
            file: file.into(),
            key: key.into(),
            value: value.into(),
        });
    }

    pub fn create_file<T: Into<String>, U: Into<String>>(&mut self, file: T, value: U) {
        self.actions.push(TransactionAction::CreateFile {
            file: file.into(),
            value: value.into(),
        });
    }
}

#[derive(Debug)]
pub enum TransactionAction {
    ReplaceInFile {
        file: String,
        key: String,
        value: String
    },
    ReplaceInFiles {
        key: String,
        value: String,
    },
    CreateFile {
        file: String,
        value: String,
    }
}