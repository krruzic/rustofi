//! A `Window` is the closest to an actual binding of a rofi command as is feasible. The types and
//! functions here allow you to construct a rofi of any size or type anywhere on the screen with ease
//! and even allows the construction of rofi's beyond that with the `additional_args` field.
//! # Example
//! ```no_run
//! // examples/simple_window.rs
//! use rustofi::window::*;
//!
//! fn fizzbuzz() -> Vec<String> {
//!     let mut results = Vec::new();
//!     // print fizzbuzz as a rofi!
//!     for x in 1..25 {
//!         // divisible by 3 or by 5
//!         match (x % 3 == 0, x % 5 == 0) {
//!             (false, false) => results.push(x.to_string()), // neither
//!             (false, true) => results.push("Fizz".to_string()), // divisible by 5
//!             (true, false) => results.push("Buzz".to_string()), // divisible by 3
//!             (true, true) => results.push("FizzBuzz".to_string()), // divisible by both
//!         }
//!     }
//!     results
//! }
//!
//! fn main() {
//!     // create a window with 8 lines and a vector of strings and show it
//!     Window::new("FizzBuzz in Rofi!").lines(8).show(fizzbuzz());
//! }
//! ```

use std::str;

use num_derive::ToPrimitive;
use num_traits::ToPrimitive;

use subprocess::{Popen, PopenConfig, Redirection};

use crate::errors::*;

/// Each variant positions the rofi window at the described position on screen
#[derive(Debug, ToPrimitive, Clone)]
pub enum Location {
    /// place the rofi window in the top left of the screen
    TopLeft = 1,
    /// place the rofi window in the top centre of the screen
    TopCentre = 2,
    /// place the rofi window in the top right of the screen
    TopRight = 3,
    /// place the rofi window in the middle left of the screen
    MiddleLeft = 8,
    /// place the rofi window in the middle centre of the screen, default
    MiddleCentre = 0,
    /// place the rofi window in the middle right of the screen
    MiddleRight = 4,
    /// place the rofi window in the bottom left of the screen
    BottomLeft = 7,
    /// place the rofi window in the bottom centre of the screen
    BottomCentre = 6,
    /// place the rofi window in the bottom right of the screen
    BottomRight = 5
}

/// represents the "dimensions" of the rofi window
#[derive(Debug, Clone)]
pub struct Dimensions {
    /// how wide to make the rofi window in pixels
    ///
    /// NOTE: this is calculated automatically otherwise
    pub width: i32,
    /// how tall to make the rofi window in pixels
    ///
    /// NOTE: this is calculated automatically otherwise
    pub height: i32,
    /// number of lines to show before scrolling
    pub lines: i32,
    /// number of columns to show
    pub columns: i32
}

/// represents the padding given to the rofi window on the X and Y axis
#[derive(Debug, Clone)]
pub struct Padding {
    /// pixels off of `Location` on the X axis to draw the window
    pub x: i32,
    /// pixels off of `Location` on the Y axis to draw the window
    pub y: i32
}

/// represents the raw 'window' that rofi shows
/// the `Window` can be customized to change the appearance of the shown window
/// note that some fields will be overwritten by types in `components.rs` and `lib.rs`
#[derive(Debug, Clone)]
pub struct Window<'m> {
    /// message to display next to the entry field
    pub prompt: String,
    /// short message displayed beneath this field and above all options
    pub message: Option<&'m str>,
    /// Additional args to pass to rofi
    pub additional_args: Vec<String>,
    /// location on screen to place the window
    pub location: Location,
    /// X and Y offsets from the `Location`
    pub padding: Padding,
    /// width, height, rows and columns of the window
    pub dimensions: Dimensions,
    /// whether to show in fullscreen. Overrides location and padding
    pub fullscreen: bool,
    /// return user selection as an index or string
    pub format: ReturnFormat
}

/// type of entry that rofi will return, typically we want the raw string using `StringReturn`
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnFormat {
    /// Return raw entry from the user
    StringReturn,
    /// Return an integer representing the index in the list selected
    IntReturn
}

impl<'a, 's, 'm> Window<'m> {
    /// open a subprocess calling the constructed rofi command and block until it returns
    fn run_blocking(self, options: Vec<String>) -> Result<String, WindowError> {
        let pc = PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::Pipe,
            ..Default::default()
        };
        let options_arr = options
            .iter()
            .map(|s| s.replace("\n", ""))
            .collect::<Vec<String>>()
            .join("\n");

        let mut call = vec!["rofi", "-dmenu", "-format"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        call.extend(self.to_args());
        let mut p = Popen::create(&call, pc)?;
        // Obtain the output from the standard streams.
        let (entry, _stdout) = p.communicate(Some(&options_arr))?;
        let entry = entry.unwrap_or("-1".to_string());
        match p.wait() {
            Ok(_p) => Ok(entry.clone().trim().to_string()),
            Err(e) => Err(e.into())
        }
    }

    /// create a window with given prompt
    pub fn new(prompt: &'a str) -> Self {
        Window {
            prompt: prompt.to_owned(),
            message: None,
            additional_args: vec![],
            location: Location::MiddleCentre,
            padding: Padding { x: 0, y: 0 },
            dimensions: Dimensions {
                width: 0,  // auto
                height: 0, // auto
                lines: 4,
                columns: 1
            },
            fullscreen: false,
            format: ReturnFormat::IntReturn
        }
    }
    /// set the window's message
    pub fn message(mut self, msg: &'static str) -> Self {
        self.message = Some(msg);
        self
    }
    /// set the window's location
    pub fn location(mut self, l: Location) -> Self {
        self.location = l;
        self
    }
    /// set the window's padding
    pub fn padding(mut self, x: i32, y: i32) -> Self {
        self.padding = Padding { x, y };
        self
    }
    /// set the window's dimensions
    pub fn dimensions(mut self, d: Dimensions) -> Self {
        self.dimensions = d;
        self
    }
    /// set the window's prompt
    pub fn prompt(mut self, s: String) -> Self {
        self.prompt = s;
        self
    }
    /// set number of lines for the window
    pub fn lines(mut self, l: i32) -> Self {
        self.dimensions.lines = l;
        self
    }
    /// set if the window should be fullscreen
    pub fn fullscreen(mut self, f: bool) -> Self {
        self.fullscreen = f;
        self
    }
    /// set the windows format
    pub fn format(mut self, f: char) -> Self {
        match f {
            's' => self.format = ReturnFormat::StringReturn,
            'i' | _ => self.format = ReturnFormat::IntReturn
        }
        self
    }

    /// add any additional args rofi accepts as an array of strings. These must include any dashes.
    ///
    /// https://gist.github.com/eyalev/a644bb75fdc6f476c2b25d9284a94682
    pub fn add_args(mut self, args: Vec<String>) -> Self {
        self.additional_args.extend(args);
        self
    }

    /// run the rofi command this window represents
    pub fn show(self, options: Vec<String>) -> Result<String, WindowError> {
        let res = self.run_blocking(options);
        match res {
            Ok(d) => {
                return Ok(d);
            }
            Err(e) => Err(e.into())
        }
    }
}

trait ToArgs {
    /// convert the type to rofi command line arguments
    fn to_args(&self) -> Vec<String>;
}
impl ToArgs for Dimensions {
    fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.width > 0 {
            args.extend(vec!["-width".to_string(), self.width.to_string().clone()]);
        }
        if self.height > 0 {
            args.extend(vec!["-height".to_string(), self.height.to_string().clone()]);
        }
        args.extend(vec![
            "-lines".to_string(),
            self.lines.to_string().clone(),
            "-columns".to_string(),
            self.columns.to_string().clone(),
        ]);
        args
    }
}

impl ToArgs for Padding {
    fn to_args(&self) -> Vec<String> {
        vec![
            "-xoffset".to_string(),
            self.x.to_string(),
            "-yoffset".to_string(),
            self.y.to_string(),
        ]
    }
}

impl ToArgs for Location {
    fn to_args(&self) -> Vec<String> {
        vec![
            "-location".to_string(),
            ToPrimitive::to_u8(self).expect("k").to_string(),
        ]
    }
}

impl ToArgs for ReturnFormat {
    fn to_args(&self) -> Vec<String> {
        match self {
            ReturnFormat::StringReturn => vec!["s".to_string()],
            ReturnFormat::IntReturn => vec!["i".to_string()]
        }
    }
}

impl<'a, 'm> ToArgs for Window<'m> {
    fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.extend(self.format.to_args());
        args.extend(self.dimensions.to_args());
        if self.fullscreen {
            args.extend(vec!["-fullscreen".to_string()]);
        } else {
            args.extend(self.padding.to_args());
            args.extend(self.location.to_args());
        }
        if let Some(msg) = self.message {
            args.extend(vec!["-mesg".to_string(), msg.to_string()]);
        }
        args.extend(vec!["-p".to_string(), self.prompt.clone()]);
        args.extend(self.additional_args.clone());
        args
    }
}
