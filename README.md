NARG  
(Neff's Alchemy Recipe Generator)  
  
A tool to generate alchemy recipes for Noita. Contains robust search capability to find seeds with specific LC / AP formulas.
  
Usage: narg [options] SEED  
Usage: narg -s [LC#1, LC#2, LC#3, AP#1, AP#2, AP#3]  
Options:  
    `-l`, `--list`          list all possible alchemy ingredients  
    `-s`, `--search`        search all seeds for a given recipe  
                            -> (can use * to sub any ingredient as a wildcard)  
    `-d`, `--debug`         prints calculated values with seed; ignored when using search flag  
    `-h`, `--help`          print this help menu  
  
Info for recipe search:  
 - Each recipe will contain at most 1 solid ingredient. Valid recipes contain either 3 liquids, or 2 liquids & 1 solid. 6 ingredients total are needed to perform a search.  
 - Seeds are unable to generate recipes containing duplicate ingredients, or elements that aren't contained in the liquids or solids categories.  
 - If searching for a recipe with an ingredient in the solids category, both recipes must include a solid at the SAME POSITION (i.e. both LC#1 and AP#1 must be solids, or both LC#2 and AP#2 must be solids, but do not have to be the same solid). This is due to how the recipes are shuffled.  
 - If you attempt to search something that doesn't meet these requirements, narg will still attempt to search for the recipe, but will not return any results. Typos or incorrect ingredient names will be defaulted to a wildcard during search (using the `--list` option will show the possible ingredient names for both categories).  
 - Since ingredient #2 for both recipes is the ingredient that is consumed, ingredients #1 & #3 are treated interchangeably for both recipes.  
 
  
  
  Example: `narg -s blood water oil water oil alcohol`  
Above example will search for a seed with a Lively Concoction recipe with blood, water, and oil, and an Alchemic Precursor recipe with water, oil, and alcohol.  
  
If you have any questions or encounter any issues, feel free to message me on Discord or ping me in the Noita discord server @Neff#6398.

![Sample output from search](/narg-output.png?raw=true)
