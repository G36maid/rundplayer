use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the music folder path as a command line argument.");
        return;
    }
    let music_folders: Vec<&str> = args[1..].iter().map(|folder| folder.as_str()).collect();
    let mut songs: Vec<String> = Vec::new();

    for music_folder in music_folders {
        let paths = fs::read_dir(music_folder).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "mp3" || extension == "wav" {
                    songs.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }

    let mut rng = rand::thread_rng();
    songs.shuffle(&mut rng);

    for song in songs {
        println!("Playing: {}", song);
        let mut child = Command::new("afplay")
            .arg(&song)
            .spawn()
            .expect("failed to play song");

        let _result = child.wait().expect("failed to wait on child");

        sleep(Duration::from_secs(1));
    }
}
