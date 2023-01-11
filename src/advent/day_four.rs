use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_four() -> Result<()> {
    let lines = read_file("inputs/day_four_input.txt")?.flatten().peekable();
    // let mut full_pairs = 0u32;
    let mut range_pairs = 0u32;

    for line in lines {
        let (first, second) = line.split_once(',').unwrap();
        let (f_s, f_e) = first.split_once('-').unwrap();
        let (s_s, s_e) = second.split_once('-').unwrap();
        let f_s = f_s.parse::<u32>().unwrap();
        let f_e = f_e.parse::<u32>().unwrap();
        let s_s = s_s.parse::<u32>().unwrap();
        let s_e = s_e.parse::<u32>().unwrap();

        // if (f_s >= s_s && f_e <= s_e) || (s_s >= f_s && s_e <= f_e) {
        //     full_pairs += 1;
        // }
        if (f_s >= s_s && f_s <= s_e)
            || (f_e >= s_s && f_e <= s_e)
            || (s_s >= f_s && s_s <= f_e)
            || (s_e >= f_s && s_e <= f_e)
        {
            range_pairs += 1;
        }
    }
    println!("{range_pairs}");

    Ok(())
}
