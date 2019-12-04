#[derive(Debug, PartialEq)]
struct Point(i16, i16);

struct Line(Point, Point);

type Graph = Vec<Point>;

fn determine_point(mut raw: String, curr: &Point) -> Point {
    let dir: char = raw.remove(0);
    let val: i16 = raw.parse().unwrap();

    match dir {
        'R' => Point(curr.0+val,curr.1),
        'L' => Point(curr.0-val,curr.1),
        'U' => Point(curr.0,curr.1+val),
        'D' => Point(curr.0,curr.1-val),
        _ => Point(0,0)
    }
    
}

fn parse_graph(a: &str) -> Graph {
    let mut points: Vec<Point> = Vec::new();
    let raw_points: Vec<_> = a.split(",").collect();

    points.push(
        Point(0,0)
    );
    for (_, raw_point) in raw_points.iter().enumerate() {
        let raw_string = String::from(*raw_point);
        let curr = determine_point(raw_string, points.last().unwrap());
        points.push(curr)
    }

    points
}

fn intersect(Line(Point(ca, cb), Point(da,db)): &Line, Line(Point(aa, ab), Point(ba,bb)): &Line) -> Option<Point> {
    // NO: None
    // (5,5)->(1,5)
    // (4,4)->(1,4)
    //
    // YES: (3,2)
    // (3,3)->(1,3)
    // (1,2)->(4,2)
    //
    // 5         x
    // 4         |
    // 3     y   |
    // 2 z - + z |
    // 1     y   x
    // 0 1 2 3 4 5
    //
    // (aa,ab)->(ba,bb)
    // (ca,cb)->(da,db)
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::{Line, Point, intersect};

    #[test]
    fn finds_intersect() {
        let a = Line(Point(3,3), Point(1,3));
        let b = Line(Point(1,2), Point(4,2));
        let p = intersect(&a, &b);
        let q = intersect(&b, &a);

        assert_eq!(p, Some(Point(3,2)));
        assert_eq!(q, Some(Point(3,2)));
    }

    #[test]
    fn no_intersect() {
        let a = Line(Point(3,3), Point(1,3));
        let b = Line(Point(4,4), Point(1,4));
        let p = intersect(&a, &b);
        let q = intersect(&b, &a);

        assert_eq!(p, None);
        assert_eq!(q, None);
    }
}

pub fn solution(a: &str, b: &str) -> String {
    let ga = parse_graph(a);
    let gb = parse_graph(b);
    println!("{:?} {:?}", ga, gb);
    format!("{} {}", a, b)
}
