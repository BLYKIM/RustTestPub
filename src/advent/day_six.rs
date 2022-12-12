use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
pub fn day_six() -> Result<()> {
    let mut lines = read_file("inputs/day_six_input.txt")?.flatten().peekable();

    if let Some(line) = lines.next() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut marker = 0_usize;
        let len = chars.len();
        'c: for m in 14..len {
            let msg = &chars[m - 14..m].to_vec();
            for i in 0..14 {
                let idx = &msg[i];
                for j in &msg[i + 1..14] {
                    if idx == j {
                        continue 'c;
                    }
                }
            }
            marker = m;
            break;
        }

        println!("marker: {}. {:?}", marker, &chars[marker - 14..marker]);
    }
    Ok(())
}
