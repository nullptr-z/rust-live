use anyhow::Result;
use std::usize;

#[derive(Debug)]
pub(crate) struct Cmd {
    // pub(crate) len: usize,
    pub(crate) value: String,
}

#[derive(Debug)]
pub(crate) struct CmdLine {
    // pub(crate) count: usize,
    pub(crate) args: Vec<Cmd>,
}

impl CmdLine {
    pub(crate) fn new_from_str(value: &str) -> Result<Self> {
        let mut cmd_line = Self {
            // count: 0,
            args: Vec::new(),
        };
        let cmds = value.split(|v| v == '\n' || v == '\r').collect::<Vec<_>>();
        let mut count = cmds[0][1..].parse::<usize>()?;
        if count == 0 {
            return Ok(cmd_line);
        }
        // cmd_line.count = count;

        let mut i = 1;
        while count > 0 {
            {
                // let len = cmds[i + 1][1..].parse::<usize>()?;
                let value = cmds[i + 3].to_owned();
                let command = Cmd::new(value);
                cmd_line.args.push(command);
                count -= 1;
                i += 4;
            }
        }

        Ok(cmd_line)
    }
}

impl Cmd {
    pub(crate) fn new(v: impl Into<String>) -> Self {
        Self {
            // len,
            value: v.into(),
        }
    }
}
