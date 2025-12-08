use std::collections::{HashMap, HashSet};

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input: HashSet<[i32; 3]> = binding
        .lines()
        .map(|line| {
            let split: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            [split[0], split[1], split[2]]
        })
        .collect();

    println!("{:?}", input);

    let mut circuits: HashSet<Vec<[i32; 3]>> = HashSet::new();
    let mut connections: HashSet<UnorderedPair<[i32; 3]>> = HashSet::new();

    let distances: Vec<UnorderedPair<[i32; 3]>> = get_distances(&input);

    for count in 0..1000 {
        println!("{count}");
        let try_closest = find_closest(&input, &connections, &distances);
        println!("{:?}", try_closest);
        if !same_circuit(&circuits, try_closest.0, try_closest.1) {
            add_circuit(&mut circuits, try_closest.0, try_closest.1);
        }
        connections.insert(UnorderedPair {
            a: try_closest.0,
            b: try_closest.1,
        });
    }
    let lengths: Vec<usize> = circuits.iter().map(|x| x.len()).collect();
    println!("{:?}", lengths);
    let lengths_hash: HashSet<usize> = lengths.iter().map(|x| *x).collect();
    println!("{:?}", lengths_hash);
    let product: usize = lengths_hash.iter().product();
    println!("{:?}", product);
    // techincally wrong answer
}

fn get_distances(input: &HashSet<[i32; 3]>) -> Vec<UnorderedPair<[i32; 3]>> {
    let input_vec: Vec<[i32; 3]> = input.iter().map(|x| *x).collect();
    let mut out = HashMap::new();
    for (i, first) in input_vec.iter().enumerate() {
        for second in &input_vec[i + 1..] {
            out.insert(UnorderedPair { a: *first, b: *second }, box_distance(*first, *second));
        }
    }

    let mut output_vec: Vec<UnorderedPair<[i32; 3]>> = out.keys().map(|x| x.clone()).collect();
    output_vec.sort_by(|a, b| out.get(a).unwrap().partial_cmp(out.get(b).unwrap()).unwrap());

    output_vec
}

fn add_circuit(circuits: &mut HashSet<Vec<[i32; 3]>>, first: [i32; 3], second: [i32; 3]) {
    let circuits_vec: Vec<&Vec<[i32; 3]>> = circuits.iter().map(|x| x).collect();

    if let Some(first_index) = find_circuit(&circuits_vec, first)
        && let Some(second_index) = find_circuit(&circuits_vec, second)
    {
        if first_index != second_index {
            let mut set_with_first: Vec<[i32; 3]> = get_set(circuits, first).clone();
            let mut set_with_second = get_set(circuits, second).clone();
            circuits.remove(&set_with_first);
            circuits.remove(&set_with_second);
            set_with_first.append(&mut set_with_second);
            circuits.insert(set_with_first);
            return;
        }
    }

    for set in circuits.clone().iter() {
        let mut set_clone = set.clone();
        if set.contains(&first) {
            set_clone.push(second);
            circuits.remove(set);
            circuits.insert(set_clone);
            return;
        } else if set.contains(&second) {
            set_clone.push(first);
            circuits.remove(set);
            circuits.insert(set_clone);
            return;
        }
    }
    circuits.insert(vec![first, second]);
}

fn get_set(circuits: &HashSet<Vec<[i32; 3]>>, junction_box: [i32; 3]) -> &Vec<[i32; 3]> {
    for set in circuits.iter() {
        if set.contains(&junction_box) {
            return set;
        }
    }
    circuits.iter().nth(0).unwrap()
}

fn find_circuit(circuits_vec: &Vec<&Vec<[i32; 3]>>, junction_box: [i32; 3]) -> Option<usize> {
    return circuits_vec.iter().enumerate().find_map(|(i, x)| {
        if x.contains(&junction_box) {
            Some(i)
        } else {
            None
        }
    });
}

fn same_circuit(circuits: &HashSet<Vec<[i32; 3]>>, first: [i32; 3], second: [i32; 3]) -> bool {
    let circuits_vec: Vec<_> = circuits.iter().map(|x| x).collect();
    if let Some(first_index) = find_circuit(&circuits_vec, first) {
        if let Some(second_index) = find_circuit(&circuits_vec, second) {
            return first_index == second_index;
        }
    }
    false
}

fn find_closest(
    boxes: &HashSet<[i32; 3]>,
    connections: &HashSet<UnorderedPair<[i32; 3]>>,
    distances: &Vec<UnorderedPair<[i32; 3]>>,
) -> ([i32; 3], [i32; 3]) {
    let output: Option<([i32; 3], [i32; 3])> = {
        for distance in distances {
            if !connections.contains(&distance) {
                return ((distance.a, distance.b));
            }
        }
        None
    };
    return output.unwrap()
}

fn box_distance(first: [i32; 3], second: [i32; 3]) -> f64 {
    let x: f64 = (first[0] - second[0]).into();
    let y: f64 = (first[1] - second[1]).into();
    let z: f64 = (first[2] - second[2]).into();

    let answer = (x * x + y * y + z * z).sqrt();
    // println!("{:?} - {:?} = {answer}", first, second);
    answer
}

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
struct UnorderedPair<T> {
    a: T,
    b: T,
}

impl<T: Ord> UnorderedPair<T> {
    pub fn new(a: T, b: T) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn ref_a(&self) -> &T {
        &self.a
    }

    pub fn ref_b(&self) -> &T {
        &self.b
    }
}

impl<T: Ord> From<(T, T)> for UnorderedPair<T> {
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<T: PartialEq<T>> PartialEq<UnorderedPair<T>> for UnorderedPair<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.a == rhs.a && self.b == rhs.b
    }
}

impl<T: Eq> Eq for UnorderedPair<T> {}

impl<T: PartialOrd> PartialOrd for UnorderedPair<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.a.partial_cmp(&rhs.a) {
            Some(Ordering::Equal) => self.b.partial_cmp(&rhs.b),
            v => v,
        }
    }
}

impl<T: Ord> Ord for UnorderedPair<T> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self.a.cmp(&rhs.a) {
            Ordering::Equal => self.b.cmp(&rhs.b),
            v => v,
        }
    }
}

impl<T: Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.a.hash(hasher);
        self.b.hash(hasher);
    }
}
