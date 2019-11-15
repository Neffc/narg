NARG  
(Neff's Alchemical Recipe Generator)  
  
A tool I made to generate alchemy recipes for Noita. I reimplemented the RNG code, which means it also works offline.  
  
Usage: narg [options] SEED  
Usage: narg -s [LC#1, LC#2, LC#3, AP#1, AP#2, AP#3]  
Options:  
    `-a`, `--array`         prints in comma-delimited format (seed#,x,x,x,y,y,y)  
    `-s`, `--search`        search all seeds for a given recipe  
    `-l`, `--list`          list all possible alchemy ingredients  
    `-h`, `--help`          print this help menu  
  
Search has been added to narg:  
 - Each recipe will contain at most 1 "Solid" ingredient. Valid recipes contain either 3 "Liquids", or 2 "Liquids" & 1 "Solid". 6 ingredients total are needed to perform a search.  
 - Seeds are unable to generate with duplicates and/or elements that aren't contained in the "Liquids" or "Solids" categories.  
 - If you attempt to search something that doesn't meet these requirements, narg will still attempt to search for the recipe, but will not return any results (using the `--list` option will show the possible ingredient names for both categories).  
 - Additionally, some recipes that are technically valid are not possible (e.g. narg -s oil water blood mud water soil), and will also not return any results.  
 - Since ingredient #2 for both recipes is the ingredient that is consumed, ingredients #1 & #3 are treated interchangeably for both recipes.  
  
  
  Example: `narg -s oil water blood oil water alcohol`  
Above example will search for a seed with a Lively Concoction recipe with oil, water, and blood, and an Alchemical Precursor recipe with oil, water, and alcohol.  
  
If you have any questions or encounter any issues, feel free to message me on Discord or ping me in the #alchemy channel @Neff#6398.