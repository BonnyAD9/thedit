use termal::{gradient, printacln};

pub fn help() {
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let signature = gradient("BonnyAD9", (250, 50, 170), (180, 50, 240));

    printacln!(
        "Welcome in {'g i}thedit{'_} by {signature}{'_}
Version {v}

{'g}Usage:
  {'c}thedit {'gr}[{'dy}flags{'gr}] [file]{'_}
    Open hex viewer with the given file. If `file` is not present, hexdump from
    stdin.

{'g}Flags:
  {'y}-h  -?  --help{'_}
    Show this help.

  {'y}-i  --input {'w}<PATH>{'_}
    Open the given file.

  {'y}-d  --dump{'_}
    Don't open interactive editor. Just dump the contents.

  {'y}-c  --count {'w}<N>|all|auto{'_}
    Show only N first hextets (=> N lines in terminal + header). Setting this
    to auto will print as much lines as fits the screen including the header or
    first 10 lines if the terminal size couldn't be determined. Default is all.

  {'y}--head{'_}
    Same as `{'y}-c auto{'_}`.

  {'y}--utf{'_}
    Use utf graphic characters to represent non graphic ascii characters in the
    ascii view.

  {'y}--stdin{'_}
    Read data from stdin. This also implies `{'y}--dump{'_}`.
"
    )
}
