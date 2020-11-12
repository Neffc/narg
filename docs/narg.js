const Liquids = ["Acid","Alcohol","Blood",
  "Fungus Blood","Worm Blood","Cement",
  "Lava","Berserkium","Pheromone",
  "Levitatium","Hastium","Invisiblium",
  "Concentrated Mana","Acceleratium","Ambrosia",
  "Teleportatium","Unstable Polymorphine","Unstable Teleportatium",
  "Worm Pheromone","Flummoxium","Mud",
  "Oil","Poison","Toxic Sludge (radioactive_liquid)",
  "Swamp (swamp)","Urine","Water",
  "Chilly Water (water_ice)","Swamp (water_swamp)","Chaotic Polymorphine"];
const Solids = ["Bone","Brass","Coal",
  "Copper","Diamond","Fungus",
  "Gold","Grass","Gunpowder",
  "Explosive Gunpowder","Rotten Meat","Sand",
  "Silver","Slime","Snow",
  "Soil","Wax","Honey"];
const I32 = 2147483647;
const SEEDMAX = 4294967295;

var hash = document.location.hash;
seedbox.value = parseInt(hash.replace("#",""));
seedbox.addEventListener('input', collectSeed);

function collectSeed() {
  var seed = seedbox.value;
  if (seed.length > 0 && seed <= SEEDMAX) {
    seed = parseInt(seed);
    var tmp = main(seed);
    var mats = tmp[0];
    var prob = tmp[1];
    wseed.innerHTML = seed;
    lc1.innerHTML = mats[0];
    lc2.innerHTML = mats[1];
    lc3.innerHTML = mats[2];
    ap1.innerHTML = mats[3];
    ap2.innerHTML = mats[4];
    ap3.innerHTML = mats[5];
    var all = document.getElementsByTagName("td");
    for (var i = 0; i < all.length; i++) {
      all[i].style.color = '#262626';
    }
    maxalert.innerHTML = "";
  } else {
    wseed.innerHTML = "";
    lc1.innerHTML = "LC#1";
    lc2.innerHTML = "LC#2";
    lc3.innerHTML = "LC#3";
    ap1.innerHTML = "AP#1";
    ap2.innerHTML = "AP#2";
    ap3.innerHTML = "AP#3";
    var all = document.getElementsByTagName("td");
    for (var i = 0; i < all.length; i++) {
      all[i].style.color = '#737480';
    }
  }
  if (seed > SEEDMAX) {
    maxalert.innerHTML = "WARNING: Seeds only go up to 4294967295!";
  } else {
    maxalert.innerHTML = "";
  }
}

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