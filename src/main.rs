use anyhow::Result;
use bpaf::*;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
mod frames;
mod rainbow;

use std::sync::{
    atomic::{AtomicU64, Ordering},
    OnceLock,
};
#[derive(Default, Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Options {
    /// Enable rainbow mode
    #[bpaf(short, long)]
    rainbow: bool,
    /// Enable ratmark
    #[bpaf(short('m'), long)]
    ratmark: bool,
    /// Enable framerate unlock
    #[bpaf(short('u'), long("unlock"))]
    fps_unlock: bool,
    /// Enable debug mode
    #[bpaf(short, long)]
    debug: bool,
}

static LOOP_COUNT: AtomicU64 = AtomicU64::new(0);

static OPTIONS: OnceLock<Options> = OnceLock::new();

fn get_options() -> Options {
    OPTIONS.get_or_init(|| options().run()).clone()
}

fn print_debug() {
    let opts = get_options();
    if opts.debug {
        if opts.rainbow {
            print!("Rainbow enabled\n");
        }
        if opts.ratmark {
            print!("Ratmark enabled\n");
        }
        if opts.fps_unlock {
            print!("Framerate unlock enabled\n");
        }
        print!("Loop count: {}\n", LOOP_COUNT.load(Ordering::SeqCst));
    }
}

fn port_cls() {
    print!("\x1b[H\x1b[J");
}

fn port_gotoxy(x: i32, y: i32) {
    print!("\x1b[{y};{x}H");
}
const AUDIO: &[u8] = include_bytes!("./audio_loop.wav");

fn main() -> Result<()> {
    use std::time::Instant;
    let start = Instant::now();
    let opts = get_options();

    port_cls();
    port_gotoxy(0, 0);

    std::thread::spawn(|| audio().unwrap());

    let mut index = 0;
    loop {
        print_debug();
        if !opts.rainbow {
            print!("\n                                                         You have been blessed with {} spins of the rat.\n", LOOP_COUNT.load(Ordering::SeqCst));
            print!("{}", frames::FRAMES[index]);
        } else {
            print_rainbow!("\n                                                         You have been blessed with {} spins of the rat.\n", LOOP_COUNT.load(Ordering::SeqCst));
            print_rainbow!("{}", frames::FRAMES[index]);
        }
        if !opts.fps_unlock && !opts.ratmark {
            std::thread::sleep(std::time::Duration::from_millis(frames::DELAY_TIME as u64))
        }

        if index == (frames::FRAME_COUNT - 2) as usize {
            if LOOP_COUNT.load(Ordering::SeqCst) == 100 && opts.ratmark {
                let end = Instant::now();
                let time_taken = start - end;
                print!("100 ratmarks in {}μs\n", time_taken.as_micros());
                return Ok(());
            };

            LOOP_COUNT.fetch_add(1, Ordering::SeqCst);
            index = 0;
        };

        port_gotoxy(0, 0);
        index += 1;
    }
}
fn audio() -> Result<()> {
    loop {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new_wav(Cursor::new(AUDIO))?;
        sink.append(source);

        sink.sleep_until_end();
    }
}
