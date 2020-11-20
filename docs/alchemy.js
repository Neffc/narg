const Liquids = ["Acid","Whiskey (alcohol)","Blood",
  "Fungus Blood","Worm Blood","Cement",
  "Lava","Berserkium","Pheromone",
  "Levitatium","Hastium","Invisiblium",
  "Concentrated Mana","Acceleratium","Ambrosia",
  "Teleportatium","Unstable Polymorphine","Unstable Teleportatium",
  "Worm Pheromone","Flummoxium","Mud",
  "Oil","Poison","Toxic Sludge (radioactive_liquid)",
  "Swamp (swamp)","Urine","Water",
  "Chilly Water (water_ice)","Swamp (water_swamp)","Chaotic Polymorphine"];
const Solids = ["Bone Dust (bone)","Brass","Coal",
  "Copper","Diamond","Fungus (fungi)",
  "Gold","Grass","Gunpowder",
  "Explosive Gunpowder","Rotten Meat","Sand",
  "Silver","Slime","Snow",
  "Soil","Wax","Honey"];

const LiquidsBeta = ["Water","Chilly Water (water_ice)","Swamp Water (water_swamp)",
  "Oil","Whiskey (alcohol)","Swamp (swamp)",
  "Mud","Blood","Fungus (blood_fungi)",
  "Worm Blood","Toxic Sludge (radioactive_liquid)","Cement",
  "Acid","Lava","Urine",
  "Poison (glowing_liquid)","Teleportatium","Polymorphine",
  "Chaotic Polymorphine","Berserkium","Pheromone",
  "Invisiblium"];
const SolidsBeta = ["Sand","Bone Dust (bone)","Soil",
  "Honey","Slime","Snow",
  "Rotten Meat","Wax","Gold",
  "Silver","Copper","Brass",
  "Diamond","Coal","Gunpowder",
  "Explosive Gunpowder","Grass","Fungus (fungi)"];

const I32 = 2147483647;
const SEEDMAX = 4294967295;

function main (seed) {
  var iseed = init(seed);
  var shuf = initShuffle(seed);
  var tmp = recipe(iseed,shuf);
  return tmp;
}

//prepares rng 'iseed' for recipe calculations
function init (seed) {
  var iseed = parseInt(seed * 0.17127000 + 1323.59030000);
  iseed = lgmRandom(iseed,5);
  return iseed;
}

//calculates shuffle indices for later use
function initShuffle (seed) {
  var nseed = (seed >> 1) + 12534;
  nseed = lgmRandom(nseed,1);
  var shuf = [];
  for (n = 0; n < 3; n++) {
    nseed = lgmRandom(nseed,1);
    shuf[n] = parseInt(nseed / I32 * (3 - n + 1));
  }
  return shuf;
}

//rng calculation; takes number and a 'count' var for iterations
function lgmRandom (iseed,count) {
  while (count > 0) {
    iseed = 16807 * (iseed % 127773) - 2836 * parseInt(iseed / 127773);
    if (iseed < 0) {
      iseed = iseed + I32;
    };
    count--;
  }
  return iseed;
}

//pass rng value to materials fn and calculate probability
function recipe(iseed,shuf) {
  var mats = [];
  var i = 0;
  var prob = [];
  for (x = 0; x < 2; x++) {
    iseed = lgmRandom(iseed,1);
    var tmp = materials(iseed,shuf);
    var array = tmp[0];
    iseed = tmp[1];
    for (n = 0; n < 3; n++) {
      mats[i] = array[n];
      i++;
    }
    iseed = lgmRandom(iseed,1);
    prob[x] = 10 - parseInt(iseed / I32 * -91);
  }
  var tmp = [mats,prob];
  return tmp;
}

//select materials from ingredients list by index
function materials(iseed,shuf) {
  var index = [];
  var array = [];
  var i = 0;
  if (oldrecipes.checked == false) {
    while (i < 3) {
      iseed = lgmRandom(iseed,1);
      var tmp = parseInt(iseed / I32 * Liquids.length);
      if (!index.includes(tmp)) {
        index[i] = tmp;
        i++;
      }
    }
    iseed = lgmRandom(iseed,1);
    index[3] = parseInt(iseed / I32 * Solids.length);
    for (n = 0; n < 3; n++) {
      array[n] = Liquids[index[n]];
    }
    array[3] = Solids[index[3]];
  } else {
    while (i < 3) {
      iseed = lgmRandom(iseed,1);
      var tmp = parseInt(iseed / I32 * LiquidsBeta.length);
      if (!index.includes(tmp)) {
        index[i] = tmp;
        i++;
      }
    }
    iseed = lgmRandom(iseed,1);
    index[3] = parseInt(iseed / I32 * SolidsBeta.length);
    for (n = 0; n < 3; n++) {
      array[n] = LiquidsBeta[index[n]];
    }
    array[3] = SolidsBeta[index[3]];
  }
  array = shuffle(array,shuf);
  var index = [array,iseed];
  return index;
}

//shuffle array with shuffle indices calculated earlier
function shuffle(array,shuf) {
  var i = 3;
  for (n = 0; n < 3; n++) {
    if (shuf[n] != i) {
      [array[i], array[shuf[n]]] = [array[shuf[n]], array[i]];
    }
    i--;
  }
  return array;
}

//print to console
function print(seed,tmp) {
  var mats = tmp[0];
  var prob = tmp[1];
  console.log("\nSeed:",seed);
  console.log("Lively Concoction:",mats[0],mats[1],mats[2]);
  console.log("Alchemic Precursor:",mats[3],mats[4],mats[5]);
  console.log("Lively Concoction Probability:",prob[0]+"%;","Alchemic Precursor Probability:",prob[1]+"%");
}
