use std::env;
use std::process::exit;
use std::process::Command;
use std::fs;


const MARK: &str = "# #";
const FILE_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";
const TASK_UNCOMMENT:u8 = 0;
const TASK_COMMENT:u8 = 1;
const TASK_ADD:u8 = 2;
const TASK_DELETE:u8 = 3;
const REROUTE: &str = "127.0.0.1";

struct Task {
    task: u8,
    text: String,
}

fn main() {

    let task:Task = collect_vars();
    
    println!("Reading hosts at {}", FILE_PATH);

    let contents = read_file( String::from( FILE_PATH ) );

    {
        let contents = find_contents_after_mark( contents );

        println!("hosts content:\n{}", contents );

        match task.task {
            TASK_UNCOMMENT => uncomment( contents, task.text ),
            TASK_COMMENT => comment( contents, task.text ),
            TASK_ADD => add( contents, task.text ),
            TASK_DELETE => delete( contents, task.text ),
            1_u8..=u8::MAX => println!( "how" )
        }
    }
}

fn print_docs() {
    println!( "USAGE:" );
    println!( "   website-blocker.exe [COMMAND] [arg]" );
    println!( "                                ^ arg is a website => website.com" );
    println!( "    -uncom => brings back previously commented out websites" );
    println!( "    -com => removes websites by commenting them out" );
    println!( "    -add => adds a new website to the list" );
    println!( "    -del => permanently deletes a website from the list" );
    println!( "    -help => help command" );
    println!( "" );
}

fn collect_vars() -> Task {
    
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_docs();
        exit( 0 );
    }

    else if args[1] == "help" || args[1] == "-help"  {
        print_docs();
        exit(0)
    }

    if args.len() == 2 {
        println!( "insert an arguement" );
        exit( 0 );
    }

    let task:Task;
    let arg_1 = &args[1];
    let arg_2 = &args[2];

    if arg_1 == "-uncom" {
        task = Task { 
            task: TASK_UNCOMMENT, 
            text: arg_2.to_string(),
        }
    }
    else if arg_1 == "-com" {
        task = Task { 
            task: TASK_COMMENT, 
            text: arg_2.to_string(),
        }
    }
    else if arg_1 == "-add" {
        task = Task { 
            task: TASK_ADD, 
            text:arg_2.to_string(),
        }
    }
    else if arg_1 == "-del" {
        task = Task { 
            task: TASK_DELETE, 
            text: arg_2.to_string(),
        }
    }
    else {
        println!( "wrong command supplied" );
        exit(0)
    }

    return task; 
}

fn read_file( path:String ) -> String {
    match fs::read_to_string( path ) {
        Ok( contents ) => return contents,
        Err( _ ) => {
            println!( "failed to read file" );
            exit( 0 );
        },
    };
}

fn find_contents_after_mark( contents:String ) -> String {
    
    let mut do_push: bool = false;
    let mut output: String = String::from( "" );

    for line in contents.lines() {

        if line.starts_with( MARK ) && !do_push {
            do_push = true
        }
        else if do_push {
            output.push_str( line );
            output.push_str( "\n" );
        }
    }

    if !do_push {
        create_mark( contents )
    }

    return output;
}

fn create_mark( contents:String ) {

    let mut new_contents = contents.clone();

    println!( "no mark found creating a new one" );

    let env: Vec<String> = env::args().collect();
    
    new_contents.push_str( "\n\n" );
    new_contents.push_str( MARK );
    new_contents.push_str( " " );
    new_contents.push_str( &env[0] );
    new_contents.push_str( "\n" );

    _ = fs::write( FILE_PATH, new_contents );
}

fn write_after_mark( mark_contents:String ) {
    
    let mut new_contents = String::new();

    let contents = read_file( String::from( FILE_PATH ) );

    for line in contents.lines() {

        if line.starts_with( MARK ) {
            break
        }
        new_contents.push_str( line );
        new_contents.push_str( "\n" );
    }

    let env: Vec<String> = env::args().collect();
    
    // new_contents.push_str( "\n" );
    new_contents.push_str( MARK );
    new_contents.push_str( " " );
    new_contents.push_str( &env[0] );
    new_contents.push_str( "\n" );

    new_contents = format!("{}{}", new_contents.to_owned(), mark_contents.to_owned() );
    
    _ = fs::write( FILE_PATH, new_contents );

    _ = Command::new("cmd")
                    .args( [ "ipconfig /flushdns" ] )
                    .output()
                    .expect("failed to execute process");
}

fn add_reroute_to_target( target:String ) -> String {
    let mut s = String::from( target );

    s.insert_str(0, "   " );
    s.insert_str(0, REROUTE );
    s.insert_str(0, "    " );

    return s;
}

fn generate_all_targets( target: String ) -> Vec<String> {
    
    let mut targets: Vec<String> = Vec::new();
    let mut s = String::from( target.clone() );

    targets.push( add_reroute_to_target( target ) );

    s.insert_str( 0, "www." );
    targets.push( add_reroute_to_target( s.clone() ) );

    s.insert_str( 0, "https://" );
    targets.push( add_reroute_to_target( s.clone() ) );

    return targets;
}

fn uncomment( contents: String, target: String ) {

    println!( "\nUncommenting : {}\n", target );

    let mut new_content:String = String::new();

    let targets:Vec<String> = generate_all_targets( target );

    for line in contents.lines() {
        let mut line = line.to_string();

        for target in &targets {
            let s = "# ".to_owned() + &target.clone();
            if line.starts_with( &s ) {
                println!( "Uncommented : {s}" );
                line = target.clone()
            }
        }

        new_content = format!("{}{}\n", new_content.to_owned(), line.to_owned() );
    }

    write_after_mark( new_content )
}

fn comment( contents: String, target: String ) {

    println!( "\nCommenting out : {}\n", target );

    let mut new_content:String = String::new();

    let targets:Vec<String> = generate_all_targets( target );

    for line in contents.lines() {
        let mut line = line.to_string();
        
        for target in &targets {
            let s = target.clone();
            if line.starts_with( &s ) {
                println!( "Commented out : {s}" );
                line = "# ".to_owned() + &target.clone();
            }
        }

        new_content = format!("{}{}\n", new_content.to_owned(), line.to_owned() );
    }

    write_after_mark( new_content )
}

fn add( contents: String, target: String ) {

    println!( "\nAdding : {}\n", target );

    let mut new_content:String = String::from( contents );

    let targets:Vec<String> = generate_all_targets( target );
        
    for line in targets {
        new_content = format!("{}{}\n", new_content.to_owned(), line.to_owned() );
        println!( "Adding : {line}" );
    }

    write_after_mark( new_content )
}

fn delete( contents: String, target: String ) {

    println!( "\nDeleting : {}\n", target );

    let mut new_content:String = String::new();

    let targets:Vec<String> = generate_all_targets( target );

    for line in contents.lines() {
        let line = line.to_string();

        let mut should_add = true;
        
        for target in &targets {
            let s = target.clone();
            if line.starts_with( &s ) {
                println!( "Deleted : {s}" );
                should_add = false
            }
        }
        
        if should_add {
            new_content = format!("{}{}\n", new_content.to_owned(), line.to_owned() );
        }
    }

    write_after_mark( new_content )
}