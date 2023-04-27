use std::iter;

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

trait Item {
    fn generate_points(&self) -> Vec<(Coord, Coord)>;
}

fn main() {
}
