extern crate getopts;
use getopts::Options;
use std::time::Instant;
use std::env;
use alchemy::alchemy;
use alchemy::lgm_random;
use alchemy::materials;
use alchemy::init;
use alchemy::init_shuffle;

mod alchemy;

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
    opts.optflag("x", "silent", "print no output");
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
    if matches.opt_present("x") {
        v+=100;
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
    let (mats,prob) = alchemy(seed);
    print_recipe(seed,mats,v,prob);
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
        _ => return,
    }
}

fn search(mut s_mats: [&str; 6], mut seed: i64, v: usize) {
    let now = Instant::now();
    let mut i = SEEDMAX;
    let mut j = 0;
    let mut x = 0;
    let mut y = 0;
    for n in 0..6 {
        if LIQUIDS.contains(&s_mats[n]) {
            x+=1;
        } else if SOLIDS.contains(&s_mats[n]) {
            y+=1;
        } for n in 0..6 {
            if !(LIQUIDS.contains(&s_mats[n])) && !(SOLIDS.contains(&s_mats[n])) && s_mats[n] != "*" {
                println!("'{}' not found! Replacing with wildcard (*)!",s_mats[n]);
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
                    j+=1;
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
                    j+=1;
                }
            }
        } else {
            let shuf = init_shuffle(seed);
            let (mats,prob) = recipe_with_search(iseed,shuf,s_mats);
            if is_valid(mats,s_mats) == true {
                print_recipe(seed,mats,v,prob);
                j+=1;
            }
        }
        seed+=1;
        i-=1;
    }
    println!("Time elapsed: {}.{} seconds!",now.elapsed().as_secs(), now.elapsed().subsec_millis());
    println!("{} results found!",j);
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