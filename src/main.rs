use std::iter;
use std::f64::consts::PI;

type Angle = f64;
type Coord = (f64, f64, f64);
type Vector = (Coord, Coord);

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
    fn convert(&self, coords: Coord) -> Coord {
        let (cam_x, cam_y, cam_z) = self.camera_position;
        let scale = self.distance as f64 / coords.2;
        let x = cam_x + (coords.0 - cam_x) * scale;
        let y = cam_y + (coords.1 - cam_y) * scale;
        return (x, y, cam_z);
    }

    fn draw(&self) {
        let mut z_buffer: Vec<Option<(f64, Coord)>> =
            iter::repeat(None)
            .take((self.width * self.height) as usize)
            .collect();

        for item in self.items.as_slice() {
            let points = item.generate_points();
            for point in points {
                let (coords, incr) = point;
                let (x, y, z) = self.convert(coords);

                if !((x as usize) < self.width && (y as usize) < self.height) {
                    continue;
                }

                if let Some((zz, _)) = z_buffer[self.width * y as usize + x as usize] {
                    if z < zz {
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
    fn get_vector(&self, xy_a: f64, z_a: f64) -> Coord {
        let z_angle = z_a + self.z_angle;
        let xy_angle = xy_a + self.xy_angle;
        let x = self.size * z_angle.sin() * xy_angle.cos() / 2.0;
        let y = self.size * z_angle.sin() * xy_angle.sin() / 2.0;
        let z = self.size * z_angle.cos() / 2.0;
        return (x, y, z);
    }

    fn generate_side(&self, vector: Coord) -> Vec<(Coord, Coord)> {
        todo!();
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
                let vector = self.get_vector(PI / 2.0 * i as f64, PI / 2.0 * j as f64);
                points.extend(self.generate_side(vector));
            }
        }
        return points;
    }
}

trait Item {
    fn generate_points(&self) -> Vec<(Coord, Coord)>;
}

fn main() {
}
