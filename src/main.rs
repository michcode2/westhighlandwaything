use std::{
    cmp::Ordering,
    f64::consts::PI,
    fmt::{self, write},
    fs::{self, File},
    io::Write,
    num,
    ops::Sub,
    path::Display,
};

use dted2::DTEDData;
use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use rayon::prelude::*;

fn main() {
    let filenames = vec!["../Copernicus_DSM_10_N55_00_W005_00_DEM.dt2", "../Copernicus_DSM_10_N56_00_W005_00_DEM.dt2", "../Copernicus_DSM_10_N56_00_W006_00_DEM.dt2"];

    let data = filenames.into_iter().map(|file| DTEDData::read(file).unwrap()).collect::<Vec<DTEDData>>();

    data.iter().for_each(|dem_slice| println!("extents of {} are {:?} and then {:?}", dem_slice.metadata.filename, dem_slice.max, dem_slice.min));
    let start_lat = 56.7645839;
    let start_lon = -5.0476345;
    /*let day_2 = read_gpx("../Day_2_Drymen_to_Inchcailloch.gpx".to_string());
    let day_3 = read_gpx("../Day_3_Inchcailloch_to_Rowardennan.gpx".to_string());
    let day_4 = read_gpx("../Day_4_Rowardennan_to_Beinglas.gpx".to_string());
    let day_5 = read_gpx("../Day_5_Beinglas_to_Tyndrum.gpx".to_string());
    let day_6 = read_gpx("../Day_6_Tyndrum_to_Inveroran.gpx".to_string());
    let _all_days = [day_2, day_3, day_4, day_5, day_6].concat();
    */
    let day_9 = read_gpx("../Day_9_Kinlochleven_to_Glen_Nevis.gpx".to_string());
    //make_image(start_lat, start_lon, 0.2, 0.2, &data, Some(day_9));
    make_stl(start_lat, start_lon, 0.2, 0.2, &data, true);
}

fn make_stl(lat_0: f64, long_0: f64, width: f64, height: f64, data: &Vec<DTEDData>, add_base: bool) {
    let lat_low = lat_0 - width / 2.0;
    let long_low = long_0 - height / 2.0;
    let lat_high = lat_0 + width / 2.0;
    let long_high = long_0 + height / 2.0;

    let step_size = 30.0; // meters
    let num_width = (width * latitude_length(long_0) / step_size) as u32; // try the dimensional analysis, it works
    let num_height = (height * longitude_length(long_0) / step_size) as u32; // try the dimensional analysis, it works

    println!("image will be {}x{}", num_width, num_height);
    println!("bottom left {},{}", lat_low, long_low);
    println!("top right {},{}", lat_high, long_high);

    let elev_data: Vec<Vec<f64>> = (0..num_height)
        .into_iter()
        .map(|index_long| {
            let current_longitude = long_low + (index_long as f64 * step_size) / longitude_length(long_0);
            (0..num_width)
                .into_iter()
                .map(|index_lat| {
                    let current_latitude = lat_low + index_lat as f64 * step_size / latitude_length(current_longitude);
                    find_elevation(current_latitude, current_longitude, data).expect(&format!("lat: {}, lon: {}", current_latitude, current_longitude))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let mut triangles: Vec<Triangle3D> = vec![];

    for x in 0..elev_data.len() - 1 {
        for y in 0..elev_data[0].len() - 1 {
            let h1 = elev_data[x][y];
            let p1 = Vector3::new(x as f64 * step_size, y as f64 * step_size, h1);

            let h2 = elev_data[x + 1][y];
            let p2 = Vector3::new((x as f64 + 1.0) * step_size, y as f64 * step_size, h2);

            let h3 = elev_data[x][y + 1];
            let p3 = Vector3::new(step_size * (x as f64), step_size * (y as f64 + 1.0), h3);

            let h4 = elev_data[x + 1][y + 1];
            let p4 = Vector3::new(step_size * (x as f64 + 1.0), step_size * (y as f64 + 1.0), h4);

            let t1 = Triangle3D::new(p1, p2, p3);
            let t2 = Triangle3D::new(p4, p2, p3);
            triangles.push(t1);
            triangles.push(t2);
        }
    }

    if add_base {
        let z_base = -500.0;
        let num_y = elev_data[0].len() - 1; // not quite number of them but it makes the code better to look at
        let num_x = elev_data.len() - 1;

        let x_middle = num_x as f64 / 2.0 * step_size;
        let y_middle = num_y as f64 / 2.0 * step_size;
        let p_middle = Vector3::new(x_middle, y_middle, z_base);

        for x in 0..num_x {
            let p1 = Vector3::new(x as f64 * step_size, 0.0, elev_data[x][0]);
            let p2 = Vector3::new((x + 1) as f64 * step_size, 0.0, elev_data[x + 1][0]);
            let p3 = Vector3::new(x as f64 * step_size, 0.0, z_base);
            let p4 = Vector3::new((x + 1) as f64 * step_size, 0.0, z_base);
            triangles.push(Triangle3D::new(p1, p2, p3));
            triangles.push(Triangle3D::new(p4, p2, p3));
            triangles.push(Triangle3D::new(p3, p4, p_middle));

            let p5 = Vector3::new(x as f64 * step_size, num_y as f64 * step_size, elev_data[x][num_y]);
            let p6 = Vector3::new((x + 1) as f64 * step_size, num_y as f64 * step_size, elev_data[x + 1][num_y]);
            let p7 = Vector3::new(x as f64 * step_size, num_y as f64 * step_size, z_base);
            let p8 = Vector3::new((x + 1) as f64 * step_size, num_y as f64 * step_size, z_base);
            triangles.push(Triangle3D::new(p5, p6, p7));
            triangles.push(Triangle3D::new(p8, p6, p7));
            triangles.push(Triangle3D::new(p7, p8, p_middle));
        }
        for y in 0..num_y {
            let p1 = Vector3::new(0.0, y as f64 * step_size, elev_data[0][y]);
            let p2 = Vector3::new(0.0, (y + 1) as f64 * step_size, elev_data[0][y + 1]);
            let p3 = Vector3::new(0.0, y as f64 * step_size, z_base);
            let p4 = Vector3::new(0.0, (y + 1) as f64 * step_size, z_base);
            triangles.push(Triangle3D::new(p1, p2, p3));
            triangles.push(Triangle3D::new(p4, p2, p3));
            triangles.push(Triangle3D::new(p3, p4, p_middle));

            let p5 = Vector3::new(num_x as f64 * step_size, y as f64 * step_size, elev_data[num_x][y]);
            let p6 = Vector3::new(num_x as f64 * step_size, (y + 1) as f64 * step_size, elev_data[num_x][y + 1]);
            let p7 = Vector3::new(num_x as f64 * step_size, y as f64 * step_size, z_base);
            let p8 = Vector3::new(num_x as f64 * step_size, (y + 1) as f64 * step_size, z_base);
            triangles.push(Triangle3D::new(p5, p6, p7));
            triangles.push(Triangle3D::new(p8, p6, p7));
            triangles.push(Triangle3D::new(p7, p8, p_middle));
        }
        println!("num_x {}, num_y {}", num_x, num_y);
    }
    let mut stl_string = String::from("solid name\n");
    for tri in triangles {
        stl_string.push_str(format!("{:?}\n", tri).as_str());
    }
    stl_string.push_str("endsolid name");
    let mut file = File::create("./obj.stl").unwrap();
    file.write(stl_string.as_bytes());
}

#[derive(Clone, Copy)]
struct Triangle3D {
    c1: Vector3<f64>,
    c2: Vector3<f64>,
    c3: Vector3<f64>,
}

impl Triangle3D {
    fn new(c1: Vector3<f64>, c2: Vector3<f64>, c3: Vector3<f64>) -> Triangle3D {
        Triangle3D { c1, c2, c3 }
    }

    fn unit_normal(&self) -> (f64, f64, f64) {
        let i = self.c1 - self.c2;
        let j = self.c3 - self.c2;
        let mut normal = i.cross(&j) / (i.cross(&j)).norm();
        if normal.z < 0.0 {
            normal *= -1.0;
        }
        return (normal.x, normal.y, normal.z);
    }
}

impl fmt::Debug for Triangle3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let normal = self.unit_normal();
        write!(
            f,
            r#"facet normal {} {} {}
    outer loop
        {}
        {}
        {}
    endloop
endfacet"#,
            normal.0,
            normal.1,
            normal.2,
            stl_vector(&self.c1),
            stl_vector(&self.c2),
            stl_vector(&self.c3)
        )
    }
}
fn stl_vector(vec: &Vector3<f64>) -> String {
    format!("vertex {:e} {:e} {:e}", vec.x, vec.y, vec.z)
}

/// everything in degrees
fn make_image(lat_0: f64, long_0: f64, width: f64, height: f64, data: &Vec<DTEDData>, path_data: Option<Vec<(f64, f64)>>) {
    let lat_low = lat_0 - width / 2.0;
    let long_low = long_0 - height / 2.0;
    let lat_high = lat_0 + width / 2.0;
    let long_high = long_0 + height / 2.0;

    let step_size = 10.0; // meters
    let num_width = (width * latitude_length(long_0) / step_size) as u32; // try the dimensional analysis, it works
    let num_height = (height * longitude_length(long_0) / step_size) as u32; // try the dimensional analysis, it works

    println!("image will be {}x{}", num_width, num_height);
    println!("bottom left {},{}", lat_low, long_low);
    println!("top right {},{}", lat_high, long_high);

    let elev_data: Vec<Vec<f64>> = (0..num_height)
        .into_iter()
        .map(|index_long| {
            let current_longitude = long_low + (index_long as f64 * step_size) / longitude_length(long_0);
            (0..num_width)
                .into_iter()
                .map(|index_lat| {
                    let current_latitude = lat_low + index_lat as f64 * step_size / latitude_length(current_longitude);
                    find_elevation(current_latitude, current_longitude, data).expect(&format!("lat: {}, lon: {}", current_latitude, current_longitude))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    println!("image is {}x{}", elev_data.len(), elev_data[0].len());
    let lowest = elev_data
        .iter()
        .map(|strip| {
            strip.iter().fold(f64::INFINITY, |a, &b| match PartialOrd::partial_cmp(&a, &b) {
                None => 0.0,
                Some(Ordering::Less) => a,
                Some(_) => b,
            })
        })
        .fold(f64::INFINITY, |a, b| match PartialOrd::partial_cmp(&a, &b) {
            None => 0.0,
            Some(Ordering::Less) => a,
            Some(_) => b,
        });

    let highest = elev_data
        .iter()
        .map(|strip| {
            strip.iter().fold(0.0, |a, &b| match PartialOrd::partial_cmp(&a, &b) {
                None => 0.0,
                Some(Ordering::Less) => b,
                Some(_) => a,
            })
        })
        .fold(0.0, |a, b| match PartialOrd::partial_cmp(&a, &b) {
            None => 0.0,
            Some(Ordering::Less) => b,
            Some(_) => a,
        });

    let normalised_elev_data = elev_data.iter().map(|strip| strip.iter().map(|p| (p - lowest) / (highest - lowest)).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();

    println!("{} {}", lowest, highest);
    let mut image = RgbImage::new(num_height + 1, num_width + 1);

    for x in 0..num_height - 1 {
        for y in 0..num_width - 1 {
            let tallness = normalised_elev_data[x as usize][y as usize];
            let mut r = (255.0 * (1.0 + (2.0 * PI * tallness).cos()) / 2.0) as u8;
            let mut g = (255.0 * (1.0 + (2.0 + 2.0 * PI * tallness).cos()) / 2.0) as u8;
            let mut b = (255.0 * (1.0 + (4.0 + 2.0 * PI * tallness).cos()) / 2.0) as u8;
            let grey = (tallness * 255.0) as u8;
            if elev_data[x as usize][y as usize] < 10.0 {
                r = 0;
                g = 0;
                b = 0;
            }

            let this_long = long_low + x as f64 * step_size / longitude_length(long_0);
            let this_lat = lat_low + y as f64 * step_size / latitude_length(long_0);

            let adjusted_x = x;
            let adjusted_y = num_width - y;

            image.put_pixel(adjusted_x, adjusted_y, Rgb([grey, grey, grey]));
            let error = 1e-4;

            if let Some(path) = &path_data {
                if path.par_iter().filter(|p| (p.0 - this_lat).abs() < error && (p.1 - this_long).abs() < error).count() > 0 {
                    image.put_pixel(adjusted_x, adjusted_y, Rgb([255, 255, 255]));
                }
            }
        }
        println!("{:?}", 100.0 * x as f64 / num_height as f64)
    }
    image.save("./elevation.png");
}

/// length of one degree of longitude at a given longitude (meters)
fn longitude_length(long_deg: f64) -> f64 {
    let long_rad = long_deg * PI / 180.0;
    return 111132.954 - 559.822 * (2.0 * long_rad).cos() * (4.0 * long_rad).cos();
}

/// length of one degree of latitude at a given longitude (meters)
fn latitude_length(long_deg: f64) -> f64 {
    let long_rad = long_deg * PI / 180.0;
    let numerator = PI * 6378137.0 * long_rad.cos();
    let denominator = 180.0 * (1.0 - -0.00669437999014 * long_rad.sin().sin()).powf(0.5);
    return numerator / denominator;
}

fn find_elevation(lat: f64, lon: f64, data: &Vec<DTEDData>) -> Option<f64> {
    let matching = data.iter().map(|dem_slice| dem_slice.get_elevation(lat, lon)).filter(|e| e.is_some()).collect::<Vec<Option<f64>>>();
    return match matching.len() {
        0 => None,
        1 => matching[0],
        _ => panic!("{} {} in multiple dem slices", lat, lon),
    };
}

fn read_gpx(path: String) -> Vec<(f64, f64)> {
    let file = fs::read_to_string(path).unwrap();
    let mut points = vec![];
    for line in file.lines() {
        if line.contains("<trkpt ") {
            let bits = line.split("\"").collect::<Vec<&str>>();
            let lat = bits[1].parse::<f64>().unwrap();
            let long = bits[3].parse::<f64>().unwrap();
            points.push((lat, long));
        }
    }
    points
}
