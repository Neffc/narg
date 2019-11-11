extern crate getopts;
use getopts::Options;
use std::env;
use std::mem;


const LIQUIDS: [&str; 22] = ["water","water_ice","water_swamp",
    "oil","alcohol","swamp",
    "mud","blood","blood_fungi",
    "blood_worm","radioactive_liquid","cement",
    "acid","lava","urine",
    "poison","magic_liquid_teleportation","magic_liquid_polymorph",
    "magic_liquid_random_polymorph","magic_liquid_berserk","magic_liquid_charm",
    "magic_liquid_invisibility"];
const SOLIDS: [&str; 18] = ["sand","bone","soil",
    "honey","slime","snow",
    "rotten_meat","wax","gold",
    "silver","copper","brass",
    "diamond","coal","gunpowder",
    "gunpowder_explosive","grass","fungi"];
const I32: i64 = i32::max_value() as i64;

fn print_about() {
    println!("\nNeff's Alchemical Recipe Generator (NARG)");
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: narg [options] SEED");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let mut v = 0;
    opts.optflag("a", "array", "prints in comma-delimited format (seed#,x,x,x,y,y,y)");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }
    if matches.opt_present("a") {
        v = v + 1;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_about();
        print_usage(opts);
        return;
    };
    let seed = input.parse::<i64>().unwrap();
    let fseed = seed as f64;
    let nseed = (seed >> 1) + 12534;
    let iseed = init(fseed);
    recipe(seed,iseed,nseed,v);
}

fn init(fseed: f64) -> i64 {
    let fseed = fseed * 0.17127000 + 1323.59030000;
    let iseed = fseed as i64;
    let iseed = lgm_random(iseed,6);
    return iseed;
}

fn lgm_random(mut iseed: i64, count: i64) -> i64 {
    let mut i = 0;
    while i < count {
        i+=1;
        iseed = 16807 * (iseed % 127773) - 2836 * (iseed / 127773);
        if iseed < 0 {
            iseed = iseed + I32;
        };
    }
    return iseed;
}

fn recipe(seed: i64, iseed: i64, nseed: i64, v: usize) {
    let mut mats: [&str; 6] = [""; 6];
    let mut array = pick_materials(iseed,nseed);
    let mut x = 2;
    let mut i = 0;
    while x > 0 {
        for n in 0..3 {
            mats[i] = array[n];
            i+=1;
        }
        let iseed = lgm_random(iseed,6);
        array = pick_materials(iseed,nseed);
        x-=1;
    };

    if v == 0 {
        print_recipe(seed,mats);
    };
    if v == 1 {
        print_array(seed,mats);
    };
}

fn pick_materials(mut iseed: i64, nseed: i64) -> [&'static str; 4] {
    let mut shuf: [i64; 4] = [25,26,27,28];
    let mut array: [&str; 4] = [""; 4];
    let mut i = 0;
    while i < 3 {
        iseed = lgm_random(iseed,1);
        shuf[i] = (iseed as f64 / I32 as f64 * LIQUIDS.len() as f64) as i64;
        i = i + 1;
        if shuf[0] == shuf[1] {
            i = i - 1;
        };
        if shuf[0] == shuf[2] {
            i = i - 1;
        };
        if shuf[1] == shuf[2] {
            i = i - 1;
        };
    }
    iseed = lgm_random(iseed,1);
    shuf[3] = (iseed as f64 / I32 as f64 * SOLIDS.len() as f64) as i64;
    array[0] = LIQUIDS[shuf[0] as usize];
    array[1] = LIQUIDS[shuf[1] as usize];
    array[2] = LIQUIDS[shuf[2] as usize];
    array[3] = SOLIDS[shuf[3] as usize];
    array = shuffle(array,nseed);
    return array;
}

fn shuffle(mut array: [&str; 4], nseed: i64) -> [&str; 4] {
    let mut i = 3;
    let mut x = "NA";
    let mut iseed = lgm_random(nseed,1);
    while i>= 0 {
        let shuffle = (lgm_random(iseed,1) as f64 / I32 as f64 * (i + 1) as f64) as usize;
        if shuffle != i as usize {
            mem::swap(&mut x, &mut array[shuffle]);
            mem::swap(&mut x, &mut array[i as usize]);
            mem::swap(&mut x, &mut array[shuffle]);
        };
        iseed = lgm_random(iseed,1);
        i = i - 1;
    }
    return array;
}

fn print_recipe(seed: i64, mats: [&str; 6]) {
    println!("\nSeed: {}",seed);
    println!("Lively Concoction: {}, {}, {}",mats[0],mats[1],mats[2]);
    println!("Alchemical Precursor: {}, {}, {}\n",mats[3],mats[4],mats[5]);
}

fn print_array(seed: i64, mats: [&str; 6]) {
    println!("{},{},{},{},{},{},{}",seed,mats[0],mats[1],mats[2],mats[3],mats[4],mats[5]);
}