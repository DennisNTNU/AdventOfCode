//use std::env;
use std::fs;

fn part1(printing: bool)
{
    let file_path = String::from("../input.txt");

    let file_content = fs::read_to_string(file_path).expect("Could not read the file");

    let char_count = file_content.chars().count();
    print!("File length in chars: {}\n", char_count);

    let mut score: u32 = 0;

    for n in 0..(char_count/4)
    {
        let char_byte1 = file_content.as_bytes()[4*n];
        let char_byte2 = file_content.as_bytes()[4*n+2];
        let hand_shape1 = hand_shape_to_integer(char_byte1);
        let hand_shape2 = hand_shape_to_integer(char_byte2);
        score = score + hand_shape2 + 1;

        if printing
        {
            print!("{}: {} -> {} | {} -> {} | {} &   ",
                   n, char_byte1, hand_shape1, char_byte2, hand_shape2, score);
        }
        if hand_shape1 == hand_shape2
        {
            score = score + 3;
        }
        if (hand_shape2 + 1) % 3 == hand_shape2
        {
            score = score + 0;
        }
        if (hand_shape1 + 1) % 3 == hand_shape2
        {
            score = score + 6;
        }
        if printing
        {
            print!("{}\n", score);
        }
    }

    println!("Final score part 1: {score}");

}

fn part2(printing: bool)
{
    let file_path = String::from("../input.txt");

    let file_content = fs::read_to_string(file_path).expect("Could not read the file");

    let char_count = file_content.chars().count();
    print!("File length in chars: {}\n", char_count);

    let mut score: u32 = 0;

    for n in 0..(char_count/4)
    {
        let char_byte1 = file_content.as_bytes()[4*n];
        let char_byte2 = file_content.as_bytes()[4*n+2];
        let hand_shape1 = hand_shape_to_integer(char_byte1);
        let hand_shape2 = hand_shape_to_integer(char_byte2);
        score = score + 3*hand_shape2; // outcome score
        if printing
        {
            print!("{}: {} -> {} | {} -> {} | {} & ",
                   n, char_byte1, hand_shape1, char_byte2, hand_shape2, score);
        }
        // 0 beats 2, 1 beats 0 and 2 beats 1
        if hand_shape2 == 0
        { // lose
            // 0 -> 2 -> score + 3
            // 1 -> 0 -> score + 1
            // 2 -> 1 -> score + 2
            if hand_shape1 == 0
            {
                score = score + 3;
            }
            else
            {
                score = score + hand_shape1;
            }

            if printing
            {
                print!("lose ");
            }
        }
        if hand_shape2 == 1
        { // draw
            score = score + hand_shape1 + 1;

            if printing
            {
                print!("draw ");
            }
        }
        if hand_shape2 == 2
        { // win
            // 0 -> 1 -> score + 2
            // 1 -> 2 -> score + 3
            // 2 -> 0 -> score + 1
            score = score + (hand_shape1 + 1) % 3 + 1;

            if printing
            {
                print!("win! ");
            }
        }

        if printing
        {
            print!("{}\n", score);
        }
    }

    println!("Final score part 2: {score}");

}

fn main()
{
    part1(false);
    part2(false);
}

fn hand_shape_to_integer(c: u8) -> u32
{
    let mut return_value = 5; 
    if c == 65 || c == 88
    {
        return_value = 0;
    }
    if c == 66 || c == 89
    {
        return_value = 1;
    }
    if c == 67 || c == 90
    {
        return_value = 2;
    }
    return_value
}
