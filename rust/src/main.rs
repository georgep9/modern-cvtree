// const CODE: [i32; 26] = [0, 2, 1, 2, 3, 4, 5, 6, 7, -1, 8, 9, 10, 11, -1, 12, 13, 14, 15, 16, 1, 17, 18, 5, 19, 3];
// fn encode(ch: char) -> i32 { CODE[(ch as u32 - 'A' as u32) as usize] }
const LEN: u32 = 6;
const AA_NUMBER: u32 = 20;
// const EPSILON: f32 = 1e-010;

fn init(m: &mut u32, m1: &mut u32, m2: &mut u32){
    *m2 = 1;
    for _n in 0..LEN-2 {
        *m2 = *m2 * AA_NUMBER; // M2 = AA_NUMBER ^ (LEN-2);
    }
    *m1 = *m2 * AA_NUMBER; // M1 = AA_NUMBER ^ (LEN-1);
    *m = *m1 * AA_NUMBER; // M  = AA_NUMBER ^ (LEN);
}
        
fn read_input_file() -> &'static str {
    let input_file_contents = include_str!("list.txt");
    return input_file_contents;
}

fn load_all_bacteria_parallel() {
    println!("load_all_bacteria_parallel");
}

fn compare_all_bacteria_parallel() {
    println!("compare_all_bacteria_parallel");
}

fn main() {
    let mut _number_bacteria: u32;
    let mut _bacteria_names: char;
    let mut _m: u32 = 0;
    let mut _m1: u32 = 0;
    let mut _m2: u32 = 0;

    init(&mut _m, &mut _m1, &mut _m2);
    println!("init output: {} {} {}", &mut _m, &mut _m1, &mut _m2);

    let input_file_list = read_input_file();
    println!("input file list data: {input_file_list}");

    load_all_bacteria_parallel();
    compare_all_bacteria_parallel();
}

