pub const LIQUIDS: [&str; 30] = ["acid","alcohol","blood",
    "blood_fungi","blood_worm","cement",
    "lava","berserkium","pheromone",
    "levitatium","hastium","invisiblium",
    "concentrated_mana","acceleratium","ambrosia",
    "teleportatium","unstable_polymorphine","unstable_teleportatium",
    "worm_pheromone","flummoxium","mud",
    "oil","poison","toxic_sludge",
    "swamp","urine","water",
    "water_ice","water_swamp","chaotic_polymorphine"];
pub const SOLIDS: [&str; 18] = ["bone","brass","coal",
    "copper","diamond","fungus",
    "gold","grass","gunpowder",
    "gunpowder_explosive","rotten_meat","sand",
    "silver","slime","snow",
    "soil","wax","honey"];
const I32: i64 = i32::max_value() as i64;
const F32: f64 = i32::max_value() as f64;

//prepare seed for prng function
pub fn init(seed: i64) -> i64 {
    let iseed = (seed as f64 * 0.17127000 + 1323.59030000) as i64;
    let iseed = lgm_random(iseed,6);
    return iseed;
}

//core prng function
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

//calculate materials and misnomer "probability" value
pub fn recipe(seed: i64, mut iseed: i64) -> ([&'static str; 4], i64, i64) {
    let mut index: [usize; 4] = [255; 4];
    let mut mats: [&str; 4] = [""; 4];
    let mut x = 0;
    while x < 3 {
        iseed = lgm_random(iseed,1);
        let tmp = (iseed as f64 / F32 * LIQUIDS.len() as f64) as usize;
        if !(index.contains(&tmp)) {
            index[x] = tmp;
            x+=1;
        }    //if duplicate liquid, discard and recycle iseed
    }
    iseed = lgm_random(iseed,1);
    index[3] = (iseed as f64 / F32 * SOLIDS.len() as f64) as usize;
    for n in 0..3 {
        mats[n] = LIQUIDS[index[n]];
    }
    mats[3] = SOLIDS[index[3]];
    mats = shuffle(mats,seed);
    iseed = lgm_random(iseed,1);
    let prob = 10 - ((iseed as f64 / F32) as f64 * -91.0) as i64;
    iseed = lgm_random(iseed,1);
    return (mats,prob,iseed);
}

//shuffle materials; material in last position is then omitted from recipe
fn shuffle (mut mats: [&str; 4], seed: i64) -> [&str; 4] {
    let mut nseed = (seed as i64 >> 1) + 12534;
    nseed = lgm_random(nseed,1);
    let mut index: [usize; 3] = [0; 3];
    for n in 0..3 {
        nseed = lgm_random(nseed,1);
        index[n] = (nseed as f64 / F32 * ( (3 - n) as f64 + 1.0)) as usize;
    }
    let mut x: usize = 3;
    for n in 0..3 {
        mats.swap(x, index[n]);
        x-=1;
    }
    return mats;
}