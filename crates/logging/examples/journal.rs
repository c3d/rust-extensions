use std::fs;
use std::thread;

use containerd_shim_logging as logging;

use logging::{Config, Driver};

#[cfg(target_os = "linux")]
fn pump(reader: fs::File) {
    use std::io::{self, BufRead};
    use systemd::journal;

    io::BufReader::new(reader)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|str| {
            journal::print(0, &str);
        });
}

#[cfg(not(target_os = "linux"))]
fn pump(_reader: fs::File) {}

struct Journal {
    stdout_handle: thread::JoinHandle<()>,
    stderr_handle: thread::JoinHandle<()>,
}

impl Driver for Journal {
    type Error = String;

    fn new(config: Config) -> Result<Self, Self::Error> {
        let stdout = config.stdout;
        let stderr = config.stderr;

        Ok(Journal {
            stdout_handle: thread::spawn(|| pump(stdout)),
            stderr_handle: thread::spawn(|| pump(stderr)),
        })
    }

    fn run(self) -> Result<(), Self::Error> {
        self.stdout_handle
            .join()
            .map_err(|err| format!("{:?}", err))?;
        self.stderr_handle
            .join()
            .map_err(|err| format!("{:?}", err))?;
        Ok(())
    }
}

fn main() {
    logging::run::<Journal>()
}