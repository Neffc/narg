# NARG  
(Neff's Alchemy Recipe Generator)  
  
A tool to generate alchemy recipes for Noita. Contains robust search capability to find seeds with specific LC / AP formulas.
  
# Usage 
  
narg [options] SEED  
narg -s [`LC#1` `LC#2` `LC#3` `AP#1` `AP#2` `AP#3`]
  
Options:  
    `-l`, `--list`          list all possible alchemy ingredients  
    `-s`, `--search`        search all seeds for a given recipe  
                            -> (can use * to sub any ingredient as a wildcard)  
                            -> (can use -p to enable parallel search mode)  
    `-p`, `--parallel`      use multiple processor threads in parallel in search mode  
    `-d`, `--debug`         prints calculated values with seed; ignored when using search flag  
    `-h`, `--help`          print this help menu  
  
# Searching for recipes:  
 - Each recipe will contain at most 1 solid ingredient. Valid recipes contain either 3 liquids, or 2 liquids & 1 solid. 6 ingredients total are needed to perform a search.  
 - Seeds are unable to generate recipes containing duplicate ingredients, or elements that aren't contained in the liquids or solids categories.  
 - If searching for a recipe with an ingredient in the solids category, both recipes must include a solid at the SAME POSITION (i.e. both LC#1 and AP#1 must be solids, or both LC#2 and AP#2 must be solids, but do not have to be the same solid). This is due to how the recipes are shuffled.  
 - If you attempt to search something that doesn't meet these requirements, narg will still attempt to search for the recipe, but will not return any results. Typos or incorrect ingredient names will be defaulted to a wildcard during search (using the `--list` option will show the possible ingredient names for both categories).  
 - Since ingredient #2 for both recipes is the ingredient that is consumed, ingredients #1 & #3 are treated interchangeably for both recipes.  
 
  
  
  Example: `narg -s mud water oil blood alcohol water`  
Above string will search for a seed with a Lively Concoction recipe with mud, water, and oil, and an Alchemic Precursor recipe with blood, alcohol, and water.  
  
![Sample output from search](/narg-output.png?raw=true)  
  
# Frequently Asked Questions  
  
### What is the "probability" percentage?  
  This is a misnomer. The probability value describes the rate at which the reaction occurs, not how likely a recipe is to be correct. However, this is the word Nolla uses internally to describe the reaction rate, which is why most tools will refer to it as such. 
  
  There isn't any guesswork in calculating the recipes: either the tool is correctly updated or it isn't. I've hidden this value by default in all of my tools, as it is usually insignificant to the player, and is generally confusing.  
  
### The mod/webpage/program is giving different results than the mod/webpage/program!
  All of the tools I've created use the exact same algorithm. If you're having issues with the mod not working, make sure you're using one that's up to date (e.g. [modworkshop.net/mod/29782](https://modworkshop.net/mod/29782)).  
  
### The recipe isn't working, and it includes sand/gunpowder/swamp.  
 - Sand in the desert has a different material ID, and is not the same sand used in alchemy (`sand` vs `sand_surface`). 
 - `swamp` and `water_swamp` have the same UI name (shown near toolbar when hovering with cursor). `water_swamp` is slightly lighter in color than `swamp`.  
 - The materials `gunpowder`, `gunpowder_explosive`, and `gunpowder_tnt` also have the same UI name. Additionally, `gunpowder_explosive` and `gunpowder_tnt` look identical, but `gunpowder_tnt` isn't used in alchemy.  
  
I've brought this up to Nolla, as this is a common source of confusion, but until they change it certain recipes will continue to be somewhat difficult to interpret.  
  
##  
  
If you have any questions or encounter any issues, feel free to message me on Discord or ping me in the Noita discord server @Neff#6398.

