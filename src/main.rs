#![allow(non_snake_case)]

extern crate dbus;
#[macro_use]
extern crate dbus_macros;

use std::rc::Rc;

const DEFAULT_OFFSET: i32 = 10;

dbus_interface!("org.mpris.MediaPlayer2.Player", interface MediaPlayer {
    fn Seek(offset: i64);
});

fn rewind_player(offset_seconds: i32) {
    let connection = match dbus::Connection::get_private(dbus::BusType::Session) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("dbus connection failed: {}", err);
            return;
        }
    };

    let player = MediaPlayer::new(
        "org.mpris.MediaPlayer2.clementine",
        "/org/mpris/MediaPlayer2",
        Rc::new(connection),
    );

    if let Err(err) = player.Seek(-offset_seconds as i64 * 1000000) {
        eprintln!("failed to seek: {}", err);
        return;
    }
}

fn get_offset_seconds() -> i32 {
    std::env::args()
        .nth(1)
        .unwrap_or(DEFAULT_OFFSET.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_OFFSET)
}

fn main() {
    let offset = get_offset_seconds();
    rewind_player(offset);
}
