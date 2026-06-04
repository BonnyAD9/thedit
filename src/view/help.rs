use std::{collections::BTreeMap, fmt::Write};

use termal::{codes, formatc, writec, writecln};

use crate::view::ctrl::{Cmd, Keys};

pub fn help(
    keybinds: Vec<(Keys, Cmd)>,
    commands: Vec<(&str, (Cmd, Option<usize>))>,
) -> String {
    let mut res = formatc!(
        "{'g}Thedit controls{'_}
{'g}Mouse:{'_}
  {'y}Scroll up/down{'_}
    Scrolling in the editor.

  {'y}Dragging on scrollbar{'_}
    Scrolling the editor.

  {'y}Left/Right click{'_}
    Move cursor to the given position

  {'y}Left drag{'_}
    Enter visual mode, preview decimal value as unsigned.

  {'y}Right drag{'_}
    Enter visual mode, preview decimal value as signed.

  {'y}Forward{'_}
    Change to big endian mode.

  {'y}Back{'_}
    Change to little endian mode.

{'g}Keybinds:{'_}
  {'y}esc{'_}
    Cancel/go go to normal mode.

"
    );

    let mut cmds: BTreeMap<String, Vec<Keys>> = BTreeMap::new();
    for (key, cmd) in keybinds {
        cmds.entry(cmd.to_string()).or_default().push(key);
    }

    for (cmd, keys) in cmds {
        for k in keys {
            _ = writecln!(res, "  {'y}{k}{'_}");
        }
        _ = writeln!(res, "    {cmd}\n");
    }

    _ = writec!(res, "{'g}Commands:{'_}");

    let mut cmds: BTreeMap<(String, Option<usize>), Vec<&str>> =
        BTreeMap::new();
    for (name, (cmd, cnt)) in commands {
        cmds.entry((cmd.to_string(), cnt)).or_default().push(name);
    }

    for ((cmd, cnt), names) in cmds {
        res += codes::YELLOW_FG;
        for n in names {
            res += "  ";
            res += n;
        }
        res += codes::RESET;
        res += "\n    ";
        res += &cmd;
        if let Some(cnt) = cnt {
            _ = write!(res, " {cnt}");
        }
        res += "\n\n";
    }

    res
}
