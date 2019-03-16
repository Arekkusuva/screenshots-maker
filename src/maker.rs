use scrap::{Capturer, Display};

use std::time::Duration;
use std::thread;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::io::ErrorKind::WouldBlock;
use std::cell::RefCell;

struct Inner {
    capturer: Capturer,
    wh_size: (usize, usize),
}

impl Inner {
    fn new() -> Self {
        let display = Display::primary().expect("Couldn't find primary display.");
        let capturer = Capturer::new(display).expect("Couldn't begin capture.");
        let wh_size = (capturer.width(), capturer.height());
        Self {
            capturer,
            wh_size,
        }
    }

    fn rebuild(&mut self) {
        let display = Display::primary().expect("Couldn't find primary display.");
        let capturer = Capturer::new(display).expect("Couldn't begin capture.");
        self.wh_size = (capturer.width(), capturer.height());
        self.capturer = capturer;
    }
}

pub struct Maker {
    inner: RefCell<Inner>,
    frame: Duration,
    rebuild_count: u8,
    get_path: Box<Fn() -> PathBuf + 'static>,
}

impl Maker {
    pub fn with_path_generator<F>(f: F) -> Self
        where F: Fn() -> PathBuf + 'static,
    {
        Self {
            inner: RefCell::new(Inner::new()),
            frame: Duration::new(1, 0) / 60,
            rebuild_count: 0,
            get_path: Box::new(f),
        }
    }

    fn rebuild_after_err<E: fmt::Display>(&self, err: E) {
        if self.rebuild_count == 3 {
            panic!("Error: {}", err);
        }
        self.inner.borrow_mut().rebuild()
    }

    pub fn take(&self) -> PathBuf {
        loop {
            let wh_size = self.inner.borrow().wh_size;
            let mut inner = self.inner.borrow_mut();
            let buffer = match inner.capturer.frame() {
                Ok(buffer) => buffer,
                Err(e) => {
                    if e.kind() != WouldBlock {
                        self.rebuild_after_err(e);
                    }
                    thread::sleep(self.frame);
                    continue
                }
            };

            // Flip the ARGB image into a BGRA image
            let mut bitflipped = Vec::with_capacity(wh_size.0 * wh_size.1 * 4);
            let stride = buffer.len() / wh_size.1;

            for y in 0..wh_size.1 {
                for x in 0..wh_size.0 {
                    let i = stride * y + 4 * x;
                    bitflipped.extend_from_slice(&[
                        buffer[i + 2],
                        buffer[i + 1],
                        buffer[i],
                        255,
                    ]);
                }
            }

            let mut file_path = (*self.get_path)();
            file_path.set_extension("png");

            // Save the image
            repng::encode(
                File::create(&file_path).expect("Couldn't create file"),
                wh_size.0 as u32,
                wh_size.1 as u32,
                &bitflipped,
            ).unwrap();

            return file_path;
        }
    }
}
