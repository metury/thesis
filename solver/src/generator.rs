use std::fs;
use std::io::{self, Write};

fn comet(filepath: &str, path_len: u64, star_size: u64, k: u64, s: u64) -> io::Result<()> {
    let mut file = fs::File::create(filepath)?;
    writeln!(file, "s={} k={}", s, k)?;
    for i in 1..path_len {
        writeln!(file, "[{};{}]", i - 1, i)?;
    }
    writeln!(file, "[{};{}]", 0, path_len)?;
    for i in 0..star_size {
        writeln!(file, "[{};{}]", path_len, path_len + i + 1)?;
    }
    Ok(())
}

fn clique(filepath: &str, n: u64, k: u64, s: u64) -> io::Result<()> {
    let mut file = fs::File::create(filepath)?;
    writeln!(file, "s={} k={}", s, k)?;
    for i in 0..n {
        for j in i + 1..n {
            writeln!(file, "[{};{}]", i, j)?;
        }
    }
    Ok(())
}

fn star(filepath: &str, n: u64, k: u64, s: u64) -> io::Result<()> {
    let mut file = fs::File::create(filepath)?;
    writeln!(file, "s={} k={}", s, k)?;
    for i in 1..n + 1 {
        writeln!(file, "[{};{}]", 0, i)?;
    }
    Ok(())
}

fn tree(filepath: &str, width: u64, height: u64, k: u64, s: u64) -> io::Result<()> {
    let mut file = fs::File::create(filepath)?;
    writeln!(file, "s={} k={}", s, k)?;
    for i in 1..height {
        for j in u64::pow(width, (i-1) as u32)..u64::pow(width, i as u32) {
            writeln!(file, "[{};{}]", j, j / width)?;
        }
    }
    Ok(())
}

fn petersen(filepath: &str,k: u64, s: u64) -> io::Result<()> {
    let mut file = fs::File::create(filepath)?;
    writeln!(file, "s={} k={}", s, k)?;
    writeln!(file, "[0;1][1;2][2;3][3;4][4;0]")?;
    writeln!(file, "[5;7][5;8][6;8][6;9][7;9]")?;
    writeln!(file, "[5;0][6;1][7;2][8;3][9;4]")?;
    Ok(())
}

// Create a predefined graphs.
pub fn generate() {
    let _ = comet("graphs/comet.in", 10u64, 10u64, 12u64, 0u64);
    println!("comet");
    let _ = comet("graphs/comet-alt.in", 10u64, 10u64, 10u64, 12u64);
    println!("comet-alt");
    let _ = clique("graphs/clique.in", 8u64, 3u64, 0u64);
    println!("clique");
    let _ = star("graphs/star.in", 20u64, 4u64, 1u64);
    println!("star");
    let _ = comet("graphs/path.in", 10u64, 10u64, 6u64, 0u64);
    println!("path");
    let _ = tree("graphs/tree.in", 3u64, 6u64, 15u64, 0u64);
    println!("tree");
    let _ = petersen("graphs/petersen.in", 6u64, 0u64);
    println!("petersen");
}
