#[macro_export]
macro_rules! log_inner_println {
    ($lvl:expr, $fnn:expr, $fmt:expr) => {
        let dt = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        println!("{} [ {} ] {}(): {}", dt, $lvl, $fnn, $fmt);
    };
    ($lvl:expr, $fnn:expr, $fmt:expr, $($arg:tt)*) => {
        let dt = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        println!("{} [ {} ] {}(): {}", dt, $lvl, $fnn, format!($fmt, $($arg)*));
    };
}

#[macro_export]
macro_rules! log_inner_eprintln {
    ($lvl:expr, $fnn:expr, $fmt:expr) => {
        let dt = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        eprintln!("{} [ {} ] {}(): {}", dt, $lvl, $fnn, $fmt);
    };
    ($lvl:expr, $fnn:expr, $fmt:expr, $($arg:tt)*) => {
        let dt = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        eprintln!("{} [ {} ] {}(): {}", dt, $lvl, $fnn, format!($fmt, $($arg)*));
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($fnn:expr, $fmt:expr) => (log_inner_println!("debug", $fnn, $fmt));
    ($fnn:expr, $fmt:expr, $($arg:tt)*) => (log_inner_println!("debug", $fnn, $fmt, $($arg)*));
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}

#[macro_export]
macro_rules! info {
    ($fnn:expr, $fmt:expr) => (log_inner_println!("info", $fnn, $fmt));
    ($fnn:expr, $fmt:expr, $($arg:tt)*) => (log_inner_println!("info", $fnn, $fmt, $($arg)*));
}

#[macro_export]
macro_rules! warn {
    ($fnn:expr, $fmt:expr) => (log_inner_eprintln!("warn", $fnn, $fmt));
    ($fnn:expr, $fmt:expr, $($arg:tt)*) => (log_inner_eprintln!("warn", $fnn, $fmt, $($arg)*));
}

#[macro_export]
macro_rules! error {
    ($fnn:expr, $fmt:expr) => (log_inner_eprintln!("error", $fnn, $fmt));
    ($fnn:expr, $fmt:expr, $($arg:tt)*) => (log_inner_eprintln!("error", $fnn, $fmt, $($arg)*));
}
