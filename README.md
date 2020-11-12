NARG  
(Neff's Alchemy Recipe Generator)  
  
A tool I made to generate alchemy recipes for Noita. Contains robust search capability to find seeds with specific LC / AP formulas.
  
Usage: narg [options] SEED  
Usage: narg -s [LC#1, LC#2, LC#3, AP#1, AP#2, AP#3]  
Options:  
    `-a`, `--array`         prints in comma-delimited format (seed#,x,x,x,y,y,y)  
    `-s`, `--search`        search all seeds for a given recipe
                            -> (can use * to sub any ingredient as a wildcard)  
    `-l`, `--list`          list all possible alchemy ingredients  
    `-h`, `--help`          print this help menu  
  
Search has been added to narg:  
 - Each recipe will contain at most 1 "Solid" ingredient. Valid recipes contain either 3 "Liquids", or 2 "Liquids" & 1 "Solid". 6 ingredients total are needed to perform a search.  
 - Seeds are unable to generate with duplicates and/or elements that aren't contained in the "Liquids" or "Solids" categories.  
 - If searching for a recipe with an ingredient in the "Solids" category, both recipes must include a solid at the SAME POSITION (i.e. both LC#1 and AP#1 must be solids, or both LC#2 and AP#2 must be solids, but do not have to be the same solid). This is due to how the recipes are shuffled.  
 - If you attempt to search something that doesn't meet these requirements, narg will still attempt to search for the recipe, but will not return any results. Typos or incorrect ingredient names will be defaulted to a wildcard during search (using the `--list` option will show the possible ingredient names for both categories).  
 - Since ingredient #2 for both recipes is the ingredient that is consumed, ingredients #1 & #3 are treated interchangeably for both recipes.  
 
  
  
  Example: `narg -s oil water blood oil water alcohol`  
Above example will search for a seed with a Lively Concoction recipe with oil, water, and blood, and an Alchemic Precursor recipe with oil, water, and alcohol.  
  
If you have any questions or encounter any issues, feel free to message me on Discord or ping me in the Noita discord server @Neff#6398.

![Sample output from search](/narg-output.png?raw=true)
