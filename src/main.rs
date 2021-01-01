extern crate getopts;
extern crate rayon;
use getopts::Options;
use std::time::Instant;
use std::env;
use alchemy::LIQUIDS;
use alchemy::SOLIDS;
use alchemy::init;
use alchemy::recipe;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

mod alchemy;

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
    opts.optflag("l", "list", "list all possible alchemy ingredients");
    opts.optflag("s", "search", "search all seeds for a given recipe
                \n-> (can use * to sub any ingredient as a wildcard)
                \n-> (can use -p to enable parallel search mode)");
    opts.optflag("p", "parallel", "use multiple processor threads in parallel in search mode");
    opts.optflag("d", "debug", "prints calculated values with seed; ignored when using search flag");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[0..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let input = if !matches.free.is_empty() && args.len() > 1 {
        matches.free.clone()
    } else {
        print_about();
        print_usage(opts);
        return;
    };
    if matches.opt_present("l") {
        println!("\nLiquids:{:?}",LIQUIDS);
        println!("\nSolids:{:?}\n",SOLIDS);
        return;
    }
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }
    if matches.opt_present("s"){
        v+=10;
        if input.len() != 7 {
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
        let s_mats: [&str; 6] = [&input[1],&input[2],&input[3],&input[4],&input[5],&input[6]];
        search(s_mats,v,matches.opt_present("p"));
        return;
    }
    let seed = input[1].parse::<i64>().unwrap();
    if seed > SEEDMAX {
        println!("Seeds only go up to {}, try again.",SEEDMAX);
        return;
    }
    let (lc,lc_prob,iseed) = recipe(seed,init(seed));
    let (ap,ap_prob,_x) = recipe(seed,iseed);
    print_recipe(seed,lc,ap,v,[lc_prob, ap_prob]);
    if matches.opt_present("d") {
        print_debug(seed,lc[3],ap[3]);
        return;
    }
}

fn print_recipe(seed: i64, lc: [&str; 4], ap: [&str; 4], v: usize, prob: [i64; 2]) {
    match v {
        0 => {
            println!("\nSeed: {}
            \rLively Concoction: {}, {}, {}
            \rAlchemic Precursor: {}, {}, {}
            \rLively Concoction Probability: {}%; Alchemic Precursor Probability: {}%\n"
            ,seed,lc[0],lc[1],lc[2],ap[0],ap[1],ap[2],prob[0],prob[1]);
            },
        10 => println!("Seed: {}    LC: {},{},{}    AP: {},{},{}",seed,lc[0],lc[1],lc[2],ap[0],ap[1],ap[2]),
        _ => return,
    }
}

fn search(mut s_mats: [&str; 6], v: usize, parallel: bool) {
    let now = Instant::now();
  	for n in 0..6 {
    	if !(LIQUIDS.contains(&s_mats[n])) && !(SOLIDS.contains(&s_mats[n])) && s_mats[n] != "*" {
        	println!("'{}' not found! Replacing with wildcard (*)!",s_mats[n]);
        	s_mats[n] = "*";
    	}
    }
    if !parallel { // limit iterator to 1 thread if not using parallel mode
        rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    }
    let iter = (1..SEEDMAX).into_par_iter(); // rayon library parallel iterator

    println!("Searching for:{:?}",s_mats);
    let j: i64 = iter.map(|seed| {
        let (lc,lc_prob,iseed) = recipe(seed,init(seed));
        if is_valid(lc,[s_mats[0],s_mats[1],s_mats[2]]) == true {
            let (ap,ap_prob,_x) = recipe(seed,iseed);
            if is_valid(ap,[s_mats[3],s_mats[4],s_mats[5]]) == true {
                print_recipe(seed,lc,ap,v,[lc_prob, ap_prob]);
                return 1;
            }
        }
        return 0;
    }).sum();
    println!("Time elapsed: {}.{} seconds!",now.elapsed().as_secs(), now.elapsed().subsec_millis());
    println!("{} results found!",j);
}

fn is_valid(mats: [&str; 4], s_mats: [&str; 3]) -> bool{
    if (mats[1] == s_mats[1] || s_mats[1] == "*") &&
       (mats[0] == s_mats[0] || mats[2] == s_mats[0] || s_mats[0] == "*") &&
       (mats[0] == s_mats[2] || mats[2] == s_mats[2] || s_mats[2] == "*") {
        return true;
    } else {
        return false
    }
}

fn print_debug(seed: i64, lc: &str, ap: &str) {
	let fmax: f64 = i32::max_value() as f64;
	let mut iseed = init(seed);
	for n in 0..14 {
		println!("PRNG value #{}: {}", n,iseed);
		iseed = alchemy::lgm_random(iseed,1);
	}
	let mut nseed = (seed as i64 >> 1) + 12534;
	println!("Shuffle seed: {}",nseed);
    nseed = alchemy::lgm_random(nseed,1);
    let mut index: [usize; 3] = [0; 3];
    for n in 0..3 {
        nseed = alchemy::lgm_random(nseed,1);
        index[n] = (nseed as f64 / fmax * ( (3 - n) as f64 + 1.0)) as usize;
    }
    println!("Shuffle index: {:?}",index);
    println!("LC ingredient shuffled out: {}",lc);
    println!("AP ingredient shuffled out: {}",ap);
}