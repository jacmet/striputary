use crate::recording_session::RecordingSession;
use crate::song::Song;
use log::info;
use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub fn cut_session(session: RecordingSession) {
    cut_session_lengths(session);
    // let start_iter = session.timestamps.iter();
    // let mut end_iter = session.timestamps.iter();
    // let offset = 1.6;
    // end_iter.next().unwrap();
    // for ((start_time, end_time), song) in start_iter.zip(end_iter).zip(session.songs.iter()) {
    //     cut_song(
    //         session.get_buffer_file(),
    //         song,
    //         start_time + offset,
    //         end_time + offset,
    //     );
    // }
}

pub fn cut_session_lengths(session: RecordingSession) {
    let offset = 0.0;
    let mut start_time =
        Duration::from_secs_f64(offset + session.timestamps.iter().next().unwrap().as_secs_f64());
    for (song) in session.songs.iter() {
        let length = Duration::from_micros(song.length);
        dbg!(song.length, length);
        let end_time = start_time.clone() + length.clone();
        cut_song(session.get_buffer_file(), song, &start_time, &end_time);
        start_time = end_time;
    }
}

pub fn cut_song(source_file: PathBuf, song: &Song, start_time: &Duration, end_time: &Duration) {
    let difference = end_time.as_secs_f64() - start_time.as_secs_f64();
    let music_dir = Path::new("music");
    let target_file = song.get_target_file(&music_dir);
    create_dir_all(target_file.parent().unwrap());
    info!(
        "Cutting song: {:.2}+{:.2}: {} to {}",
        start_time.as_secs_f64(),
        difference,
        song,
        target_file.to_str().unwrap()
    );
    let out = Command::new("ffmpeg")
        .arg("-ss")
        .arg(format!("{}", start_time.as_secs_f64()))
        .arg("-t")
        .arg(format!("{}", difference))
        .arg("-i")
        .arg(source_file.to_str().unwrap())
        .arg("-acodec")
        .arg("copy")
        .arg("-y")
        .arg(target_file.to_str().unwrap())
        .output()
        .expect("Failed to execute song cutting command");

    // let o = String::from_utf8_lossy(&out.stdout);
    // let e = String::from_utf8_lossy(&out.stderr);
    // info!("{} {}", o, e);
}
