mod utils;


use utils::get_file;

fn main() {
    let input_str = get_file("day1.input");

    let mut elf_sums = Vec::new();

    for elf_inv in input_str.split("\n\n") {
        let sum = get_sum(elf_inv);
        elf_sums.push(sum);
    }

    elf_sums.sort();
    elf_sums.reverse();

    println!("part 1 => {}", elf_sums[0]);
    println!("part 2 => {}", elf_sums[0] + elf_sums[1] + elf_sums[2]);

}

fn get_sum(elf_inv: &str) -> i32 {
    let mut sum = 0;
    for item in elf_inv.trim().split("\n") {
        sum += item.trim().parse::<i32>().unwrap();
    }
    sum
}
