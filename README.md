NARG
(Neff's Alchemical Recipe Generator)  
  
A tool I made to generate alchemy recipes for Noita, since other tools either didn't work well or required a modded client. I reimplemented the RNG code, which means it also works offline, so I can index large batches without slamming Zatherz's server. Patched up a bit for public release, though some things I'm still working on (and the code is hideous).  
  
Usage: narg [options] SEED  
Options:  
    `-a`, `--array`         prints recipe in comma-delimited format (seed#,x,x,x,y,y,y)  
    `-s`, `--search`        returns recipe only if specified ingredients are present (WIP, doesn't really work :) )  
    `-h`, `--help`          print this help menu  
