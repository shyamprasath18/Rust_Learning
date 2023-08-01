struct Todo{
    id: i32,
    description: String,
    completed: bool,
}


use std::io;

fn add_todo(todos: &mut Vec<Todo>, last_id: &mut i32){
    println!("Enter the descritpion of the todo:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");


    let todo = Todo{
        id: *last_id,
        description: description.trim().to_string(),
        completed:false,
    };

    *last_id += 1;
    todos.push(todo);
    println!("Todo added Successfully");
}

fn list_todos(todos: &Vec<Todo>){
    for todo in todos{
        println!(
            "[{}] {} - {}",
            if todo.completed { "x" } else { " " },    
            todo.id,
            todo.description
        );
    }
}

fn complete_todo(todos: &mut Vec<Todo>, id: i32){
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id){
        todo.completed = true;
        println!("Todo marked as completed!");
    }
    else{
        println!("Todo not found!");
    }
}

fn main(){
    let mut todos: Vec<Todo> = Vec::new();
    let mut last_id =0;

    loop{
        println!("Commands:");
        println!(" 1. Add todo");
        println!(" 2. List todos");
        println!(" 3. Complete todo");
        println!(" 4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u32 = input.trim().parse().expect("Invalid input");

        match choice{
            1 => add_todo(&mut todos, &mut last_id),
            2 => list_todos(&todos),
            3 => {
                println!("Enter the ID of the todo to complete:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");
                let id:i32 = id_input.trim().parse().expect("Invalid Input");
                complete_todo(&mut todos, id)
            }
            4 => break,
            _ => println!("Invaild choice!"),

        }
    }
}
