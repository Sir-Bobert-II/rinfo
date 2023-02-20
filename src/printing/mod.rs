pub mod ascii_art;
pub use ascii_art::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[allow(dead_code)]
pub enum OsArt
{
    ArchLinux,
    AlpineLinux,
    Debian,
    Windows,
    Windows1011,
    Unknown,
}

impl std::fmt::Display for OsArt
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let art = match self
        {
            OsArt::ArchLinux => ARCH_LINUX,
            OsArt::AlpineLinux => ALPINE_LINUX,
            OsArt::Debian => DEBIAN,
            OsArt::Windows1011 => WINDOWS_10_11,
            OsArt::Windows => WINDOWS,
            _ => UNKNOWN,
        };

        write!(f, "{art}")
    }
}

pub fn print_with_logo(os: OsArt, s: &str)
{
    print!("{}", with_both(&os.to_string(), s));
}

/// Returns the length of the longest line in the string
fn min_max_line_len(s: &str) -> (usize, usize)
{
    let mut max = 0;
    let mut min = s.split('\n').next().unwrap_or("").len();

    for line in s.split('\n')
    {
        let len = line.len();

        if len > max
        {
            max = len;
        }

        if len < min
        {
            min = len;
        }
    }

    (min, max)
}

fn with_both(first: &str, second: &str) -> String
{
    let (_, max_len) = min_max_line_len(first);

    let first: Vec<&str> = first.split('\n').collect();
    let second: Vec<&str> = second.split('\n').collect();
    let mut s = String::new();

    let mut i = 0;
    while i < first.len() || i < second.len()
    {
        let fir = first.get(i).unwrap_or(&"");
        let sec = second.get(i).unwrap_or(&"");

        s.push_str(fir);

        for _ in 0..=max_len - fir.len() + 2
        {
            s.push(' ');
        }

        s.push_str(&format!("{sec}\n"));

        i += 1;
    }
    s
}
