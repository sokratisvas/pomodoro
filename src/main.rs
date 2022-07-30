use std::env;
use std::io::stdin;
use std::io::{self, BufRead, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
}

fn check_continue(arg: String) {
    println!("> Do you want to start your {} [Y/n]?  ", arg);
    let mut command = String::new();
    stdin().read_line(&mut command)
           .ok()
           .expect("Failed to read line");
    if command.as_str() == "n\n" {
        start_timer(4);
    }
}

fn main() {
    let sessions: u16 = 4;
    let work: u16 = 45 * 60;
    let short: u16 = 5 * 60;
    let long: u16 = 15 * 60;

    for session in 1..=sessions {
        println!("> Session {}/{}:", session, sessions);
        start_timer(work);
        let notify = Command::new("notify-send")
            .arg("Work session complete. Take a break.")
            .output()
            .expect("failed to execute process");
        let bepp = Command::new("aplay")
            .arg("beep.wav")
            .output()
            .expect("failed to notify");

        println!(" ");
        check_continue("break".to_string());
        
        if session != sessions {
            println!("> Take a short break:");
            start_timer(short);
            println!("");
            check_continue("work".to_string());
        } else {
            println!("> Take a long break:");
            start_timer(long);
        }

        let notify = Command::new("notify-send")
            .arg("Work session complete. Take a break.")
            .output()
            .expect("failed to execute process");
        let bepp = Command::new("aplay")
            .arg("beep.wav")
            .output()
            .expect("failed to notify");
    }
}
