use std::fs;

fn print_solution(stacks: & Vec<Vec<char>>, stack_count: usize)
{
    for n in 0..stack_count
    {

        let top_index = find_top_index(&stacks[n]);
        if top_index == 0
        {
            print!("{}", stacks[n][top_index]);
        }
        else
        {
            print!("{}", stacks[n][top_index-1]);
        }
    }
    println!("");
}

fn print_stacks(stacks: & Vec<Vec<char>>, stack_count: usize)
{
    for n in 0..stack_count
    {
        print!("Stack {}: ", n+1);
        let mut height: usize = 0;
        //while true
        loop
        {
            if stacks[n][height] == ' '
            {
                break;
            }
            print!("{} ", stacks[n][height]);
            height += 1;
        }
        println!("");
    }
}

fn find_top_index(stack: &Vec<char>) -> usize
{
    let mut i = 0;
    let mut top_index = 0;
    let mut top_found: bool = false;
    while !top_found
    {
        if stack[i] == ' '
        {
            top_found = true;
            top_index = i;
        }
        i += 1;
    }
    return top_index;
}

fn move_crates(count: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>)
{
    let mut top_index_from = find_top_index(&stacks[from-1]);
    let mut top_index_to   = find_top_index(&stacks[to-1]);
    println!("Moving {} from {} to {}", count, from, to);
    //println!("top indices: from: {} to: {}", top_index_from, top_index_to);

    for _c in 0..count
    {
        stacks[to-1][top_index_to] = stacks[from-1][top_index_from-1];
        stacks[from-1][top_index_from-1] = ' ';

        top_index_from -= 1;
        top_index_to += 1;
    }
}

fn move_crates_part2(count: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>)
{
    let mut top_index_from = find_top_index(&stacks[from-1]);
    let top_index_to   = find_top_index(&stacks[to-1]);
    println!("Moving {} from {} to {}", count, from, to);
    //println!("top indices: from: {} to: {}", top_index_from, top_index_to);

    for _c in 0..count
    {
        stacks[to-1][top_index_to + count - _c - 1] = stacks[from-1][top_index_from-1];
        stacks[from-1][top_index_from-1] = ' ';

        top_index_from -= 1;
        //top_index_to += 1;
    }
}

fn main()
{

    let part1: bool = false;


    //let file_path = String::from("../test-input.txt");
    let file_path = String::from("../input.txt");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read the file {file_path}\n");
    let char_count = file_content.chars().count();
    println!("File length in chars: {}", char_count);
    
    let file_content_bytes = file_content.as_bytes();

    let mut line_count: usize = 0;
    let mut line_empty: bool = true;
    let mut empty_line_index: usize = 0;
    let mut first_line_length: usize = 0;
    for n in 0..char_count
    {
        if line_count == 0
        {
            first_line_length += 1;
        }
        if file_content_bytes[n] == b'\n'
        {
            if line_empty
            {
                empty_line_index = line_count;
            }
            line_count += 1;
            line_empty = true;
        }
        else
        {
            line_empty = false;
        }
    }

    let stack_count = (first_line_length/4) as usize;

    print!("Lines: {}\n", line_count);
    println!("first_line_length {first_line_length}");
    println!("stacks: {}", stack_count);
    println!("empty_line_index: {empty_line_index}");

    println!("----");

    // get first stack
    // stack size 2 more than total theoretical capacity because why not to be safe
    let mut stacks = vec![ vec![' '; ((empty_line_index-1)*stack_count + 2) as usize]; stack_count];

    for m in 0..stack_count
    {
        for n in 0..(empty_line_index-1)
        {
            let file_content_index: usize = (1 + 4*m + first_line_length*(empty_line_index-n-2)) as usize;
            stacks[m as usize][n as usize] = file_content_bytes[file_content_index] as char;
        }
    }

    println!("Initial state:");
    print_stacks(&stacks, stack_count);
    println!("----");


    //move_crates(2, 0, 2, &mut stacks);
    //print_stacks(&stacks, stack_count);
    //println!("----");




    let mut line_start_index: usize = first_line_length*empty_line_index + 1;
    //let mut i: usize = 0;
    loop
    {

        // +5 to skip "move "
        // parsing crate move count
        let mut i: usize = line_start_index + 5; 

        let mut number_buffer = [b' '; 4];
        let mut j: usize = 0;
        loop
        {
            if file_content_bytes[i] == b' '
            {
                break;
            }
            number_buffer[j] = file_content_bytes[i];
            j += 1;
            i += 1;
        }

        let s = std::str::from_utf8(&number_buffer).expect("Invalid UTF-8 sequence");
        let count: usize = s.trim().parse().unwrap();


        // skipping " from "
        // parsing source stack index
        i += 6; 

        number_buffer = [b' '; 4];
        j = 0;
        loop
        {
            if file_content_bytes[i] == b' '
            {
                break;
            }
            number_buffer[j] = file_content_bytes[i];
            j += 1;
            i += 1;
        }

        let s = std::str::from_utf8(&number_buffer).expect("Invalid UTF-8 sequence");
        let from: usize = s.trim().parse().unwrap();



        // skipping " to "
        // parsing destination stack index
        i += 4; 

        number_buffer = [b' '; 4];
        j = 0;
        loop
        {
            if file_content_bytes[i] == b'\n'
            {
                break;
            }
            number_buffer[j] = file_content_bytes[i];
            j += 1;
            i += 1;
        }

        let s = std::str::from_utf8(&number_buffer).expect("Invalid UTF-8 sequence");
        let to: usize = s.trim().parse().unwrap();

        line_start_index = i+1;

        println!(" -----");
        if part1
        {
            move_crates(count, from, to, &mut stacks);
        }
        else
        {
            move_crates_part2(count, from, to, &mut stacks);
        }
        print_stacks(&stacks, stack_count);

        if line_start_index >= char_count
        //if line_start_index >= 400
        {
            break;
        }
    }
    print_solution(&stacks, stack_count);
}
