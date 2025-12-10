use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let red_tiles: Vec<[i128; 2]> = binding
        .lines()
        .map(|line| {
            let split: Vec<i128> = line.split(',').map(|x| x.parse().unwrap()).collect();
            [split[0], split[1]]
        })
        .collect();

    let red_tiles_set: HashSet<[i128; 2]> = red_tiles.iter().map(|x| x.to_owned()).collect();

    let max_x = *red_tiles.iter().map(|[x, _]| x).max().unwrap();
    let max_y = *red_tiles.iter().map(|[_, y]| y).max().unwrap();
    println!("Grid: {max_x}, {max_y}");
    let mut shape_tiles_set: Vec<[i128; 2]> = Vec::new();

    // draw outline
    for (i, red_tile) in red_tiles
        .iter()
        .chain([red_tiles[0]].iter())
        .enumerate()
        .skip(1)
    {
        let last_red_tile = red_tiles[i - 1];
        // println!("Outline: {i}");
        // println!("Current tile: {:?}", red_tile);
        // println!("Last tile: {:?}", last_red_tile);
        if last_red_tile[0] != red_tile[0] {
            let least = [last_red_tile[0], red_tile[0]]
                .iter()
                .min()
                .unwrap()
                .to_owned();
            let most = [last_red_tile[0], red_tile[0]]
                .iter()
                .max()
                .unwrap()
                .to_owned();

            for x in least..most {
                shape_tiles_set.push([x, red_tile[1]]);
            }
        } else {
            let least = [last_red_tile[1], red_tile[1]]
                .iter()
                .min()
                .unwrap()
                .to_owned();
            let most = [last_red_tile[1], red_tile[1]]
                .iter()
                .max()
                .unwrap()
                .to_owned();

            for y in least..most {
                shape_tiles_set.push([red_tile[0], y]);
            }
        }
    }
    shape_tiles_set.append(&mut red_tiles.clone());

    shape_tiles_set = shape_tiles_set
        .iter()
        .map(|x| x.to_owned())
        .collect::<HashSet<[i128; 2]>>()
        .iter()
        .map(|x| x.to_owned())
        .collect();

    println!("Outline tiles: {}", shape_tiles_set.len());

    shape_tiles_set.sort_by_key(|&[x, y]| y * max_x + x);

    let mut tiles_by_y: HashMap<i128, Vec<i128>> = HashMap::new();

    for tile in shape_tiles_set.clone() {
        if !tiles_by_y.contains_key(&tile[1]) {
            tiles_by_y.insert(tile[1], Vec::new());
        }
        tiles_by_y.get_mut(&tile[1]).unwrap().push(tile[0]);
    }

    let y_list: Vec<i128> = tiles_by_y.keys().map(|x| x.to_owned()).sorted().collect();
    let mut last_row = Vec::new();

    // print_grid(red_tiles_set.clone(), shape_tiles_set.clone(), max_x, max_y);

    for &y in y_list.iter() {
        let row = tiles_by_y.get_mut(&y).unwrap();
        let mut to_remove: Vec<usize> = vec![];
        // println!("{y} Before {:?}", row);

        let mut index = 0;
        let mut inside = false;
        while index < row.len() - 1 {
            // println!("{index}");
            if index + 1 < row.len() && row[index + 1] != row[index] + 1 {
                index += 1;
                inside = !inside;
                continue;
            }
            // if it's a line, leave endpoints
            let before_point_above: bool = { shape_tiles_set.contains(&[row[index], y - 1]) };
            let first_index = index;
            index += 1;
            while index + 1 < row.len() && row[index + 1] == row[index] + 1 {
                to_remove.push(index);
                index += 1;
            }

            let after_point_above: bool = { shape_tiles_set.contains(&[row[index], y - 1]) };

            to_remove.push(index);

            if before_point_above ^ after_point_above {
                if inside {
                    to_remove.pop();
                    to_remove.push(first_index);
                }
                inside = !inside;
            }

            if !(before_point_above ^ after_point_above) {
                if inside {
                    to_remove.push(first_index);
                } else {
                    to_remove.pop();
                }
            }

            index += 1;
        }

        for i in to_remove.iter().sorted().rev() {
            row.remove(*i);
        }
        for i in (0..row.len()).skip(1).step_by(2) {
            row[i] += 1;
        }
        // println!("{y} After {:?}", row);
        last_row = row.clone();
    }

    println!("{:?}", tiles_by_y);

    let mut maximum_size = 0;
    let length = red_tiles.len();
    for (i, first) in tqdm::tqdm(red_tiles.iter().enumerate()) {
        for second in red_tiles[i + 1..].iter() {
            let try_size = area(first, second);
            if try_size > maximum_size {
                // println!("{:?} to {:?} area {}", first, second, try_size);
                let left_x = [first[0], second[0]].iter().min().unwrap().to_owned();
                let right_x = [first[0], second[0]].iter().max().unwrap().to_owned();
                let top_y = [first[1], second[1]].iter().min().unwrap().to_owned();
                let bottom_y = [first[1], second[1]].iter().max().unwrap().to_owned();

                let mut broken = false;

                for y in top_y..bottom_y + 1 {
                    let row_tiles = tiles_by_y.get(&y).unwrap().to_owned();
                    // println!("{y}: {:?}", row_tiles);
                    if left_x < row_tiles[0] || right_x > row_tiles[row_tiles.len() - 1] {
                        broken = true;
                        break;
                    }
                    let try_left_boundary_index = row_tiles.iter().find_position(|x| x > &&left_x);
                    if try_left_boundary_index.is_none() {
                        broken = true;
                        break;
                    }
                    let left_boundary_index = try_left_boundary_index.unwrap().0 - 1;
                    // println!("{left_boundary_index}");
                    if left_boundary_index % 2 != 0 {
                        broken = true;
                        break;
                    }
                    if right_x != left_x {
                        let right_boundary_index = left_boundary_index + 1;
                        if right_boundary_index >= row_tiles.len()
                            || right_x > row_tiles[right_boundary_index]
                        {
                            broken = true;
                            break;
                        }
                    }
                }
                if !broken {
                    println!("{:?} to {:?} area {}", first, second, try_size);
                    println!("New max {try_size}");
                    maximum_size = try_size;
                } else {
                    // println!("Failed");
                }
            }
        }
    }

    println!("{maximum_size}");

    // draw inside
    /*     println!("drawing inside");
        for y in 1..max_y {
            let mut inside = false;
            for x in 1..max_x {
                println!("Inside: [{x}, {y}]");
                if inside {
                    println!("Inserted [{x}, {y}]");
                    shape_tiles_set.push([x, y]);
                }
                if !(is_red_or_green(x, y, &red_tiles_set, &shape_tiles_set)
                    ^ is_red_or_green(x - 1, y, &red_tiles_set, &shape_tiles_set))
                {
                    continue;
                }
                println!("Difference at [{x}, {y}]");
                inside = !inside;
            }
        }
    */
    // print_grid(red_tiles_set, shape_tiles_set, max_x, max_y);
}

fn is_red_or_green(
    x: i128,
    y: i128,
    red_tiles: &HashSet<[i128; 2]>,
    green_tiles: &Vec<[i128; 2]>,
) -> bool {
    red_tiles.contains(&[x, y]) || green_tiles.contains(&[x, y])
}

fn print_grid(
    red_tiles: HashSet<[i128; 2]>,
    green_tiles: Vec<[i128; 2]>,
    max_x: i128,
    max_y: i128,
) {
    for y in 0..max_y + 2 {
        for x in 0..max_x + 2 {
            if red_tiles.contains(&[x, y]) {
                print!("#");
            } else if green_tiles.contains(&[x, y]) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn area(first: &[i128; 2], second: &[i128; 2]) -> i128 {
    ((first[0] - second[0]).abs() + 1) * ((first[1] - second[1]).abs() + 1)
}
