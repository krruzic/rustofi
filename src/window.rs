use crate::errors::*;

use std::str;
use std::str::FromStr;

use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use subprocess::PopenError;
use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

trait ToArgs {
    fn to_args(&self) -> Vec<String>;
}

#[derive(Debug, ToPrimitive)]
pub enum Location {
    TopLeft = 1,
    TopCentre = 2,
    TopRight = 3,
    MiddleLeft = 8,
    MiddleCentre = 0,
    MiddleRight = 4,
    BottomLeft = 7,
    BottomCentre = 6,
    BottomRight = 5,
}

pub struct Dimensions {
    pub width: i32,
    pub height: i32,
    pub lines: i32,
    pub columns: i32,
}

pub struct Padding {
    pub x: i32,
    pub y: i32,
}

pub struct Window<'m> {
    prompt: String,
    message: Option<&'m str>,
    // Additional args not covered
    additional_args: Vec<String>,
    // Graphics information
    padding: Padding,
    location: Location,
    dimensions: Dimensions,
    fullscreen: bool,
}

pub struct RofiData {
    pub index: i32,    // which list element they selected
    pub entry: String, // which list element they selected
    pub exit_code: Result<ExitStatus, PopenError>,
}

impl<'a, 's, 'm> Window<'m> {
    fn run_blocking(self, options: Vec<String>) -> Result<RofiData, PopenError> {
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

        let mut call = vec!["rofi", "-dmenu", "-format", "i"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        call.extend(self.to_args());
        let mut p = Popen::create(&call, pc)?;
        println!("{:?}", call);
        // Obtain the output from the standard streams.
        let (entry, _) = p.communicate(Some(&options_arr))?;
        let index = i32::from_str(entry.unwrap().trim()).unwrap();
        Ok(RofiData {
            index: index,
            entry: options[index as usize].to_string(),
            exit_code: p.wait(),
        })
    }
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
                columns: 1,
            },
            fullscreen: false,
        }
    }
    pub fn message(mut self, msg: &'static str) -> Self {
        self.message = Some(msg);
        self
    }
    pub fn location(mut self, l: Location) -> Self {
        self.location = l;
        self
    }
    pub fn padding(mut self, x: i32, y: i32) -> Self {
        self.padding = Padding { x, y };
        self
    }
    pub fn dimensions(mut self, d: Dimensions) -> Self {
        self.dimensions = d;
        self
    }
    pub fn prompt(mut self, s: String) -> Self {
        self.prompt = s;
        self
    }
    pub fn lines(mut self, l: i32) -> Self {
        self.dimensions.lines = l;
        self
    }
    pub fn fullscreen(mut self, f: bool) -> Self {
        self.fullscreen = f;
        self
    }
    pub fn add_args(mut self, args: Vec<String>) -> Self {
        self.additional_args.extend(args);
        self
    }
    pub fn show(self, options: Vec<String>) -> Result<RofiData, WindowError> {
        let res = self.run_blocking(options);
        match res {
            Ok(d) => {
                return Ok(d);
            }
            Err(e) => Err(e.into()),
        }
    }
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

impl<'a, 'm> ToArgs for Window<'m> {
    fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
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
