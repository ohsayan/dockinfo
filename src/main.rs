use std::env;
use std::process;
#[cfg(unix)]
const EXIT_OK: i32 = 0x0000;
const EXIT_ERR: i32 = 0x0100;
const MSG_USAGE: &'static str = "Run dockinfo --help for usage";
const CMD_DOCKER_INSPECT: &'static str =
    "'{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}'";
const ARG_IP: &'static str = "ip";
const ARG_HELP: &'static str = "--help";
const ERR_UNIX_SOCKET: &'static str = "Got permission denied";
const ERR_NOT_FOUND: &'static str = "Error: No such object: ";
const RESULT_NO_OUTPUT: &'static str = "''";
const ENV_VARIABLE_DOCKER_BINARY: &'static str = "DOCKER_BIN";

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // (HACK) remove the binary's own name

    let dock_location = match env::var(ENV_VARIABLE_DOCKER_BINARY) {
        Ok(loc) => Some(loc),
        Err(_) => None,
    };

    if args.len() < 1 {
        eprintln!("Error: Invalid number of arguments! {}", MSG_USAGE);
        process::exit(EXIT_ERR);
    }

    let first_arg = match args.get(0) {
        Some(a) => a,
        None => {
            panic!("Error: An error occurred while getting the argument!");
        }
    };

    match first_arg.as_str() {
        // We directly pass the ownership of `args` to the respective operations
        ARG_IP => get_ipinfo(args, dock_location),
        ARG_HELP => show_help(),
        _ => {
            eprintln!("Error: Invalid operation: {}\n{}", first_arg, MSG_USAGE);
            process::exit(EXIT_ERR);
        }
    }
}

fn get_ipinfo(args: Vec<String>, path_if_any: Option<String>) {
    if args.len() == 1 {
        eprintln!(
            "Error: Invalid container name. For getting the IP address of the container run dockinfo ip <CONTAINER_NAME>"
        );
        process::exit(EXIT_ERR);
    } else if args.len() == 2 {
        let mut cmdline = process::Command::new("sudo");
        if let Some(path) = path_if_any {
            cmdline.arg(path);
        } else {
            cmdline.arg("docker");
        }
        cmdline
            .arg("inspect")
            .arg("-f")
            .arg(CMD_DOCKER_INSPECT)
            .arg(args.get(1).unwrap());
        let output = cmdline.output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout = stdout.trim();
        if stderr.len() == 0 {
            if stdout == RESULT_NO_OUTPUT {
                eprintln!(
                    "Error: The container '{}' is not running",
                    args.get(1).unwrap()
                );
                process::exit(EXIT_ERR);
            } else {
                println!("{}", stdout);
                process::exit(EXIT_OK);
            }
        } else if stderr.contains(ERR_UNIX_SOCKET) {
            eprintln!("Error: Please run the program as the superuser!");
            process::exit(EXIT_ERR);
        } else if stderr.contains(ERR_NOT_FOUND) {
            eprintln!(
                "Error: The container {} could not be found",
                args.get(1).unwrap()
            );
            process::exit(EXIT_ERR);
        } else if stderr.contains("sudo:") && stderr.contains(": command not found") {
            eprintln!("The DOCKER_BIN path has not been configured correctly");
            process::exit(EXIT_ERR);
        } else {
            eprintln!("{}", stderr);
            process::exit(EXIT_ERR);
        }
    } else {
        eprintln!("Error: Invalid usage. {}", MSG_USAGE);
        process::exit(EXIT_ERR);
    }
}
fn show_help() {
    println!(
        "USAGE: dockinfo ip <CONTAINER_NAME>\nGet information about docker containers. dockinfo expects that the docker binary is already present in the path."
    );
    process::exit(EXIT_OK);
}
