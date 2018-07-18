#![feature(uniform_paths)]

use std::cmp::min;
use std::thread;
use std::time::{Duration, Instant};

use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let m = MultiProgress::new();

    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} {prefix} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({msg}/s {eta})")
        .progress_chars("=>-");

    for (file_name, size) in vec![("a", 2311), ("bb", 1231), ("cccccc", 3523)] {
        let pb = m.add(ProgressBar::new(size));
        pb.set_style(sty.clone());
        pb.set_prefix(&file_name);
        pb.set_message(&HumanBytes(0).to_string());
        thread::spawn(move || {
            let mut downloaded = 0;
            let mut clock = Instant::now();
            let mut bps = 0;
            while downloaded < size {
                downloaded = min(downloaded + 10, size);
                pb.set_position(downloaded);
                if clock.elapsed().as_secs() >= 1 {
                    pb.set_message(&HumanBytes(bps).to_string());
                    clock = Instant::now();
                    bps = 0;
                }
                bps += 223211;
                thread::sleep(Duration::from_millis(12));
            }
            pb.finish_and_clear();
        });
    }

    m.join_and_clear().unwrap();
}
