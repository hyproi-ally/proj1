// The Rust standard library introduces the Instant timer tool to measure how long rule-based reasoning (prog.run()) takes.
use std::time::Instant;
// Import the macro "ascent" from the crate/library "ascent".
use ascent::ascent;
// Introducing the file system module from the Rust standard library for reading facts files and memory state files.
use std::fs;
use ascent::aggregators::*;

// Define the main Ascent Datalog program.
// This block declares input and derived relations, and specifies inference rules.
ascent! {
pub struct FamilyProgram;


relation parent(String, String);
relation male(String);
relation female(String);
relation spouse(String, String);

// create a new relation has_child(x)
relation has_child(String);
has_child(x) <-- parent(x, _);

//create a new relation childless(x)
//relation childless(String);
//childless(x) <-- person(x), !has_child(x);
// Ascent macro does not parse !

//create a new relation 
relation male_childless(String);
    male_childless(x) <-- male(x), !has_child(x);

//childless(x) <-- if !has_child_set.contains(&x);

relation married(String, String);
married(x, y) <-- spouse(x, y);
married(y, x) <-- spouse(x, y);


relation grandparent(String, String);
grandparent(x, z) <-- parent(x, y), parent(y, z);

relation ancestor(String, String);
ancestor(x, y) <-- parent(x, y);
ancestor(x, z) <-- parent(x, y), ancestor(y, z);

relation sibling(String, String);
sibling(x, y) <-- parent(p, x), parent(p, y), if x != y;

relation uncle(String, String);
uncle(u, x) <-- parent(p, x), sibling(u, p), male(u);

relation aunt(String, String);
aunt(a, x) <-- parent(p, x), sibling(a, p), female(a);

relation cousin(String, String);
cousin(x, y) <--
    parent(px, x), parent(py, y),
    sibling(px, py),
    if x != y;

relation descendant(String, String);
descendant(y, x) <-- parent(x, y);
descendant(z, x) <-- parent(x, y), descendant(z, y);

relation parent_in_law(String, String);
parent_in_law(x, y) <-- married(y, s), parent(x, s);

relation brother_in_law(String, String);

brother_in_law(x, y) <--
    married(x, wife),
    female(wife),
    sibling(wife, sister),
    female(sister),
    married(sister, y),
    male(x),
    male(y),
    if x != y;

relation male_child_count(String, usize);
  male_child_count(x, cnt) <--
    male(x),
    agg cnt = count() in parent(x, _);


relation male_child_count(String, usize);
  male_child_count(x, cnt) <--
    male(x),
    agg cnt = count() in parent(x, _);

relation male_max_child_count(usize);
  male_max_child_count(m) <--
    agg m = max(cnt) in male_child_count(_, cnt);

relation male_most_children(String, usize);   // (person, count)
  male_most_children(x, cnt) <--
    male_child_count(x, cnt),
    male_max_child_count(cnt);


relation ancestor_count(String, usize);
  ancestor_count(x, cnt) <--
  ancestor(_, x),
    agg cnt = count() in ancestor(_, x);

}

/// Get the actual physical memory used by the current process (in MB)
fn get_memory_usage_mb() -> f64 {
    // Read the process memory status virtual file on Linux
    if let Ok(statm) = fs::read_to_string("/proc/self/statm") {
        let fields: Vec<&str> = statm.split_whitespace().collect();
        if fields.len() > 1 {
            // fields[1] represents the Resident Set Size (RSS) in pages
            if let Ok(pages) = fields[1].parse::<u64>() {
                // Linux default page size is typically 4KB (4096 bytes)
                // pages * 4 = KB, divided by 1024 = MB
                return (pages * 4) as f64 / 1024.0;
            }
        }
    }
    0.0
}

/*
fn bench_polonius_naive(clap_dir: &str, bench_name: &str) {
    // 1. Load facts data...
    
    let mem_before = get_memory_usage_mb();
    let before = Instant::now();
    
    // 2. Run the core inference engine
    prog.run(); 
    
    let elapsed = before.elapsed();
    let mem_after = get_memory_usage_mb();
    // 3. Print data ready to paste into your Notion/Excel table
    println!("\n=== Experiment Results ===");
    println!("Rule inference time: {:.3}s", elapsed.as_secs_f64());
    println!("Physical memory before inference: {:.2} MB", mem_before);
    println!("Physical memory after inference: {:.2} MB", mem_after);
    println!("Net memory overhead (Delta): {:.2} MB", mem_after - mem_before);
}
*/

// `load_2col` is used to read a two-column facts file and convert each row into a `(String, String)`.
fn load_2col(path: &str) -> Vec<(String, String)> {
match std::fs::read_to_string(path) {
Ok(content) => content
.lines()
.filter(|l| !l.trim().is_empty())
.map(|l| {
let mut it = l.split('\t');
let a = it.next().unwrap_or("").trim().trim_matches('"').to_string();
let b = it.next().unwrap_or("").trim().trim_matches('"').to_string();
(a, b)
})
.collect(),
Err(_) => { println!("Warning: file not found: {}", path); vec![] }
}
}


// `load_2col` is used to read a two-column facts file and convert each row into a `(String, String)`.
fn load_1col(path: &str) -> Vec<(String,)> {
match std::fs::read_to_string(path) {
Ok(content) => content
.lines()
.filter(|l| !l.trim().is_empty())
.map(|l| (l.trim().trim_matches('"').to_string(),))
.collect(),
Err(_) => { println!("Warning: file not found: {}", path); vec![] }
}
}


fn main() {
    let mut prog = FamilyProgram::default();
    prog.parent = load_2col("facts/parent.facts");
    prog.spouse = load_2col("facts/spouse.facts");
    prog.male   = load_1col("facts/male.facts");
    prog.female = load_1col("facts/female.facts");

    // ✅ mem_before must be captured BEFORE prog.run()
    let mem_before = get_memory_usage_mb();
    let start = Instant::now();

    prog.run();

    let elapsed = start.elapsed();
    let mem_after = get_memory_usage_mb();


    println!("\n=== Ranking by number of children (most → fewest) ===");
    let mut child_rank: Vec<_> = prog.male_child_count.iter().collect();
    child_rank.sort_by_key(|(_, cnt)| std::cmp::Reverse(*cnt));  // DESC
    for (rank, (person, cnt)) in child_rank.iter().take(100).enumerate() {
        let bar = "█".repeat(*cnt);
        println!("  #{:<3} {:<20} {} children    {}",
                 rank + 1, person, cnt, bar);

    }


     // ── SORT 3: ancestor_count — deepest in tree first ────────────
    println!("\n=== Ancestor Count Ranking (Youngest → Oldest Generation) ===");
    let mut anc_rank: Vec<_> = prog.ancestor_count.iter().collect();
    anc_rank.sort_by_key(|(_, cnt)| std::cmp::Reverse(*cnt));
    for (rank, (person, cnt)) in anc_rank.iter().take(100).enumerate() {
        println!("  #{:<3} {:<20} {} ancestors",
            rank + 1, person, cnt);
    }


/*
    let childless: Vec<String> = prog.male
    .iter()
    .map(|(x,)| x.clone())          // ✅ clone String
    .filter(|x| !prog.has_child.contains(&(x.clone(),)))  // ✅ clone for lookup too
    .collect();
*/

    println!("\n=== Experiment Results ===");
    println!("Rule inference time:              {:.3}s", elapsed.as_secs_f64());
    println!("Physical memory before inference: {:.2} MB", mem_before);
    println!("Physical memory after inference:  {:.2} MB", mem_after);
    println!("Net memory overhead (Delta):      {:.2} MB", mem_after - mem_before);

    println!("grandparent:    {} pairs", prog.grandparent.len());
    println!("ancestor:       {} pairs", prog.ancestor.len());
    println!("sibling:        {} pairs", prog.sibling.len());
    println!("uncle:          {} pairs", prog.uncle.len());
    println!("aunt:           {} pairs", prog.aunt.len());
    println!("cousin:         {} pairs", prog.cousin.len());
    println!("descendant:     {} pairs", prog.descendant.len());
    println!("parent_in_law:  {} pairs", prog.parent_in_law.len());
    println!("brother_in_law: {} pairs", prog.brother_in_law.len());
    println!("has_child: {} pairs", prog.has_child.len());
    println!("male childless: {} pairs", prog.male_childless.len());
    println!("\n=== Most Children (MAX) ===");
    for (person, cnt) in &prog.male_most_children {
        println!("  {} → {} children", person, cnt);

    }

    println!("\n=== Top 5 ancestor relationships ===");
    for (a, b) in prog.ancestor.iter().take(5) {
        println!("  {} is an ancestor of {}", a, b);
    }
}