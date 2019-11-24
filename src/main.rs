extern crate getopts;
use getopts::Options;
use std::time::Instant;
use std::env;


const LIQUIDS: [&str; 22] = ["water","water_ice","water_swamp",
    "oil","alcohol","swamp",
    "mud","blood","blood_fungi",
    "blood_worm","toxic_sludge","cement",
    "acid","lava","urine",
    "poison","teleportatium","polymorphine",
    "chaotic_polymorphine","berserkium","pheromone",
    "invisiblium"];
const SOLIDS: [&str; 18] = ["sand","bone","soil",
    "honey","slime","snow",
    "rotten_meat","wax","gold",
    "silver","copper","brass",
    "diamond","coal","gunpowder",
    "gunpowder_explosive","grass","fungus"];
const I32: i64 = i32::max_value() as i64;
const F32: f64 = i32::max_value() as f64;
const SEEDMAX: i64 = 4294967295;

fn print_about() {
    println!("\nNeff's Alchemy Recipe Generator (NARG)");
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: narg [options] SEED
                        \rUsage: narg -s [LC1, LC2, LC3, AP1, AP2, AP3]");
    print!("{}\n", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let mut v = 0;
    opts.optflag("a", "array", "print in comma-delimited format (seed#,x,x,x,y,y,y)");
    opts.optflag("s", "search", "search all seeds for a given recipe
                \n-> (can use * to sub any ingredient as a wildcard)");
    opts.optflag("l", "list", "list all possible alchemy ingredients");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let input = if !matches.free.is_empty() {
        matches.free.clone()
    } else {
        print_about();
        print_usage(opts);
        return;
    };
    if matches.opt_present("a") {
        v+=1;
    }
    if matches.opt_present("l") {
        println!("\nLiquids:{:?}",LIQUIDS);
        println!("\nSolids:{:?}\n",SOLIDS);
        return;
    }
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }
    if matches.opt_present("s") {
        v+=10;
        if input.len() != 6 {
            println!("\n  Searching requires 6 ingredients. Check your ingredients, and try
                    \ragain. Use option '-l' or see the README.md for additional help.\n
                    \r    -s, --search  Searches all {} seeds for a given recipe.\n
                    \r  Since ingredient #2 for both recipes is the ingredient that is
                    \rconsumed, ingredients #1 & #3 are treated interchangeably.\n
                    \r    Example: narg -s oil water blood oil water alcohol\n
                    \r  Above example will search for a seed with a Lively Concoction recipe with
                    \roil, water, and blood, and an Alchemic Precursor recipe with oil, water, and alcohol.
                    ",SEEDMAX);
            return;
        }
        let s_mats: [&str; 6] = [&input[0],&input[1],&input[2],&input[3],&input[4],&input[5]];
        let seed = 1;
        search(s_mats,seed,v);
        return;
    }
    let seed = input[0].parse::<i64>().unwrap();
    if seed > SEEDMAX {
        println!("Seeds only go up to {}, try again.",SEEDMAX);
        return;
    }
    let iseed = init(seed as f64);
    let shuf = init_shuffle(seed);
    let (mats,prob) = recipe(iseed,shuf);
    print_recipe(seed,mats,v,prob);
}

fn init(seed: f64) -> i64 {
    let iseed = (seed * 0.17127000 + 1323.59030000) as i64;
    let iseed = lgm_random(iseed,5);
    return iseed;
}

fn init_shuffle(seed: i64) -> [usize; 3] {
    let mut nseed = (seed as i64 >> 1) + 12534;
    nseed = lgm_random(nseed,1);
    let mut shuf: [usize; 3] = [0; 3];
    for n in 0..3 {
        nseed = lgm_random(nseed,1);
        shuf[n] = (nseed as f64 / F32 * ( (3 - n) as f64 + 1.0)) as usize;
    }
    return shuf;
}

fn lgm_random(mut iseed: i64, mut count: i64) -> i64 {
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

fn recipe_with_search(mut iseed: i64, shuf: [usize; 3], s_mats: [&str; 6]) -> ([&'static str; 6], [i64; 2]) {
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
        if mats[1+3*x] == s_mats[1+3*x] || s_mats[1+3*x] == "*" {
            iseed = lgm_random(iseed,1);
            prob[x] = 10 - ((iseed as f64 / F32) as f64 * -91.0) as i64;
        } else {
            break;
        }
    };
    return (mats,prob);
}

fn materials(mut iseed: i64, shuf: [usize; 3]) -> ([&'static str; 4], i64) {
    let mut tmp: [usize; 4] = [25,26,27,28];
    let mut array: [&str; 4] = [""; 4];
    let mut i = 0;
    while i < 3 {
        iseed = lgm_random(iseed,1);
        tmp[i] = (iseed as f64 / F32 * LIQUIDS.len() as f64) as usize;
        if tmp[0] != tmp[1] && tmp[0] != tmp[2] && tmp[1] != tmp[2] {
            i+=1;
        };  //Check for and remove duplicate liquids.
    }
    iseed = lgm_random(iseed,1);
    tmp[3] = (iseed as f64 / F32 * SOLIDS.len() as f64) as usize;
    for n in 0..3 {
        array[n] = LIQUIDS[tmp[n]];
    }
    array[3] = SOLIDS[tmp[3]];
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

fn print_recipe(seed: i64, mats: [&str; 6], v: usize, prob: [i64; 2]) {
    match v {
        0 => {
            println!("\nSeed: {}
            \rLively Concoction: {}, {}, {}
            \rAlchemic Precursor: {}, {}, {}
            \rLively Concoction Probability: {}%; Alchemic Precursor Probability: {}%\n"
            ,seed,mats[0],mats[1],mats[2],mats[3],mats[4],mats[5],prob[0],prob[1]);
            },
        1 | 11 => println!("{},{},{},{},{},{},{}",seed,mats[0],mats[1],mats[2],mats[3],mats[4],mats[5]),
        10 => println!("Seed: {}    LC: {},{},{}    AP: {},{},{}",seed,mats[0],mats[1],mats[2],mats[3],mats[4],mats[5]),
        _ => println!("Check args?"),
    }
}

fn search(mut s_mats: [&str; 6], mut seed: i64, v: usize) {
    let now = Instant::now();
    let mut i = SEEDMAX;
    let mut x = 0;
    let mut y = 0;
    for n in 0..6 {
        if LIQUIDS.contains(&s_mats[n]) {
            x+=1;
        } else if SOLIDS.contains(&s_mats[n]) {
            y+=1;
        } for n in 0..6 {
            if !(LIQUIDS.contains(&s_mats[n])) && !(SOLIDS.contains(&s_mats[n])) {
                s_mats[n] = "*";
            }
        }
    }
    println!("Searching for:{:?}",s_mats);
    while i >= 0 {
        let iseed = init(seed as f64);
        if x == 6 {
            let check = LIQUIDS[(lgm_random(iseed,2) as f64 / F32 * LIQUIDS.len() as f64) as usize];
            if check == s_mats[0] || check == s_mats[1] || check == s_mats[2] {
                let shuf = init_shuffle(seed);
                let (mats,prob) = recipe_with_search(iseed,shuf,s_mats);
                if is_valid(mats,s_mats) == true {
                    print_recipe(seed,mats,v,prob);
                }
            }
        } else if (x+y) == 6 {
            let tmp = lgm_random(iseed,2);
            let check1 = LIQUIDS[(tmp as f64 / F32 * LIQUIDS.len() as f64) as usize];
            let check2 = SOLIDS[(tmp as f64 / F32 * SOLIDS.len() as f64) as usize];
            if s_mats.contains(&check1) || s_mats.contains(&check2) {
                let shuf = init_shuffle(seed);
                let (mats,prob) = recipe_with_search(iseed,shuf,s_mats);
                if is_valid(mats,s_mats) == true {
                    print_recipe(seed,mats,v,prob);
                }
            }
        } else {
            let shuf = init_shuffle(seed);
            let (mats,prob) = recipe_with_search(iseed,shuf,s_mats);
            if is_valid(mats,s_mats) == true {
                print_recipe(seed,mats,v,prob);
            }
        }
        seed+=1;
        i-=1;
    }
    println!("Time elapsed: {}.{} seconds!",now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn is_valid(mats: [&str; 6], s_mats: [&str; 6]) -> bool{
    if ((mats[1] == s_mats[1] || s_mats[1] == "*") && (mats[4] == s_mats[4] || s_mats[4] == "*")) &&
       (mats[0] == s_mats[0] || mats[2] == s_mats[0] || s_mats[0] == "*") &&
       (mats[0] == s_mats[2] || mats[2] == s_mats[2] || s_mats[2] == "*") &&
       (mats[3] == s_mats[3] || mats[5] == s_mats[3] || s_mats[3] == "*") &&
       (mats[3] == s_mats[5] || mats[5] == s_mats[5] || s_mats[5] == "*") {
        return true;
    } else {
        return false
    }
}