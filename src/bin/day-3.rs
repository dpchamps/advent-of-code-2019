use adventofcode::read_input_file;

type DirectionCoord = (char, f32);

#[derive(PartialEq, Copy, Clone, Debug)]
struct Coord {
    x : f32,
    y : f32
}

impl Coord {
    fn new(x : f32, y : f32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self {x : 0.0, y : 0.0}
    }

    fn manhatten_dist(point_a : &Coord, point_b : &Coord) -> f32 {
        (point_a.x - point_b.x).abs() + (point_a.y - point_b.y).abs()
    }

    fn closest_to(&self, haystack : &Vec<Coord>) -> (Option<Coord>, f32) {
        let mut closest_point = None;
        let mut closest_distance = std::f32::INFINITY;

        for candidate in haystack {
            let next_dist = Coord::manhatten_dist(&self, candidate);
           if next_dist < closest_distance {
               closest_point = Some(candidate.clone());
               closest_distance = next_dist;
           }
        }

        (closest_point, closest_distance)
    }

    fn next(previous :&Self, (direction, length) : &DirectionCoord) -> Result<Coord, String> {
        let translate = match direction {
            'U' => Coord::new(0.0, *length),
            'D' => Coord::new(0.0, -(*length)),
            'R' => Coord::new(*length, 0.0),
            'L' => Coord::new(-(*length), 0.0),
            _ => return Err(format!("Invalid direction char: {}", direction))
        };

        Ok(Self{
            x : previous.x + translate.x,
            y : previous.y + translate.y
        })
    }

    fn sub(&self, other : &Coord) -> Self {
        Self {
            x : self.x - other.x,
            y : self.y - other.y
        }
    }

    fn add(&self, other : &Coord) -> Self {
        Self {
            x : self.x + other.x,
            y : self.y + other.y
        }
    }

    fn mult(&self, other : &Coord) -> Self{
        Self{
            x : self.x * other.x,
            y : self.y * other.y
        }
    }

    fn cross(&self, other : &Coord) -> f32 {
        self.x * other.y - self.y * other.x
    }

    fn max(&self) -> f32 {
        f32::max(
            self.x.abs(),
            self.y.abs()
        )
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Segment{
    a : Coord,
    b : Coord
}

impl Segment{
    fn new<'a>(a : Coord, b : Coord) -> Self {
       Self {a, b}
    }

    fn get_intersection_point(&self, other: &Segment) -> Option<Coord>{
        let r = self.b.sub(&self.a);
        let s = other.b.sub(&other.a);
        let qp = other.a.sub(&self.a);

        let r_x_cross = r.cross(&s);
        let qp_r_cross = qp.cross(&r);

        if is_zero(r_x_cross) || is_zero(qp_r_cross){
            // they may intersect by overlapping entirely, but we want to get the intersection point... which is undefined for this.
            return None
        }

        if is_zero(r_x_cross) && !is_zero(qp_r_cross){
            return None
        }

        let t = qp.cross(&s) / r_x_cross;
        let u = qp.cross(&r) / r_x_cross;

        if !is_zero(r_x_cross) && between_one_and_zero(t) && between_one_and_zero(u){
            let translate = Coord::new(r.x * t, r.y * t);


            return Some( self.a.add(&translate));
        }

        None
    }

    fn get_steps(&self) -> f32 {
        self.a.sub(&self.b).max()
    }

    fn on_segment(&self, coord: &Coord) -> bool {
        (
                coord.x <= f32::max(self.a.x, self.b.x) &&
                coord.x >= f32::min(self.a.x, self.b.x) &&
                coord.y <= f32::max(self.a.y, self.b.y) &&
                coord.y >= f32::min(self.a.y, self.b.y)
        )
    }
}

fn is_zero(n :f32) -> bool {
    n.abs() < 10e-9
}

fn between_one_and_zero(n : f32) -> bool {
    0.0 <= n && n <= 1.0
}

fn parse_direction_coord(input : &str) -> DirectionCoord {
    let direction = input.chars().next().unwrap();
    let length = input[1..].parse::<f32>().unwrap();

    (direction, length)
}

fn extract_direction_coords_from_line(line : &str) -> Vec<DirectionCoord> {
    line.split(",").map(parse_direction_coord).collect()
}

fn direction_coords_to_coord_list(dir_coords : &Vec<DirectionCoord>) -> Vec<Coord> {
    dir_coords.into_iter().fold(vec![], |mut coord_list, dir_coord| {
        let last_coord :Coord  = if coord_list.len() == 0 {
            Coord::zero()
        } else {
            coord_list[coord_list.len()-1]
        };

        let next_coord = Coord::next(&last_coord, &dir_coord).unwrap();;

        coord_list.push(next_coord);

        coord_list
    })
}

fn create_wire(input : &str) -> Vec<Coord> {
    let dir_coords = extract_direction_coords_from_line(input);

    direction_coords_to_coord_list(&dir_coords)
}

fn get_wires_from_input() -> (Vec<Coord>, Vec<Coord>){
    let input = read_input_file("day-3-part-1-input");
    let raw_wires : Vec<&str> = input.split("\n").collect();

    (
        create_wire(raw_wires[0]),
        create_wire(raw_wires[1])
    )
}

fn collect_segments(coord_list : &Vec<Coord>) -> Vec<Segment> {
    let mut i = 0;

    coord_list.into_iter().fold(vec![], |mut segments, coord|{
        let next_idx = (i + 1) % coord_list.len();
        let next_coord = coord_list[next_idx];

        segments.push( Segment::new(coord.clone(), next_coord.clone()));
        i += 1;

        segments
    })
}

fn count_steps_to_point(segment_list : &Vec<Segment>, point : &Coord) -> f32 {
    let mut steps = segment_list[0].a.max();
    let mut step_list = vec![steps];

    for segment in segment_list{
        if segment.on_segment(point){
            let sub_seg = Segment::new(segment.a, point.clone());
            step_list.push(sub_seg.get_steps());
            steps += sub_seg.get_steps();
            break;
        }else{
            step_list.push(segment.get_steps());
            steps += segment.get_steps();
        }

    }

    steps
}


fn find_intersections(coord_list_a : &Vec<Coord>, coord_list_b : &Vec<Coord>) -> Vec<(Coord, f32)> {
    let segment_list_a = collect_segments(&coord_list_a);
    let segment_list_b = collect_segments(&coord_list_b);

    let mut intersections : Vec<(Coord, f32)> = vec![];

    for seg_a in &segment_list_a{
        for seg_b in &segment_list_b{

            if let Some(intersection) = seg_a.get_intersection_point(seg_b){
                let steps_a = count_steps_to_point(&segment_list_a, &intersection);
                let steps_b = count_steps_to_point(&segment_list_b, &intersection);

                intersections.push((intersection, steps_a + steps_b ))
            }
        }
    }

    intersections
}

fn find_fewest_steps_intersection(intersections : Vec<(Coord, f32)>) -> (Option<Coord>, f32){
    let mut fewest_steps = std::f32::INFINITY;
    let mut closest_intersection = None;

    for (coord, steps) in intersections {
        if steps < fewest_steps {
            fewest_steps = steps;
            closest_intersection = Some(coord);
        }
    }

    (closest_intersection, fewest_steps)
}

fn run_part_1(){
    let (wire_a, wire_b) = get_wires_from_input();
    let intersections = find_intersections(&wire_a, &wire_b).into_iter().map(|(intersection,_)|intersection).collect();

    println!("Answer part 1: {:?}", Coord::zero().closest_to(&intersections));
}

fn run_part_2(){
    let (wire_a, wire_b) = get_wires_from_input();
    let intersections = find_intersections(&wire_a, &wire_b);
    let (_, steps) = find_fewest_steps_intersection(intersections);


    println!("Answer part 2: {}", steps);
}


fn main(){
    run_part_1();
    run_part_2();
}

#[cfg(test)]
mod day_3_tests{
    use crate::*;

    #[test]
    fn parse_string_to_dir_coord(){
        assert_eq!(
            parse_direction_coord("R123"),
            ('R', 123.0)
        );

        // Shouldn't panic for unexpected chars
        assert_eq!(
            parse_direction_coord("Q234"),
            ('Q', 234.0)
        )
    }

    #[test]
    fn get_next_coord_from_prev(){
        let prev_coord = Coord::new(15.0, 35.0);
        let next_coord = Coord::next(&prev_coord, &('D', 250.0)).unwrap();

        assert_eq!(
            next_coord,
            Coord::new(15.0, 35.0-250.0)
        )
    }

    #[test]
    fn get_direction_coords(){
        let line = "R1,D2,U3,L4";

        assert_eq!(
            extract_direction_coords_from_line(line),
            vec![('R', 1.0), ('D', 2.0), ('U',3.0),('L',4.0)]
        );
    }

    #[test]
    fn get_coords_from_dir_coord_list(){
        let list = vec![('R', 1.0), ('D', 2.0), ('U',3.0),('L',4.0)];
        let expected_result = vec![
            Coord::new(1.0, 0.0),
            Coord::new(1.0, -2.0),
            Coord::new(1.0, 1.0),
            Coord::new(-3.0, 1.0),
        ];

        assert_eq!(
            direction_coords_to_coord_list(&list),
            expected_result
        )
    }

    #[test]
    fn get_valid_intersection(){
        let seg1 = Segment::new(
            Coord::zero(),
            Coord::new(5.0, 5.0)
        );

        let seg2 = Segment::new(
            Coord::new(0.0, 5.0),
            Coord::new(5.0, 0.0)
        );

       match seg1.get_intersection_point(&seg2){
           Some(coord) => assert_eq!(coord, Coord::new(2.5, 2.5)),
           _ => panic!("Recieved invalid result")
        }

    }

    #[test]
    fn get_invalid_intersection(){
        let seg1 = Segment::new(
            Coord::new(3.0, 0.0),
            Coord::new(3.0, 4.0)
        );

        let seg2 = Segment::new(
            Coord::new(0.0, 5.0),
            Coord::new(5.0, 5.0)
        );

        match seg1.get_intersection_point(&seg2) {
            None => assert!(true),
            _ => panic!("Received invalid result")
        }
    }

    #[test]
    fn get_intersections() {
        let wire_a = create_wire("R8,U5,L5,D3");
        let wire_b = create_wire("U7,R6,D4,L4");

        let intersections = find_intersections(&wire_a, &wire_b);


        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn get_closest_intersection(){
        let wire_a = create_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire_b = create_wire("U62,R66,U55,R34,D71,R55,D58,R83");
        let intersections = find_intersections(&wire_a, &wire_b);


        let intersection_distances = intersections.into_iter().map(|(intersection,_)| intersection).collect();
        let (_, dist) = Coord::zero().closest_to(&intersection_distances);

        assert_eq!(
            dist,
            159.0
        )
    }

    #[test]
    fn get_fewest_steps_intersection(){
        let wire_a = create_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire_b = create_wire("U62,R66,U55,R34,D71,R55,D58,R83");
        let intersections = find_intersections(&wire_a, &wire_b);


        let (_, steps) = find_fewest_steps_intersection(intersections);

        assert_eq!(
            steps,
            610.0
        )
    }
}