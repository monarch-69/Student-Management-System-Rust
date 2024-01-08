use std::fs::{ File, OpenOptions, metadata };
use std::io::{ Read, Write };
use std::error::Error;
use std::thread;
use std::time::Duration;

struct User
{
    username: String,
    password: String
}

struct Student
{
    name: String,
    roll_no: String,
    branch: String,
    division: String,
    admission_year: String
}

impl User
{
    // Creating an associated function to create a new instance of an User.
    fn new( username: String, password: String ) -> User { User { username, password } }
}

impl Student
{
    // Creating an associated function to create a new instance of a Student.
    fn new( name: String, roll_no: String, branch: String, division: String, admission_year: String ) -> Student
    { Student { name, roll_no, branch, division, admission_year } }
}

fn file_parser() -> Vec<String>
{
   
    let mut contents = String::new(); 
    let mut file = File::open("users.txt").unwrap();
    file.read_to_string(&mut contents).unwrap();
    
    let user_list: Vec<String> = contents.lines().map(|line| line.to_owned()).collect();

    user_list
}

fn user_authentication() -> Option< User >
{
    let user_list = file_parser();
    let mut username = String::new();
    let mut password = String::new();

        println!("Please enter your username : ");
        std::io::stdin().read_line(&mut username).unwrap();
        println!("Please enter your password : ");
        std::io::stdin().read_line(&mut password).unwrap();
        let user = User { username: username.trim().to_string(), password: password.trim().to_string() };
        match login( &user, &user_list )
        {
            true => { Some( user ) },
            false => { None }
        }
}

fn create_user()
{
    let mut username = String::new();
    let mut password = String::new();
    println!("Enter the name of the new user : ");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Enter the password of the new user : ");
    std::io::stdin().read_line(&mut password).unwrap();
    // The contents of the user will be written to the file.
    let user = User::new( username.trim().to_string(), password.trim().to_string() );
    
    // Calling the file_parser function to open a file.
    let mut file = OpenOptions::new().append(true).open("users.txt").unwrap();
    // Now time to append in the file.
    let contents = format!("{}:{}\n", user.username, user.password);
    match file.write_all(contents.as_bytes())
    {
        Ok(_)  => { println!("User ( {} ) created successfully!", user.username.as_str()) },
        Err(_)  => { create_user() }
    }
}

fn login( user: &User, user_list: &Vec<String> ) -> bool
{
    for users in user_list.iter()
    {
        let check: Vec<_> = users.split(":").collect();
        if check[0] == user.username && check[1] == user.password
        {
            return true;
        }
         continue;
    }
    
    false
}

fn add_student()
{
    let mut response =  String::new();
    let mut file = OpenOptions::new().append(true).open("students.txt").expect("Some error occurred!");
    
    loop
    {
        let mut name = String::new();
        let mut roll_no = String::new();
        let mut branch = String::new();
        let mut division = String::new();
        let mut admission_year = String::new();
        println!("Enter the name of the student: ");
        std::io::stdin().read_line(&mut name).unwrap();
        println!("Enter the roll_no of the student: ");
        std::io::stdin().read_line(&mut roll_no).unwrap();
        println!("Enter the branch of the student: ");
        std::io::stdin().read_line(&mut branch).unwrap();
        println!("Enter the division of the student: ");
        std::io::stdin().read_line(&mut division).unwrap();
        println!("Enter the admission_year of the student: ");
        std::io::stdin().read_line(&mut admission_year).unwrap();
        let student = Student::new(name.trim().to_string(), roll_no.trim().to_string(), branch.trim().to_string(), division.trim().to_string(), admission_year.trim().to_string());
        println!("Name : {}\nRoll No = {}\nBranch = {}\nDivision = {}\nAdmission year = {}", student.name, student.roll_no, student.branch, student.division, student.admission_year);

        println!("Are the above details correct ? ( Y | N )");
        std::io::stdin().read_line(&mut response).unwrap();
        if response.trim() == "Y" || response.trim() == "y"
        {
            let contents = format!("{}:{}:{}:{}:{}::",student.name, student.roll_no, student.branch, student.division, student.admission_year);
            file.write_all(contents.as_bytes()).unwrap();
            println!("Student ( {} ) entered successfully!\n", student.name.to_string());
            return;
        }
        response.clear();
        name.clear();
        roll_no.clear();
        branch.clear();
        division.clear();
        admission_year.clear();
    }
}

fn student_list() -> Option< Vec<String> >
{
    if let Ok(metadata) = metadata("students.txt")
    {
        if metadata.len() == 0 { return None; }
    }
    
    let mut file = File::open("students.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // let list: Vec<_> = contents.lines().map(|lines| lines.to_string()).collect();
    let mut list: Vec<String> = contents.split("::").map(|item| item.to_string()).collect();

    let mut index = 0;
    while index < list.len()
    {
        if list[index].as_bytes() == "".as_bytes()
        {
            list.remove(index);
            index += 1;
        }
        else 
        {
            index += 1;
        };
    };
    file.flush().unwrap();

    Some( list )
}

fn show_students()
{
    let student_list = match student_list()
    {
        Some(list) => { list },
        None => { println!("The student list is empty!"); return; }
    };
    
    for students in student_list.iter()
    {
        println!("( {} )", students);
    }
}

fn search_student() -> Option< Student >
{
    let list = match student_list()
    {   
        Some( list ) => { list },
        None => { println!("The student list is empty!"); return None; }
    };

    println!("Enter the Roll No of the student : ");
    let mut roll_no = String::new();
    std::io::stdin().read_line(&mut roll_no).unwrap();
    
    for students in list.iter()
    {
        let check: Vec<_> = students.split(":").collect();
        if check[1] == roll_no.trim()
        { 
            let student = Student { name: check[0].to_string(), roll_no: check[1].to_string(), branch: check[2].to_string(), division: check[3].to_string(), admission_year: check[4].to_string() };
            
            return Some( student );
        }
        continue;
    }

    None
}

fn delete_student()
{
    if let Ok(metadata) = metadata("students.txt")
    {
        if metadata.len() == 0
        {
            println!("The student list is empty!");
            return;
        }
    }
    
    let mut file = OpenOptions::new().read(true).write(true).open("students.txt").unwrap();
    let mut contents = String::new();
    // let mut student_found = false;

    file.read_to_string(&mut contents).unwrap();
    let mut list: Vec<_> = contents.split("::").map(|items| items.to_string()).collect();

    // Deleting all the "" and on getting true the element is retained and for false its removed.
    list.retain(|item| !item.is_empty());

    println!("Enter the Roll No of the student to delete : ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let index = list.iter().position(|student| { 
        let check: Vec<_> = student.split(":").collect();
        check.get(1) == Some(&input.trim())
     });
    if let Some(index) = index
    {
        list.remove(index);
        if list.len() == 0
        {
            file.set_len(0).err();
            file.flush().unwrap();
            return;
        }
        let to_write = format!( "{}::",list.join("::").to_string());
        file.set_len(0).err();
        file.write_all(to_write.as_bytes()).err();
        file.flush().unwrap();
        println!("Record deleted!");
    }
    else
    {
        println!("Student not found!");
    }
}

fn update_student()
{
    if let Ok(metadata) = metadata("students.txt")
    {
        if metadata.len() == 0
        {
            println!("The student list is empty!");
            return;
        }
    }
    let mut file = OpenOptions::new().read(true).write(true).open("students.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut list: Vec<_>= contents.split("::").map(|item| item.to_string()).collect();
    // Deleting all the "" and on getting true the element is retained and for false its removed.
    list.retain(|item| !item.is_empty());

    let mut input = String::new();
    println!("Enter the Roll No of the student to update : ");
    std::io::stdin().read_line(&mut input).unwrap();

    for students in list.iter_mut()
    {
        let check: Vec<String> = students.split(":").map(|item| item.to_string()).collect();
        if check[1] == input.trim()
        {
            let mut name = String::new();
            let mut roll_no = String::new();
            let mut branch = String::new();
            let mut division = String::new();
            let mut ad_year = String::new();
            println!("Match found!\nEnter the name of the student : ");
            std::io::stdin().read_line(&mut name).unwrap();
            println!("Enter the Roll No of the student : ");
            std::io::stdin().read_line(&mut roll_no).unwrap();
            println!("Enter the branch of the student : ");
            std::io::stdin().read_line(&mut branch).unwrap();
            println!("Enter the division of the student : ");
            std::io::stdin().read_line(&mut division).unwrap();
            println!("Enter the admission year of the student : ");
            std::io::stdin().read_line(&mut ad_year).unwrap();
            *students = format!("{}:{}:{}:{}:{}",name.trim(),roll_no.trim(),branch.trim(),division.trim(),ad_year.trim());
            file.set_len(0).unwrap();
            let to_write = format!("{}::", list.join("::"));
            file.write_all(to_write.as_bytes()).err();
            println!("Record updated Successfully!");
            return;
        }
    }
    println!("Student not found!");
}

fn greeter(user: &User)
{
    println!("Welcome back! ( {} )", user.username.to_string());
    println!("Booting up the system ...");
    thread::sleep(Duration::from_secs(1));
    println!("Retrieving files from the previous session and cleaning redundant cache ...");
    thread::sleep(Duration::from_secs(2));
    start_session();
}

fn start_session()
{
    loop 
    {
        println!("\n1. Add new student\n2. List students\n3. Search student\n4. Delete student\n5. Update student\n6. Exit\nSelect from ( 1 | 2 | 3 | 4 | 5 | 6 )");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: u8 = choice.trim().parse().unwrap();
        match choice
        {
            1 => { add_student() },
            2 => { show_students() },
            3 => {
                    match search_student()
                        {
                            Some(student) => { println!("\nFetching details : \nName : {}\nRoll No = {}\nBranch = {}\nDivision = {}\nAdmission year = {}", student.name, student.roll_no, student.branch, student.division, student.admission_year) },
                            None => { println!("Not found!"); }
                        };
                },
            4 => { delete_student(); },
            5 => { update_student(); },
            6 => { println!("Shutting down the system."); break; },
            _ => { println!("Not a write choice. Select from ( 1 | 2 | 3 | 4 | 5 | 6 )"); continue; }
        };
    }
}

fn main() -> Result< () , Box<dyn Error> > 
{
    println!("Student Management System\nAre you a : ");
    println!("1. New user\n2. Registered user\nSelect ( 1 or 2 ) : ");
    let choice:u8;

    loop 
    {
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        let response: u8 = response.trim().parse()?;
        match response
        {
            1 | 2 => { choice = response; break; },
            _ => { println!("Choose from ( 1 or 2 )"); continue; }
        }
    }
    if choice == 1
    {
        println!("Creating a new user ...");
        create_user();
    }
    else
    {
        let user = user_authentication();
        match user
        {
            Some( user ) => { greeter(&user) },
            None => { println!("Invalid credentials!"); }
        }
    }
    Ok(())
}