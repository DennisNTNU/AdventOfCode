use std::fs;
use std::str;

fn _range_contains_check(r1_0: u8, r1_1: u8, r2_0: u8, r2_1: u8) -> bool
{
    if r1_0 > r1_1
    {
        println!("Range error: {r1_0} is greater than {r1_1}");
        return false;
    }
    if r2_0 > r2_1
    {
        println!("Range error: {r2_0} is greater than {r2_1}");
        return false;
    }

    if r1_0 <= r2_0
    {
        if r1_1 >= r2_1
        {
            return true;
        }
    }

    if r2_0 <= r1_0
    {
        if r2_1 >= r1_1
        {
            return true;
        }
    }
    false
}

fn _range_overlap_check(r1_0: u8, r1_1: u8, r2_0: u8, r2_1: u8) -> bool
{
    if r1_0 > r1_1
    {
        println!("Range error: {r1_0} is greater than {r1_1}");
        return false;
    }
    if r2_0 > r2_1
    {
        println!("Range error: {r2_0} is greater than {r2_1}");
        return false;
    }

    if r1_0 <= r2_1
    {
        if r1_1 >= r2_0
        {
            return true;
        }
    }

    if r2_0 <= r1_1
    {
        if r2_1 >= r1_0
        {
            return true;
        }
    }
    false
}

fn main()
{
    //let file_path = String::from("../test-input.txt");
    let file_path = String::from("../input.txt");
    let file_content = fs::read_to_string(file_path)
        .expect("Could not read the file {file_path}\n");
    let file_content_bytes = file_content.as_bytes();
    let char_count = file_content.chars().count();
    println!("File length in chars: {}", char_count);

    let val1 = _range_contains_check(3,5,4,4);
    let val2 = _range_contains_check(3,5,4,5);
    let val3 = _range_contains_check(3,5,4,6);
    let val4 = _range_contains_check(3,5,7,6);
    let val5 = _range_contains_check(0,5,5,6);
    let val6 = _range_contains_check(3,5,3,5);

    println!("true, true, false, false, false, true");
    println!("{val1}, {val2}, {val3}, {val4}, {val5}, {val6}\n---------");

    let mut line_count: u32 = 0;
    let mut number_buffer = [b' '; 3];
    let mut number_buffer_index = 0;

    let mut r1_0 = 0;
    let mut r1_1 = 0;
    let mut r2_0 = 0;
    let mut r2_1;

    let mut containment_count: u32 = 0;
    for n in 0..char_count
    {
        if file_content_bytes[n] == b'-'
        {
            let s = match str::from_utf8(&number_buffer)
            {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };


            if r1_0 == 0
            {
                //r1_0 = s.trim().parse::<u8>().expect("String not an integer"); //str::from_str::<u8>(s);
                r1_0 = s.trim().parse().unwrap(); //str::from_str::<u8>(s);
            }
            else
            {
                //r2_0 = s.trim().parse::<u8>().expect("String not an integer"); //str::from_str::<u8>(s);                
                r2_0 = (&s).trim().parse().unwrap(); //str::from_str::<u8>(s);                
            }


            number_buffer_index = 0;
            number_buffer[0] = b' ';
            number_buffer[1] = b' ';
            number_buffer[2] = b' ';
        }
        else if file_content_bytes[n] == b','
        {
            let s = match str::from_utf8(&number_buffer)
            {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };


            //r1_1 = s.trim().parse::<u8>().expect("String not an integer"); //str::from_str::<u8>(s);
            //r1_1 = s.trim().parse().unwrap(); //str::from_str::<u8>(s);
            r1_1 = match s.trim().parse::<u8>() {
                Ok(i) => i,
                Err(_e) => panic!("Cant parse string to u8? {}", _e),
            };



            number_buffer_index = 0;
            number_buffer[0] = b' ';
            number_buffer[1] = b' ';
            number_buffer[2] = b' ';
        }
        else if file_content_bytes[n] == b'\n'
        {
            let s = match str::from_utf8(&number_buffer)
            {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            //r2_1 = s.trim().parse::<u8>().expect("String not an integer"); //str::from_str::<u8>(s);
            r2_1 = s.trim().parse().unwrap(); //str::from_str::<u8>(s);



            //let containment = _range_contains_check(r1_0, r1_1, r2_0, r2_1);
            let containment = _range_overlap_check(r1_0, r1_1, r2_0, r2_1);
            containment_count += containment as u32;

            //print!("{r1_0},{r1_1},{r2_0},{r2_1}");
            //println!("  {containment}");



            number_buffer_index = 0;
            number_buffer[0] = b' ';
            number_buffer[1] = b' ';
            number_buffer[2] = b' ';
            line_count += 1;
            r1_0 = 0;
            r1_1 = 0;
            r2_0 = 0;
            //r2_1 = 0;
        }
        else
        {
            number_buffer[number_buffer_index] = file_content_bytes[n];
            number_buffer_index += 1;
        }
    }
    print!("Lines: {}\n", line_count);
    print!("range containment count: {}\n", containment_count);
}

