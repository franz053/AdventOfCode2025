use std::fs;

struct Union {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Union {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find_top(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_top(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find_top(a);
        let mut rb = self.find_top(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        return true;
    }
}

struct Point {
    x: isize,
    y: isize,
    z: isize,
}

fn shortest_line(box1: &Point, box2: &Point) -> usize {
    ((box1.x - box2.x).pow(2) + (box1.y - box2.y).pow(2) + (box1.z - box2.z).pow(2)).isqrt()
        as usize
}

fn main() {
    let file_path = "inputs/input8.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    //Create a Vector of points
    let points: Vec<Point> = contents
        .lines()
        .map(|line| {
            let mut iter = line.split(',').map(|n| n.parse::<isize>().unwrap());
            Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
                z: iter.next().unwrap(),
            }
        })
        .collect();

    //Create an ordered Map of all connections
    let mut connections: Vec<(usize, (usize, usize))> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = shortest_line(&points[i], &points[j]);
            connections.push((dist, (i, j)));
        }
    }
    connections.sort_by(|a, b| a.0.cmp(&b.0));

    let mut circuits = Union::new(points.len());

    let mut count = points.len();
    for (_, (i, j)) in connections {
        if circuits.union(i, j) {
            count -= 1;
            if count == 1 {
                println!("{}", points[i].x * points[j].x);
                break;
            }
        }
    }
}
