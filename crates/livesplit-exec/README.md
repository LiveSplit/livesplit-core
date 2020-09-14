# livesplit-exec

`livesplit-exec` is a Linux-only tool needed for auto splitting to work
correctly due to Yama's `ptrace_scope` setting blocking access to 
`/proc/$pid/mem` as non-root by default. This tool works around that by calling
`prctl()` before launching the specified program to allow for the auto splitter
to read memory.


**NOTE:** This will not work if `sysctl kernel.yama.ptrace_scope` is greater
than 1. If this value is 0 or doesn't exist, then this tool isn't needed.

# Usage

## Initial Setup

First, you will need to install the rust toolchains for `i686-unknown-linux-gnu`
and `x86_64-unknown-linux-gnu`. Afterwards, run `./build.sh`

After running `./build.sh`, you'll have some directories in `./out`. The files
in `./out/bin` need to be placed in your `$PATH`, and the files in `./out/lib` need
to be placed in your `$LD_LIBRARY_PATH`. If this was installed from a package
rather than built from source, this step isn't needed.

## Non-Steam 

If your game is launched outside of Steam, your game can be launched by placing
`livesplit-exec` in front of the command to launch it.

```
# As sort of an example, if your game is launched like this:
./portal2.sh -game portal2
# you would instead launch it like this
livesplit-exec ./portal2.sh -game portal2
```

## Steam

If your game is launched from Steam, you'll just need to set your launch options
for the game to the following:
```
livesplit-exec %command%
```

Any additional options for your game can be placed after this.

