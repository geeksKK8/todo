use serde::{Deserialize, Serialize};

use crate::Result;
#[derive(Deserialize, Debug)]
pub struct TodoList {
    list: Vec<TODO>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TODO {
    id: u32,
    title: String,
    description: String,
    pub done: bool,
}
impl TodoList {
    pub fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    pub fn load(file_path: &std::path::PathBuf) -> Result<TodoList> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let list = serde_json::from_reader(reader)?;
        Ok(TodoList { list })
    }

    pub fn add(&mut self, title: String, description: String) {
        let id = self.list.len() as u32;
        let todo = TODO {
            id,
            title,
            description,
            done: false,
        };
        self.list.push(todo);
    }

    pub fn get(&self, id: u32) -> Option<&TODO> {
        self.list.iter().find(|todo| todo.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut TODO> {
        self.list.iter_mut().find(|todo| todo.id == id)
    }

    pub fn get_all(&self) -> &Vec<TODO> {
        &self.list
    }

    pub fn remove(&mut self, id: u32) -> Option<TODO> {
        let index = self.list.iter().position(|todo| todo.id == id);
        match index {
            Some(index) => Some(self.list.remove(index)),
            None => None,
        }
    }

    pub fn mark_done(&mut self, id: u32) -> Option<&TODO> {
        let todo = self.get_mut(id);
        match todo {
            Some(todo) => {
                todo.done = true;
                Some(todo)
            }
            None => None,
        }
    }

    pub fn mark_undone(&mut self, id: u32) -> Option<&TODO> {
        let todo = self.get_mut(id);
        match todo {
            Some(todo) => {
                todo.done = false;
                Some(todo)
            }
            None => None,
        }
    }

    pub fn save(&self, file_path: &std::path::PathBuf) -> Result<()> {
        let file = std::fs::File::create(file_path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self.list)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        let file_path = PathBuf::from("test.json");
        todo_list.save(&file_path).unwrap();
        let todo_list = TodoList::load(&file_path).unwrap();
        assert_eq!(todo_list.list.len(), 1);
        assert_eq!(todo_list.list[0].id, 0);
        assert_eq!(todo_list.list[0].title, "title");
        assert_eq!(todo_list.list[0].description, "description");
        assert_eq!(todo_list.list[0].done, false);
        std::fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_add() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        assert_eq!(todo_list.list.len(), 1);
        assert_eq!(todo_list.list[0].id, 0);
        assert_eq!(todo_list.list[0].title, "title");
        assert_eq!(todo_list.list[0].description, "description");
        assert_eq!(todo_list.list[0].done, false);
    }

    #[test]
    fn test_get() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        let todo = todo_list.get(0);
        assert_eq!(todo.is_some(), true);
        let todo = todo.unwrap();
        assert_eq!(todo.id, 0);
        assert_eq!(todo.title, "title");
        assert_eq!(todo.description, "description");
        assert_eq!(todo.done, false);
    }

    #[test]
    fn test_get_mut() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        let todo = todo_list.get_mut(0);
        assert_eq!(todo.is_some(), true);
        let todo = todo.unwrap();
        assert_eq!(todo.id, 0);
        assert_eq!(todo.title, "title");
        assert_eq!(todo.description, "description");
        assert_eq!(todo.done, false);
    }

    #[test]
    fn test_remove() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        let todo = todo_list.remove(0);
        assert_eq!(todo.is_some(), true);
        let todo = todo.unwrap();
        assert_eq!(todo.id, 0);
        assert_eq!(todo.title, "title");
        assert_eq!(todo.description, "description");
        assert_eq!(todo.done, false);
        assert_eq!(todo_list.list.len(), 0);
    }

    #[test]
    fn test_mark_done() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        let todo = todo_list.mark_done(0);
        assert_eq!(todo.is_some(), true);
        let todo = todo.unwrap();
        assert_eq!(todo.id, 0);
        assert_eq!(todo.title, "title");
        assert_eq!(todo.description, "description");
        assert_eq!(todo.done, true);
    }

    #[test]
    fn test_mark_undone() {
        let mut todo_list = TodoList::new();
        todo_list.add("title".to_string(), "description".to_string());
        let todo = todo_list.mark_undone(0);
        assert_eq!(todo.is_some(), true);
        let todo = todo.unwrap();
        assert_eq!(todo.id, 0);
        assert_eq!(todo.title, "title");
        assert_eq!(todo.description, "description");
        assert_eq!(todo.done, false);
    }
}
