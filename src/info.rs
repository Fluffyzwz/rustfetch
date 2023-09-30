use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
pub fn print_distro() {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("PRETTY_NAME=") {
                    let distro_name = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                    println!("Distro: {}", distro_name);
                    return;
                }
            }
        }
    } else {
        println!("Failed to open /etc/os-release");
    }
}
pub fn print_kernel() {
    let output = Command::new("uname")
        .arg("-r")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run uname");
    if output.status.success() {
        let kernel_info = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        println!("Kernel: {}", kernel_info.trim());
    } else {
        eprintln!("Failed to execute uname: {:?}", output.status);
    }
}
pub fn print_ram() {
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        let mut found_first_ram = false;
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.contains("MemTotal") {
                    let parts: Vec<&str> = line_content.split(':').collect();
                    if parts.len() == 2 {
                        let model_name = parts[1].trim();
                        println!("RAM: {}", model_name);
                        found_first_ram = true;
                    }
                }
                if found_first_ram {
                    break;
                }
            }
        }
    }
}
pub fn print_cpu() {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = BufReader::new(file);
        let mut found_first_cpu = false;
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.contains("model name") {
                    let parts: Vec<&str> = line_content.split(':').collect();
                    if parts.len() == 2 {
                        let model_name = parts[1].trim();
                        println!("CPU: {}", model_name);
                        found_first_cpu = true;
                    }
                }
                if found_first_cpu {
                    break;
                }
            }
        }
    }
}
