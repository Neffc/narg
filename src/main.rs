extern crate getopts;
use getopts::Options;
use std::env;
use std::mem;


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
    let liquids = ["water","water_ice","water_swamp","oil","alcohol","swamp","mud","blood","blood_fungi","blood_worm","radioactive_liquid","cement","acid","lava","urine","poison","magic_liquid_teleportation","magic_liquid_polymorph","magic_liquid_random_polymorph","magic_liquid_berserk","magic_liquid_charm","magic_liquid_invisibility"];
    let solids = ["sand","bone","soil","honey","slime","snow","rotten_meat","wax","gold","silver","copper","brass","diamond","coal","gunpowder","gunpowder_explosive","grass","fungi"];
    opts.optflag("a", "array", "prints recipe in comma-delimited format (seed#,x,x,x,y,y,y)");
    opts.optflag("s", "search", "returns recipe only if specified ingredients are present (WIP, doesn't really work :) )");
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
     if matches.opt_present("s") {
        v = v + 10;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_about();
        print_usage(opts);
        return;
    };
    let seed = input.parse::<i64>().unwrap();
    let fseed = input.parse::<f64>().unwrap();
    let nseed = (seed >> 1) + 12534;
    let iseed = init(fseed);
    recipe(seed,iseed,nseed,liquids,solids,v);
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
        i = i + 1;
        iseed = 16807 * (iseed % 127773) - 2836 * (iseed / 127773);
        if iseed < 0 {
            iseed = iseed + i32::max_value() as i64;
        };
    }
    return iseed;
}

fn recipe(seed: i64, iseed: i64, nseed: i64, liquids: [&str; 22], solids: [&str; 18], v: usize) {
    let mut lc: [&str; 4] = [""; 4];
    let array = pick_materials(iseed);
    lc[0] = liquids[array[0] as usize];
    lc[1] = liquids[array[1] as usize];
    lc[2] = liquids[array[2] as usize];
    lc[3] = solids[array[3] as usize];
    let iseed = lgm_random(iseed,6);
    let mut ap: [&str; 4] = [""; 4];
    let array = pick_materials(iseed);
    ap[0] = liquids[array[0] as usize];
    ap[1] = liquids[array[1] as usize];
    ap[2] = liquids[array[2] as usize];
    ap[3] = solids[array[3] as usize];
    let lc: [&str; 4] = shuffle(lc,nseed);
    let ap: [&str; 4] = shuffle(ap,nseed);
    if v == 0 {
        print_recipe(seed,lc,ap);
    };
    if v == 1 {
        print_array(seed,lc,ap);
    };
    if v == 10 {
        print_search(seed,lc,ap);
    };
    if v == 11 {
        println!("Whoops!");
    };
}

fn pick_materials(mut iseed: i64) -> [i64; 4] {
    let mut array: [i64; 4] = [25,26,27,28];
    let int32 = i32::max_value();
    let mut i = 0;
    while i < 3 {
        iseed = lgm_random(iseed,1);
        array[i] = (iseed as f64 / int32 as f64 * 22 as f64) as i64;
        i = i + 1;
        if array[0] == array[1] {
            i = i - 1;
        };
        if array[0] == array[2] {
            i = i - 1;
        };
        if array[0] == array[3] {
            i = i - 1;
        };
        if array[1] == array[2] {
            i = i - 1;
        };
        if array[1] == array[3] {
            i = i - 1;
        };
        if array[2] == array[3] {
            i = i - 1;
        };
    }
    iseed = lgm_random(iseed,1);
    array[3] = (iseed as f64 / int32 as f64 * 18 as f64) as i64;
    return array;
}

fn shuffle(mut array: [&str; 4], nseed: i64) -> [&str; 4] {
    let mut i = 3;
    let mut x = "error";
    let mut iseed = lgm_random(nseed,1);
    while i>= 0 {
        let shuffle = (lgm_random(iseed,1) as f64 / i32::max_value() as f64 * (i + 1) as f64) as usize;
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

fn print_recipe(seed: i64, lc: [&str; 4], ap: [&str; 4]) {
    println!("\nSeed: {}",seed);
    println!("Lively Conction: {}, {}, {}",lc[0],lc[1],lc[2]);
    println!("Alchemical Precursor: {}, {}, {}\n",ap[0],ap[1],ap[2]);
}

fn print_array(seed: i64, lc: [&str; 4], ap: [&str; 4]) {
    println!("{},{},{},{},{},{},{}",seed,lc[0],lc[1],lc[2],ap[0],ap[1],ap[2]);
}

fn print_search(seed: i64, lc: [&str; 4], ap: [&str; 4]) {
    if lc[1] == "water" &&
    (lc[2] == "oil" || lc[2] == "blood") &&
    (lc[0] == "oil" || lc[0] == "blood") {
        println!("{},{},{},{},{},{},{}",seed,lc[0],lc[1],lc[2],ap[0],ap[1],ap[2]);
    };
}
