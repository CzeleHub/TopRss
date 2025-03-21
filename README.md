# TopRSS 0.1

Linux Command line utility for printing top **VmRSS** processes

This program was designed in mind with integration to status bars like [WayBar](https://github.com/Alexays/Waybar/) or Gnome Top Bar with [Executor](https://extensions.gnome.org/extension/2932/executor/)

# Usage

As of right now, simply running **toprss** (in terminal) without any options gives a result alike to this one:
`Isolated Web Co: 2.02 GB codium: 1.54 GB firefox: 983 MB` 

By default it prints 3 top VmRSS consuming processes from /proc directory in appropriate unit(see --smart option in --help for details).

Well, the above sentence is actually a lie!!!

To be technically correct, the best kind of correct,

what really happens is that by default **toprss**

goes through all processes that are in the /proc directory,

then keeps the ones that have both name and VmRSS information in status file,

and then groups all processes with the same name.

Currently **toprss** has these options (You can also see them by running `toprss --help`)
```
options:
  -h, --help, -H, -?                 display this help message and exit
  -v, --version                      display program's version and exit
  -g, --group        DEFAULT         group processes with the same name
      --group-count                  display how many processes are in a given group
  -u, --ungroup                      ungroup processes with the same name
  -n,                DEFAULT n = 3   display at most top 'n' processes
  -a, --all                          display all processes
  -o, --line         DEFAULT         display processes in one line 
  -l, --lines                        display each process on separate line
      --kb                           display VmRSS usage in kB
      --mb                           display VmRSS usage in MB
      --gb                           display VmRSS usage in GB
  -s, --smart        DEFAULT         display VmRSS usage in appropriate unit ( kB if vmrss < MB, MB if vmrss < GB, else GB )
```

Since this program is still not officially released with 1.0 version, ***all those options and the way they work might be a subject to change***

There is no official release yet.
To try it for yourself check the installation section below!

# Installation

To compile (on linux) follow these steps:
1. [install rust](https://www.rust-lang.org/tools/install)
2. Clone the repository
```
git clone https://github.com/CzeleHub/TopRss.git
```
3. Compile
```
cargo build --release
```

You can now run it. just go to target/release and run toprss

# Future plans
todo:
 - ~~add ungroup option (does not group processes with the same name)~~ IMPLEMENTED
 - ~~add intelligent option (displays usage in appropriate unit based on individual process VmRSS size)~~ IMPLEMENTED
 - add option to color the output
 - add options to customize coloring, color diffrent parts of the output, color based on some conditions
 - add option to print a separator between each printed process

## In case of any problems..

Contact me or create an issue!
You can also request new features!