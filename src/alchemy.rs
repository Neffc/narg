const LIQUIDS: [&str; 30] = ["acid","alcohol","blood",
    "blood_fungi","blood_worm","cement",
    "lava","berserkium","pheromone",
    "levitatium","hastium","invisiblium",
    "concentrated_mana","acceleratium","ambrosia",
    "teleportatium","unstable_polymorphine","unstable_teleportatium",
    "worm_pheromone","flummoxium","mud",
    "oil","poison","toxic_sludge",
    "swamp","urine","water",
    "water_ice","water_swamp","chaotic_polymorphine"];
const SOLIDS: [&str; 18] = ["bone","brass","coal",
    "copper","diamond","fungus",
    "gold","grass","gunpowder",
    "gunpowder_explosive","rotten_meat","sand",
    "silver","slime","snow",
    "soil","wax","honey"];
const I32: i64 = i32::max_value() as i64;
const F32: f64 = i32::max_value() as f64;

pub fn alchemy(seed:i64) -> ([&'static str; 6], [i64; 2]) {
    let iseed = init(seed as f64);
    let shuf = init_shuffle(seed);
    let (mats,prob) = recipe(iseed,shuf);
    return (mats,prob);
}

pub fn init(seed: f64) -> i64 {
    let iseed = (seed * 0.17127000 + 1323.59030000) as i64;
    let iseed = lgm_random(iseed,5);
    return iseed;
}

pub fn init_shuffle(seed: i64) -> [usize; 3] {
    let mut nseed = (seed as i64 >> 1) + 12534;
    nseed = lgm_random(nseed,1);
    let mut shuf: [usize; 3] = [0; 3];
    for n in 0..3 {
        nseed = lgm_random(nseed,1);
        shuf[n] = (nseed as f64 / F32 * ( (3 - n) as f64 + 1.0)) as usize;
    }
    return shuf;
}

pub fn lgm_random(mut iseed: i64, mut count: i64) -> i64 {
    while count > 0 {
        iseed = 16807 * (iseed % 127773) - 2836 * (iseed / 127773);
        if iseed < 0 {
            iseed = iseed + I32;
        };
        count-=1;
    }
    return iseed;
}

fn recipe(mut iseed: i64, shuf: [usize; 3]) -> ([&'static str; 6], [i64; 2]) {
    let mut mats: [&str; 6] = [""; 6];
    let mut i = 0;
    let mut prob: [i64; 2] = [0,0];
    for x in 0..2 {
        iseed = lgm_random(iseed,1);
        let (array,tmp) = materials(iseed,shuf);
        iseed = tmp;
        for n in 0..3 {
            mats[i] = array[n];
            i+=1;
        }
        iseed = lgm_random(iseed,1);
        prob[x] = 10 - ((iseed as f64 / F32) as f64 * -91.0) as i64;
    };
    return (mats,prob);
}

pub fn materials(mut iseed: i64, shuf: [usize; 3]) -> ([&'static str; 4], i64) {
    let mut index: [usize; 4] = [255; 4];
    let mut array: [&str; 4] = [""; 4];
    let mut i = 0;
    while i < 3 {
        iseed = lgm_random(iseed,1);
        let tmp = (iseed as f64 / F32 * LIQUIDS.len() as f64) as usize;
        if !(index.contains(&tmp)) {
            index[i] = tmp;
            i+=1;
        }    //Check for and remove duplicate liquids.
    }
    iseed = lgm_random(iseed,1);
    index[3] = (iseed as f64 / F32 * SOLIDS.len() as f64) as usize;
    for n in 0..3 {
        array[n] = LIQUIDS[index[n]];
    }
    array[3] = SOLIDS[index[3]];
    array = shuffle(array,shuf);
    return (array,iseed);
}

fn shuffle (mut array: [&str; 4], shuf: [usize; 3]) -> [&str; 4] {
    let mut i: usize = 3;
    for n in 0..3 {
        if shuf[n] != i {
            array.swap(i, shuf[n]);
        }
        i-=1;
    }
    return array;
}

