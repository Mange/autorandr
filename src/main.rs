extern crate regex;
#[macro_use]
extern crate structopt;

use regex::Regex;
use std::process::Command;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opts {
    #[structopt(short = "n", long = "dry-run")]
    /// Don't set resolution, instead print the xrandr command that would have been executed.
    dry_run: bool,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opts = Opts::from_args();
    let output = Command::new("xrandr").arg("--current").output()?;

    if output.status.success() {
        let screens = parse_screens(&String::from_utf8(output.stdout)?)?;
        let args = xrandr_args(screens);

        if opts.dry_run {
            println!("xrandr {}", args.join(" "));
            Ok(())
        } else {
            setup_screens(args)
        }
    } else {
        Err("xrandr command failed".into())
    }
}

fn xrandr_args(screens: Vec<Screen>) -> Vec<String> {
    let mut last_screen: Option<Screen> = None;
    let mut args: Vec<String> = Vec::new();

    for screen in screens.into_iter() {
        args.push("--output".into());
        args.push(screen.output.clone());
        args.push("--mode".into());
        args.push(screen.resolution.clone());
        if let Some(rate) = screen.rate {
            args.push("--rate".into());
            args.push(format!("{}", rate));
        }

        if let Some(last_screen) = last_screen.take() {
            args.push("--right-of".into());
            args.push(last_screen.output);
        } else {
            args.push("--primary".into());
        }
        last_screen = Some(screen);
    }

    args
}

fn setup_screens(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    let status = Command::new("xrandr").args(&args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "xrandr failed with exit status: {}",
            status.code().unwrap_or(1)
        ).into())
    }
}

#[derive(Debug)]
struct Monitor {
    output: String,
    best_modeline: Modeline,
}

#[derive(Debug)]
struct Modeline {
    resolution: (i32, i32),
    best_rate: Option<f32>,
}

#[derive(Debug)]
/// Represents a finished configuration for a single screen. This will be sent
/// to xrandr to configure the given output.
struct Screen {
    output: String,
    resolution: String,
    rate: Option<f32>,
}

fn parse_screens(xrandr_output: &str) -> Result<Vec<Screen>, String> {
    let output_re: Regex = "^(?P<output>[A-Z0-9-]+) connected ".parse().unwrap();
    let modeline_re: Regex = "^\\s+(?P<width>\\d+)x(?P<height>\\d+)\\s+(?P<rates>.*)$"
        .parse()
        .unwrap();
    let rates_re: Regex = "(\\d+\\.\\d+)".parse().unwrap();

    let mut monitors: Vec<Monitor> = Vec::new();
    let mut current_output: Option<String> = None;
    let mut best_modeline: Option<Modeline> = None;

    for line in xrandr_output.lines() {
        if let Some(captures) = output_re.captures(line) {
            // If it matches a new output line, finish the current monitor and
            // start over on a new one.
            if let (Some(output), Some(modeline)) = (current_output, best_modeline) {
                monitors.push(Monitor::new(output, modeline));
            }

            current_output = Some(captures["output"].into());
            best_modeline = None;
        } else if let Some(captures) = modeline_re.captures(line) {
            let height = captures["height"].parse().unwrap();
            let width = captures["width"].parse().unwrap();
            let best_rate = rates_re
                .captures_iter(&captures["rates"])
                .map(|caps| caps[0].parse().unwrap())
                .max_by(|a: &f32, b: &f32| a.partial_cmp(b).unwrap()); // Begone, NaN!

            let new_modeline = Modeline {
                resolution: (width, height),
                best_rate,
            };

            let old_modeline = best_modeline.take();

            match old_modeline {
                Some(best) => {
                    if new_modeline.resolution.0 > best.resolution.0 {
                        best_modeline = Some(new_modeline);
                    } else {
                        best_modeline = Some(best);
                    }
                }
                None => {
                    best_modeline = Some(new_modeline);
                }
            }
        }
    }

    if let (Some(output), Some(modeline)) = (current_output, best_modeline) {
        monitors.push(Monitor::new(output, modeline));
    }

    monitors.sort_by_key(|monitor| -monitor.best_modeline.resolution.0);

    Ok(monitors.into_iter().map(Screen::from).collect())
}

impl Monitor {
    fn new(output: String, best_modeline: Modeline) -> Monitor {
        Monitor {
            output,
            best_modeline,
        }
    }
}

impl From<Monitor> for Screen {
    fn from(monitor: Monitor) -> Screen {
        let (width, height) = monitor.best_modeline.resolution;

        Screen {
            output: monitor.output,
            resolution: format!("{}x{}", width, height),
            rate: monitor.best_modeline.best_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_and_sorts_output() {
        let output = r##"
Screen 0: minimum 8 x 8, current 4480 x 1440, maximum 16384 x 16384
DVI-I-0 disconnected (normal left inverted right x axis y axis)
DVI-I-1 disconnected (normal left inverted right x axis y axis)
HDMI-0 disconnected (normal left inverted right x axis y axis)
DP-0 connected primary 2560x1440+0+0 (normal left inverted right x axis y axis) 598mm x 336mm
   2560x1440     59.95*+ 143.86   119.88    99.95
   1920x1080    143.85    60.00    59.94    50.00
   1680x1050     59.95
   1440x900      59.89
   1280x1024     75.02    60.02
   1280x960      60.00
   1280x800      59.81
   1280x720      60.00    59.94    50.00
   1152x864      75.00
   1024x768      75.03    70.07    60.00
   800x600       75.00    72.19    60.32    56.25
   720x576       50.00
   720x480       59.94
   640x480       75.00    72.81    59.94    59.93
DP-1 disconnected (normal left inverted right x axis y axis)
DP-2 disconnected (normal left inverted right x axis y axis)
DP-3 disconnected (normal left inverted right x axis y axis)
DP-4 connected 1920x1200+2560+0 (normal left inverted right x axis y axis) 518mm x 324mm
   1920x1200     59.95*+
   1920x1080     60.00
   1680x1050     59.95
   1600x1200     60.00
   1280x1024     60.02
   1280x960      60.00
   1024x768      60.00
   800x600       60.32
   640x480       59.94
DP-5 disconnected (normal left inverted right x axis y axis)
"##;
        let screens = parse_screens(output).unwrap();
        assert_eq!(screens.len(), 2);

        assert_eq!(screens[0].output, "DP-0");
        assert_eq!(screens[1].output, "DP-4");

        assert_eq!(screens[0].resolution, "2560x1440");
        assert_eq!(screens[1].resolution, "1920x1200");

        assert_eq!(screens[0].rate, Some(143.86));
        assert_eq!(screens[1].rate, Some(59.95));
    }

    #[test]
    fn it_builds_xrandr_args_from_the_left() {
        let screens = vec![
            Screen {
                output: "FOO-1".into(),
                resolution: "2560x1440".into(),
                rate: Some(144.0),
            },
            Screen {
                output: "FOO-2".into(),
                resolution: "1920x1080".into(),
                rate: Some(59.95),
            },
        ];

        let args = xrandr_args(screens);

        assert_eq!(
            &args,
            &[
                "--output",
                "FOO-1",
                "--mode",
                "2560x1440",
                "--rate",
                "144",
                "--primary",
                "--output",
                "FOO-2",
                "--mode",
                "1920x1080",
                "--rate",
                "59.95",
                "--right-of",
                "FOO-1",
            ]
        );
    }
}
