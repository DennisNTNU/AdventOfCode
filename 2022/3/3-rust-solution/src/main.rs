
use std::fs;

fn byte_to_index(byte: u8) -> usize
{
    let return_value: usize;
    if (byte) < 92
    {
        return_value = ((byte as u32) + 26 - 65) as usize;
    }
    else
    {
        return_value = ((byte as u32) - 97) as usize;
    }
    return_value
}

fn _byte_to_priority(byte: u8) -> u32
{
    (byte_to_index(byte) + 1) as u32
}


fn main()
{
    //let file_path = String::from("../test-input.txt");
    let file_path = String::from("../input.txt");
    let file_content = fs::read_to_string(file_path).expect("Could not read the file {file_path}\n");
    let file_content_bytes = file_content.as_bytes();
    let char_count = file_content.chars().count();
    print!("File length in chars: {}\n", char_count);

    let mut elf_in_group_index: u32 = 0;
    let mut score: u32 = 0;
    let mut char_array0: [u8; 52] = [0; 52];
    let mut char_array1: [u8; 52] = [0; 52];
    let mut char_array2: [u8; 52] = [0; 52];
    for n in 0..char_count
    {
        let current_char: u8 =  file_content_bytes[n];

        if current_char == b'\n' // this way of doing things requires that the last line also has a newline at the end
        {
            elf_in_group_index += 1;
            if elf_in_group_index == 3
            {
                elf_in_group_index = 0;

                for m in 0..52
                {
                    if char_array0[m] != 0 && char_array1[m] != 0 && char_array2[m] != 0
                    {
                        score += (m+1) as u32;
                        let p = m+1;
                        println!("Score: {p} total: {score}");
                    }
                    char_array0[m] = 0;
                    char_array1[m] = 0;
                    char_array2[m] = 0;

                }
            }
            continue;
        }

        let char_index = byte_to_index(current_char);

        if elf_in_group_index == 0
        {
            char_array0[char_index] += 1;
        }
        if elf_in_group_index == 1
        {
            char_array1[char_index] += 1;
        }
        if elf_in_group_index == 2
        {
            char_array2[char_index] += 1;
        }
    }
}

fn _part1()
{
    let file_path = String::from("../input.txt");
    let file_content = fs::read_to_string(file_path).expect("Could not read the file {file_path}\n");
    let file_content_bytes = file_content.as_bytes();
    let char_count = file_content.chars().count();
    print!("File length in chars: {}\n", char_count);

    // count lines
    let mut newline_count: u32 = 0;
    for n in 0..char_count
    {
        if file_content_bytes[n] == b'\n'
        {
            newline_count += 1
        }
    }
    print!("Lines: {}\n", newline_count);

    // hardcoding 300 here, because I dont yet know how dynamic sizes work in Rust
    let mut len_arr: [u8; 300] = [0; 300];

    let mut index: usize = 0;
    for n in 0..char_count
    {
        if file_content_bytes[n] == b'\n'
        {
            index += 1;
        }
        else
        {
            len_arr[index] += 1;
        }
    }

    let mut input_index: usize = 0;
    let mut score: u32 = 0;
    for n in 0..newline_count
    {
        let line_index: usize = n as usize;
        let len = len_arr[line_index] as usize;
        print!("{len}\n");
        let len_2 = len / 2;
        let mut common_char: char = 0 as char;
        let mut common_char_found: bool = false;
        for m in len_2..len
        {
            let char_to_check: u8 = file_content_bytes[input_index + m];
            for o in 0..len_2
            {
                let char_to_check2: u8 = file_content_bytes[input_index + o];
                if char_to_check2 == char_to_check
                {
                    common_char = char_to_check2 as char;
                    print!("common char: {common_char} | {char_to_check2}");
                    common_char_found = true;
                    break;
                }
            }
            if common_char_found
            {
                if (common_char as u8) < 92
                {
                    let line_score = (common_char as u32) - 64 + 26;
                    print!(" {line_score}\n");
                    score += line_score;
                }
                else
                {
                    let line_score = (common_char as u32) - 96;
                    print!(" {line_score}\n");
                    score += line_score;
                }
                break;
            }
        }

        input_index += len+1;
    }
    print!("Final Score: {score}\n");

}