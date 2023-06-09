use std::{iter, io};
use std::f64::consts::PI;

type Coord = (f64, f64, f64);

const LUMINOSITY: &str = ".,-~:;=!*#$@";

struct Renderer {
    width: usize,
    height: usize,
    distance: u32,
    camera_position: Coord,
    light_direction: Coord,
    items: Vec<Box<dyn Item>>,
}

impl Renderer {
    fn new(width: usize, height: usize, distance: u32, camera_position: Coord, light_direction: Coord) -> Self {
        Renderer {
            width,
            height,
            distance,
            camera_position,
            light_direction,
            items: vec![],
        }
    }

    fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    fn convert(&self, coords: Coord) -> Coord {
        let (cam_x, cam_y, cam_z) = self.camera_position;
        let scale = self.distance as f64 / coords.2;
        let x = (coords.0 - cam_x) * scale;
        let y = (coords.1 - cam_y) * scale;

        //println!("{:?}", x);
        //let mut s = String::new();
        //io::stdin().read_line(&mut s);

        return (x, y, cam_z);
    }

    fn draw(&self) {
        let mut z_buffer: Vec<Option<(f64, Coord)>> =
            iter::repeat(None)
            .take((self.width * self.height) as usize)
            .collect();

        for item in self.items.as_slice() {
            let points = item.generate_points();

            // TODO This is DEBUG code
            //println!("{:?}", points);
            //panic!();

            for point in points {
                let (coords, incr) = point;
                let (x, y, z) = self.convert(coords);
                println!("{:?}", coords);

                if !((x as usize) < self.width && (y as usize) < self.height) {
                    continue;
                }

                if let Some((zz, _)) = z_buffer[self.width * y as usize + x as usize] {
                    if z > 0.0 && z < zz {
                        z_buffer[self.width * y as usize + x as usize] = Some((z, incr));
                    }
                } else {
                    z_buffer[self.width * y as usize + x as usize] = Some((z, incr));
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some((_, (a, b, c))) = z_buffer[self.width * y + x] {
                    let (la, lb, lc) = self.light_direction;
                    let lum = (a * la + b * lb + c * lc) as usize;
                    print!("{}", LUMINOSITY.get(lum..lum + 1).unwrap());
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

struct Cube {
    center: Coord,
    size: f64,  // Size of an edge
    xy_angle: f64,
    z_angle: f64,
}

impl Cube {
    fn new(center: Coord, size: f64, xy_angle: f64, z_angle: f64) -> Self {
        Cube {
            center,
            size,
            xy_angle,
            z_angle,
        }
    }

    fn get_vector(&self, xy_a: f64, z_a: f64) -> Coord {
        let z_angle = z_a + self.z_angle;
        let xy_angle = xy_a + self.xy_angle;
        let x = self.size * z_angle.sin() * xy_angle.cos() / 2.0;
        let y = self.size * z_angle.sin() * xy_angle.sin() / 2.0;
        let z = self.size * z_angle.cos() / 2.0;
        return (x, y, z);
    }

    // TODO Bug works only for surfaces parallel to angle z = 0
    fn generate_side(&self, xy: f64, z_a: f64) -> Vec<(Coord, Coord)> {
        let mut ris = vec![];
        let dir = self.get_vector(xy, z_a);

        let x_vec = self.get_vector(xy, z_a + PI / 2.0);
        let start_x = self.center.0 - x_vec.0;

        let y_vec = self.get_vector(xy + PI / 2.0, z_a + PI / 2.0);
        let start_y = self.center.1 - y_vec.1;

        let t = (start_x, start_y, dir.2);
        for x in 0..101 {
            for y in 0..101 {
                let mut tt = t;
                tt.0 += x as f64 * self.size / 100.0;
                tt.1 += y as f64 * self.size / 100.0;
                ris.push((tt, dir));
            }
        }
        return ris;
    }
}

impl Item for Cube {
    /* First coords are the location of the point,
     * the second ones are the vector indicating the
     * direction of the surface normal.
     *
     * It also generates some sides twice, would be more
     * efficient to specify only the 6 sides to render instead
     * rotating on both axes
     */
    fn generate_points(&self) -> Vec<(Coord, Coord)> {
        let mut points: Vec<(Coord, Coord)> = vec![];
        for i in 0..4 {
            for j in 0..4 {
                points.extend(self.generate_side(PI / 2.0 * i as f64, PI / 2.0 * j as f64));
            }
        }
        return points;
    }
}

trait Item {
    fn generate_points(&self) -> Vec<(Coord, Coord)>;
}

fn main() {
    let mut renderer = Renderer::new(
        90,
        60,
        10,
        (400.0, 300.0, 0.0),
        (500.0, 600.0, 0.0),
    );

    renderer.add_item(Box::new(Cube::new((400.0, 300.0, 100.0), 20.0, 0.0, 0.0)));
    renderer.draw();

    let mut s = String::new();
    io::stdin().read_line(&mut s);
}
