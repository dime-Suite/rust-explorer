use std::{panic, thread};

use log::error;

// TODO file based logging
pub fn init(level: &str) {
    std::env::set_var("RUST_LOG", level);
    env_logger::init();

    // catch panic and log them using tracing instead of default output to StdErr
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unknown");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        let backtrace = backtrace::Backtrace::new(); // for better debungging

        match info.location() {
            Some(location) => {
                // without backtrace
                if msg.starts_with("notrace - ") {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}",
                        thread,
                        msg.replace("notrace - ", ""),
                        location.file(),
                        location.line()
                    );
                }
                // with backtrace
                else {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}\n{:?}",
                        thread,
                        msg,
                        location.file(),
                        location.line(),
                        backtrace
                    );
                }
            }
            None => {
                // without backtrace
                if msg.starts_with("notrace - ") {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}'",
                        thread,
                        msg.replace("notrace - ", ""),
                    );
                }
                // with backtrace
                else {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}'\n{:?}",
                        thread,
                        msg,
                        backtrace
                    );
                }
            }
        }
    }));
}
