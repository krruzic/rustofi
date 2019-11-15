use std::str;

use num_derive::ToPrimitive;
use num_traits::ToPrimitive;

use subprocess::{Popen, PopenConfig, Redirection};

use crate::errors::*;


/// Each variant positions the rofi window at the described position on screen
#[derive(Debug, ToPrimitive, Clone)]
pub enum Location {
    TopLeft = 1,
    TopCentre = 2,
    TopRight = 3,
    MiddleLeft = 8,
    MiddleCentre = 0,
    MiddleRight = 4,
    BottomLeft = 7,
    BottomCentre = 6,
    BottomRight = 5
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    /// width flag
    pub width: i32,
    /// height flag
    pub height: i32,
    /// lines flag
    pub lines: i32,
    /// columns flag
    pub columns: i32
}

#[derive(Debug, Clone)]
pub struct Padding {
    /// xoffset flag
    pub x: i32,
    /// yoffset flag
    pub y: i32
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnFormat {
    /// Return raw entry from the user
    StringReturn,
    /// Return an integer representing the index in the list selected
    IntReturn
}

/// represents the raw 'window' that rofi shows
/// the Window struct can be customized to change the appearance of the shown window
/// note that some fields will be overwritten by `Rustofi::AppRoot`
impl<'a, 's, 'm> Window<'m> {
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
            location: Location::TopRight,
            padding: Padding { x: 0, y: 0 },
            dimensions: Dimensions {
                width: 480,
                height: 240,
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
    fn to_args(&self) -> Vec<String>;
}
impl ToArgs for Dimensions {
    fn to_args(&self) -> Vec<String> {
        vec![
            "-width".to_string(),
            self.width.to_string().clone(),
            "-height".to_string(),
            self.height.to_string().clone(),
            "-lines".to_string(),
            self.lines.to_string().clone(),
            "-columns".to_string(),
            self.columns.to_string().clone(),
        ]
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
