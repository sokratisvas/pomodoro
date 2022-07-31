use std::env;
use std::io::stdin;
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration};
extern crate colour;

fn cntdown(duration: u16) -> String {
    let hours: u16 = duration / 3600;
    let mins: u16 = (duration % 3600) / 60;
    let secs: u16 = (duration % 3600) % 60;

    if duration < 3600 {
        format!("> {:0>2}:{:0>2}", mins, secs)
    } else {
        format!("> {:0>2}:{:0>2}:{:0>2}", hours, mins, secs)
    }
}

fn start_timer(mut duration: u16) {
    while duration != 0 {
        sleep(Duration::from_secs(1));
        print!("\r{}   ", cntdown(duration));
        duration -= 1;
        io::stdout().flush().unwrap();
    }
    sleep(Duration::from_secs(1));
    print!("\r{}   \n", cntdown(duration));
    io::stdout().flush().unwrap();
    //println!();
}

fn check_continue(arg: String) {
    print!("> Do you want to start your {} [Y/n]?  ", arg);
    io::stdout().flush().unwrap();
    let mut command = String::new();
    stdin()
        .read_line(&mut command)
        .ok()
        .expect("Failed to read line");
    
    while command.as_str() != "n\n" && command.as_str() != "y\n" && command.as_str() != "Y\n" {
        print!("> Do you want to start your {} [Y/n]?  ", arg);
        io::stdout().flush().unwrap();
        stdin()
            .read_line(&mut command)
            .ok()
            .expect("Failed to read line");
    }

    println!();
    if command.as_str() == "n\n" {
        colour::blue_ln!("> Take some extra time:");
        start_timer(4);
        let _notify = Command::new("notify-send")
            .arg("Extra time complete.")
            .output()
            .expect("failed to execute process");
        let _bepp = Command::new("aplay")
            .arg("beep.wav")
            .output()
            .expect("failed to notify");
        println!();
    }
}

fn run(sessions: u16, work: u16, short: u16, long: u16) {
    for session in 1..=sessions {
        colour::green_ln!("> Session {}/{}:", session, sessions);
        start_timer(work);
        let _notify = Command::new("notify-send")
            .arg("Work session complete. Take a break.")
            .output()
            .expect("failed to execute process");
        let _bepp = Command::new("aplay")
            .arg("beep.wav")
            .output()
            .expect("failed to notify");
        println!();
        check_continue("break".to_string());

        if session != sessions {
            colour::yellow_ln!("> Take a short break:");
            start_timer(short);
            let _notify = Command::new("notify-send")
                .arg("Break complete. Time to work.")
                .output()
                .expect("failed to execute process");
            let _bepp = Command::new("aplay")
                .arg("beep.wav")
                .output()
                .expect("failed to notify");
            println!();
            check_continue("work".to_string());
        } else {
            colour::cyan_ln!("> Take a long break:");
            start_timer(long);
            let _notify = Command::new("notify-send")
                .arg("Pomodoro complete! Nice work!")
                .output()
                .expect("failed to execute process");
            let _bepp = Command::new("aplay")
                .arg("beep.wav")
                .output()
                .expect("failed to notify");
        }
    }
}

fn main() {
    // Default parameters
    let mut sessions: u16 = 4;
    let mut work: u16 = 45 * 60;
    let mut short: u16 = 8 * 60;
    let mut long: u16 = 20 * 60;

    // target ses 4 w 45 s 5 l 5
    let args: Vec<String> = env::args().collect();

    if args.len() == 9 {
        sessions = args[2].to_string().parse::<u16>().unwrap(); 
        work = 60 * args[4].to_string().parse::<u16>().unwrap();
        short = 60 * args[6].to_string().parse::<u16>().unwrap();
        long = 60 * args[8].to_string().parse::<u16>().unwrap();
    }

    run(sessions, work, short, long); 
}
