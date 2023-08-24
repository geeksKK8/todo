use crate::args::{AddArgs, DoneArgs, GetArgs, RemoveArgs, UndoneArgs};
use crate::todo::TodoList;
use crate::Result;
pub fn add(args: AddArgs) -> Result<()> {
    if args.file_path.exists() {
        let mut todo_list = TodoList::load(&args.file_path)?;
        todo_list.add(args.title, args.description);
        todo_list.save(&args.file_path)?;
    } else {
        let mut todo_list = TodoList::new();
        todo_list.add(args.title, args.description);
        todo_list.save(&args.file_path)?;
    }
    Ok(())
}

pub fn get(args: GetArgs) -> Result<()> {
    let todo_list = TodoList::load(&args.file_path)?;
    match args.id {
        Some(id) => {
            let todo = todo_list.get(id);
            if let Some(todo) = todo {
                println!("{:?}", todo);
            } else {
                println!("Todo not found");
            }
        }
        None => {
            todo_list.get_all().iter().for_each(|todo| {
                println!("{:?}", todo);
            });
        }
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut todo_list = TodoList::load(&args.file_path)?;
    let todo = todo_list.remove(args.id);
    if let Some(todo) = todo {
        println!("Removed todo: {:?}", todo);
        todo_list.save(&args.file_path)?;
    } else {
        println!("Todo not found");
    }
    Ok(())
}

pub fn done(args: DoneArgs) -> Result<()> {
    let mut todo_list = TodoList::load(&args.file_path)?;
    let todo = todo_list.get_mut(args.id);
    if let Some(todo) = todo {
        todo.done = true;
        println!("Marked todo as done: {:?}", todo);
        todo_list.save(&args.file_path)?;
    } else {
        println!("Todo not found");
    }
    Ok(())
}

pub fn undone(args: UndoneArgs) -> Result<()> {
    let mut todo_list = TodoList::load(&args.file_path)?;
    let todo = todo_list.get_mut(args.id);
    if let Some(todo) = todo {
        todo.done = false;
        println!("Marked todo as undone: {:?}", todo);
        todo_list.save(&args.file_path)?;
    } else {
        println!("Todo not found");
    }
    Ok(())
}
