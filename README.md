# TopRSS

Linux Command line utility for printing top **VmRSS** processes

This program was designed in mind with integration to status bars like [WayBar](https://github.com/Alexays/Waybar/) or Gnome Top Bar with [Executor](https://extensions.gnome.org/extension/2932/executor/)

# Usage

As of right now, simply running **toprss** (in terminal) without any options gives a result alike to this one:
`Isolated Web Co: 2465 MB codium: 1507 MB firefox: 1130 MB` 

By default it prints 3 top VmRSS consuming processes from /proc directory in MB.

Well, the above sentence is actually a lie!!!

To be technically correct, the best kind of correct,

what really happens is that by default **toprss**

goes through all processes that are in the /proc directory,

the keeps the ones that have both name and VmRSS information in status file,

and then groups all processes with the same name.

There is going to be an option to ungroup those in the future
if You wish to use that, although **I** don't find it useful in my usecase.

Currently **toprss** has this options (You can also see them by running `toprss --help`)
```
options:
  -h, --help, -H, -?                 display this help message and exit
  -v, --version                      display program's version and exit
  -g, --group        DEFAULT         group processes with the same name
  -n,                DEFAULT n = 3   display at most top 'n' processes
  -a, --all                          display all processes
  -o, --line         DEFAULT         display processes in one line 
  -l, --lines                        display each process on separate line
      --kb                           display VmRSS usage in kB
      --mb           DEFAULT         display VmRSS usage in MB
      --gb                           display VmRSS usage in GB
```

Since this program is still not officially released with 1.0 version, all those options and the way they work might be a subject to change

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
 - add ungroup option (does not group processes with the same name)
 - add intelligent option (displays usage in appropriate unit based on individual process VmRSS size)
 - add option to color the output
 - add options to customize coloring, color diffrent parts of the output, color based on some conditions
 - add option to print a separator between each printed process

## In case of any problems..

Contact me or create an issue!
You can also request new features!