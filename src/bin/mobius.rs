use std::env;
use std::process;
use mobius_ecs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "new" => handle_new_command(&args[2..]),
        "templates" | "list" => handle_list_templates(),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            process::exit(1);
        }
    }
}

fn handle_new_command(args: &[String]) {
    let mut template_name = None;
    let mut project_name = None;
    let mut output_dir = None;
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--template" | "-t" => {
                if i + 1 < args.len() {
                    template_name = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --template requires a value");
                    process::exit(1);
                }
            }
            "--name" | "-n" => {
                if i + 1 < args.len() {
                    project_name = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --name requires a value");
                    process::exit(1);
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_dir = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --output requires a value");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                process::exit(1);
            }
        }
    }
    
    let template_name = template_name.unwrap_or_else(|| {
        eprintln!("Error: --template is required");
        process::exit(1);
    });
    
    let project_name = project_name.unwrap_or_else(|| {
        eprintln!("Error: --name is required");
        process::exit(1);
    });
    
    let output_dir = output_dir.unwrap_or_else(|| format!("./{}", project_name));
    
    match generate_mobius_project(&template_name, &project_name, &output_dir) {
        Ok(()) => {
            println!("🎉 Project created successfully!");
            println!("📁 Location: {}", output_dir);
            println!("🚀 To run: cd {} && cargo run", output_dir);
        }
        Err(e) => {
            eprintln!("❌ Error creating project: {}", e);
            process::exit(1);
        }
    }
}

fn handle_list_templates() {
    let registry = MobiusTemplateRegistry::default();
    
    println!("📋 Available Mobius-ECS Templates:");
    println!();
    
    for (name, template) in registry.get_templates() {
        println!("  🔹 {} - {}", name, template.description);
    }
    
    println!();
    println!("💡 Usage: mobius new --template <template_name> --name <project_name>");
    println!("💡 Example: mobius new --template gerber_viewer --name MyGerberApp");
}

fn print_help() {
    println!("🌀 Mobius-ECS - ECS-based UI templating framework for egui");
    println!();
    println!("USAGE:");
    println!("    mobius <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    new         Create a new project from a template");
    println!("    templates   List available templates");
    println!("    help        Show this help message");
    println!();
    println!("OPTIONS for 'new' command:");
    println!("    --template, -t <NAME>    Template to use");
    println!("    --name, -n <NAME>        Project name");
    println!("    --output, -o <DIR>       Output directory (default: ./<project_name>)");
    println!();
    println!("EXAMPLES:");
    println!("    mobius templates");
    println!("    mobius new --template gerber_viewer --name MyGerberApp");
    println!("    mobius new -t text_editor -n MyEditor -o ./my_editor");
    println!();
    println!("For more information, visit: https://github.com/yourusername/mobius-ecs");
}