use std::{
    cmp::Ordering,
    f64::consts::PI,
    fmt,
    fs::{self, File},
    io::Write,
};

use dted2::DTEDData;
use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use rayon::prelude::*;

const TILE_SIZE: f64 = 10000.0;

fn main() {
    let filenames = vec!["../Copernicus_DSM_10_N55_00_W005_00_DEM.dt2", "../Copernicus_DSM_10_N56_00_W005_00_DEM.dt2", "../Copernicus_DSM_10_N56_00_W006_00_DEM.dt2"];

    let data = filenames.into_iter().map(|file| DTEDData::read(file).unwrap()).collect::<Vec<DTEDData>>();

    data.iter().for_each(|dem_slice| println!("extents of {} are {:?} and then {:?}", dem_slice.metadata.filename, dem_slice.max, dem_slice.min));
    let start_lat = 56.7645839;
    let start_lon = -5.0476345;

    let kinlochleven_lat = 56.7146178;
    let kinlochleven_long = -4.9687199;

    let balmaha_lat = 56.0643991;
    let balmaha_long = -4.5816642;
    let milngavnie_lat = 55.9084616;
    let milngavnie_long = -4.37;

    let move_north = |loc: (f64, f64)| {
        let (lat, long) = loc;
        let height = TILE_SIZE / latitude_length(start_lat);
        let width = TILE_SIZE / longitude_length(start_lat);
        (lat + height, long)
    };
    let move_east = |loc: (f64, f64)| {
        let (lat, long) = loc;
        let height = TILE_SIZE / latitude_length(start_lat);
        let width = TILE_SIZE / longitude_length(start_lat);
        (lat, long + width)
    };
    let move_west = |loc: (f64, f64)| {
        let (lat, long) = loc;
        let height = TILE_SIZE / latitude_length(start_lat);
        let width = TILE_SIZE / longitude_length(start_lat);
        (lat, long - width)
    };

    let mut squares = vec![(milngavnie_lat, milngavnie_long)]; //0
    squares.push(move_north(squares[0])); //  1
    squares.push(move_west(squares[1])); //   2
    squares.push(move_north(squares[2])); //  3
    squares.push(move_west(squares[3])); //   4 - rowardennan
    squares.push(move_north(squares[4])); //  5
    squares.push(move_north(squares[5])); //  6
    squares.push(move_west(squares[6])); //   7
    squares.push(move_north(squares[6])); //  8
    squares.push(move_north(squares[7])); //  9
    squares.push(move_north(squares[9])); // 10
    squares.push(move_north(squares[10])); //11
    squares.push(move_north(squares[11])); //12
    squares.push(move_west(squares[12])); // 13 - kinlochleven
    squares.push(move_north(squares[13])); //14 - ben nevis
    squares.push(move_west(squares[14])); // 15
    squares.push(move_north(squares[15])); //16 - fort william
    squares.push(move_west(squares[2])); // 17 - inchcailloch
    squares.push(move_north(squares[14])); //18 - ben nevis north
    squares.push(move_west(squares[5])); // 19 - northwest loch lomond
    squares.push(move_west(squares[11])); // 21 - cool mountain by kingshouse
    println!("{:?}", squares);

    /* -------------------------- for images(boooring) ------------------------------
    let names = [
        "../Day_2_Drymen_to_Inchcailloch.gpx",
        "../Day_3_Inchcailloch_to_Rowardennan.gpx",
        "../Day_4_Rowardennan_to_Beinglas.gpx",
        "../Day_5_Beinglas_to_Tyndrum.gpx",
        "../Day_6_Tyndrum_to_Inveroran.gpx",
        "../Day_7_Inveroran_to_Kingshouse.gpx",
        "../Day_8_Kingshouse_to_Kinlochleven.gpx",
        "../Day_9_Kinlochleven_to_Glen_Nevis.gpx",
        "../Day_10_Glen_Nevis_to_Fort_William.gpx",
    ];
    let mut iter_days = names.iter().map(|f| read_gpx(f.to_string())).flatten().collect::<Vec<(f64, f64)>>();
    let mut waypoints = vec![];
    waypoints.push((55.941457, -4.3180973)); //  day 1
    waypoints.push((55.9587088, -4.3316057)); // day 1
    waypoints.push((55.9636016, -4.3426032)); // day 1
    waypoints.push((56.0129791, -4.3688067)); // day 1
    waypoints.push((56.0338561, -4.3903345)); // day 1
    waypoints.push((56.0338911, -4.3902921)); // day 1
    waypoints.push((56.0518637, -4.4103411)); // day 1
    waypoints.push((56.0656151, -4.4391594)); // ardlui
    waypoints.push((56.3924312, -4.6200532)); // crainlarich
    waypoints.push((56.438124, -4.7124773)); // green welly stop
    waypoints.push((56.5160846, -4.7653192)); // bridge of orchy
    waypoints.push((56.532745, -4.8080007)); // inveroran hotel
    waypoints.push((56.6512179, -4.8407526)); // kingshouse
    waypoints.push((56.714449, -4.9657868)); // kinlochleven
    waypoints.push((56.7984163, -5.01039)); // ben nevis
    let mut reduced_path = vec![];
    let reduce_factor = 200;
    for i in 0..iter_days.len() {
        if i % reduce_factor == 0 {
            reduced_path.push(iter_days[i]);
        }
    }

    */
    //make_image(start_lat, start_lon, 0.2, 0.2, &data, Some(&reduced_path), None);
    //  make_image_meters(start_lat, start_lon, TILE_SIZE, &data, Some(&iter_days), Some("nevis.png"));
    //make_image_meters(kinlochleven_lat, kinlochleven_long, TILE_SIZE, &data, Some(&iter_days), Some("kinlochleven.png"));
    //make_image_meters(balmaha_lat, balmaha_long, TILE_SIZE, &data, Some(&reduced_path), Some("balmaha.png"));
    //make_image_meters(milngavnie_lat, milngavnie_long, TILE_SIZE, &data, Some(&reduced_path), Some("milngavnie.png"));
    //make_stl(start_lat, start_lon, 0.2, 0.2, &data, true);
    //select_and_make_plates(start_lat, start_lon, 1e4, &data, &_all_days);
    //make_plate(TILE_SIZE, TILE_SIZE, start_lat, start_lon, &data);

    (0..squares.len()).into_par_iter().for_each(|i| {
        //make_image_meters(squares[i].0, squares[i].1, TILE_SIZE, &data, Some(&reduced_path), Some(format!("{}.png", i).as_str()), Some(&waypoints));
        make_plate(TILE_SIZE, TILE_SIZE, squares[i].0, squares[i].1, &data, Some(format!("{}.stl", i).as_str()));
    });
}
fn make_image_meters(start_lat: f64, start_long: f64, side_len_m: f64, data: &Vec<DTEDData>, path: Option<&Vec<(f64, f64)>>, filename: Option<&str>, waypoints: Option<&Vec<(f64, f64)>>) {
    let height = side_len_m / latitude_length(start_lat);
    let width = side_len_m / longitude_length(start_lat);
    let lat_0 = start_lat + height / 2.0;
    let long_0 = start_long + width / 2.0;
    make_image(lat_0, long_0, width, height, &data, path, filename, waypoints);
}

///lat_0 and long_0 are the bottom right of the first plates
fn select_and_make_plates(lat_0: f64, long_0: f64, side_len_m: f64, data: &Vec<DTEDData>, path_data: &Vec<(f64, f64)>) {
    let calculate_bounds = |lat_start: f64, long_start: f64, side_len_m: f64| {
        let lat_high = lat_start + side_len_m / latitude_length(long_start);
        let long_high = long_start + side_len_m / longitude_length(long_start);
        return (lat_start, long_start, lat_high, long_high);
    };
    let check_in_bounds = |point: (f64, f64), bounds: (f64, f64, f64, f64)| {
        let (lat_low, long_low, lat_high, long_high) = bounds;
        let (lat, long) = point;
        return lat_low < lat && lat < lat_high && long_low < long && long < long_high;
    };
    let mut reduced_path = vec![];
    let reduce_factor = 1000;
    for i in 0..path_data.len() {
        if i % reduce_factor == 0 {
            reduced_path.push(path_data[i]);
        }
    }
    let mut plate_locations = vec![(lat_0, long_0)];
    let mut plate_bounds = vec![calculate_bounds(lat_0, long_0, side_len_m)];

    for (lat, long) in reduced_path {
        let mut ok = false;
        for plate_index in 0..plate_locations.len() {
            if check_in_bounds((lat, long), plate_bounds[plate_index]) {
                ok = true;
                break;
            }
        }
        if ok {
            continue;
        }

        //logic to add next plate
        for vertical in -1..1 {
            for horizontal in -1..1 {}
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len).map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<T>>()).collect()
}

fn make_plate(width_m: f64, height_m: f64, lat_low: f64, long_low: f64, data: &Vec<DTEDData>, filename: Option<&str>) {
    let step_size = 30.0;
    let num_height = (height_m / step_size) as usize;
    let num_width = (width_m / step_size) as usize;
    let elev_data_t: Vec<Vec<f64>> = (0..num_height)
        .into_iter()
        .map(|index_lat| {
            let current_latitude = lat_low + (index_lat as f64 * step_size / latitude_length(lat_low));
            (0..num_width)
                .into_iter()
                .map(|index_long| {
                    let current_longitude = long_low + (index_long as f64 * step_size / longitude_length(current_latitude));
                    find_elevation(current_latitude, current_longitude, data).expect(&format!("lat: {}, lon: {}", current_latitude, current_longitude))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();
    let mut triangles: Vec<Triangle3D> = vec![];
    let elev_data = transpose(elev_data_t);

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
    let mut stl_bytes = vec![65_u8; 80];
    stl_bytes.append(&mut (triangles.len() as u32).to_le_bytes().to_vec());
    for tri in triangles {
        stl_bytes.append(&mut tri.bytes_representation());
    }
    let mut file;
    if let Some(name) = filename {
        file = File::create(format!("{}", name)).unwrap();
    } else {
        file = File::create(format!("{},{}.stl", lat_low, long_low)).unwrap();
    }
    file.write_all(&stl_bytes[..]).unwrap();
}

fn make_stl(lat_0: f64, long_0: f64, width: f64, height: f64, data: &Vec<DTEDData>, add_base: bool) {
    let lat_low = lat_0 - width / 2.0;
    let long_low = long_0 - height / 2.0;
    let lat_high = lat_0 + width / 2.0;
    let long_high = long_0 + height / 2.0;

    let step_size = 30.0; // meters
    let num_width = (width * latitude_length(long_0) / step_size) as u32; // try the dimensional analysis, it works
    let num_height = (height * longitude_length(long_0) / step_size) as u32; // try the dimensional analysis, it works

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
    let mut stl_bytes = vec![65_u8; 80];
    stl_bytes.append(&mut (triangles.len() as u32).to_le_bytes().to_vec());
    for tri in triangles {
        stl_bytes.append(&mut tri.bytes_representation());
    }
    let mut file = File::create("./obj.stl").unwrap();
    file.write_all(&stl_bytes[..]).unwrap();
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

    fn bytes_representation(&self) -> Vec<u8> {
        let mut value = vec![];
        let i = self.c1 - self.c2;
        let j = self.c3 - self.c2;
        let mut normal = i.cross(&j) / (i.cross(&j)).norm();
        if normal.z < 0.0 {
            normal *= -1.0;
        }
        value.append(&mut (normal.x as f32).to_le_bytes().to_vec());
        value.append(&mut (normal.y as f32).to_le_bytes().to_vec());
        value.append(&mut (normal.z as f32).to_le_bytes().to_vec());

        value.append(&mut (self.c1.x as f32).to_le_bytes().to_vec());
        value.append(&mut (self.c1.y as f32).to_le_bytes().to_vec());
        value.append(&mut (self.c1.z as f32).to_le_bytes().to_vec());

        value.append(&mut (self.c2.x as f32).to_le_bytes().to_vec());
        value.append(&mut (self.c2.y as f32).to_le_bytes().to_vec());
        value.append(&mut (self.c2.z as f32).to_le_bytes().to_vec());

        value.append(&mut (self.c3.x as f32).to_le_bytes().to_vec());
        value.append(&mut (self.c3.y as f32).to_le_bytes().to_vec());
        value.append(&mut (self.c3.z as f32).to_le_bytes().to_vec());

        value.push(0);
        value.push(0);

        return value;
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
fn make_image(lat_0: f64, long_0: f64, width: f64, height: f64, data: &Vec<DTEDData>, path_data: Option<&Vec<(f64, f64)>>, filename: Option<&str>, waypoints: Option<&Vec<(f64, f64)>>) {
    let lat_low = lat_0 - height / 2.0;
    let long_low = long_0 - width / 2.0;
    let lat_high = lat_0 + height / 2.0;
    let long_high = long_0 + width / 2.0;

    let step_size = 30.0; // meters
    let num_width = (width * longitude_length(lat_0) / step_size) as u32; // try the dimensional analysis, it works
    let num_height = (height * latitude_length(lat_0) / step_size) as u32; // try the dimensional analysis, it works

    println!(
        "{} will be {}x{}\nbottom left {}, {}\ntop right {}, {}",
        filename.unwrap_or("image"),
        num_width,
        num_height,
        lat_low,
        long_low,
        lat_high,
        long_high
    );

    let elev_data: Vec<Vec<f64>> = (0..num_height)
        .into_iter()
        .map(|index_long| {
            let current_longitude = long_low + (index_long as f64 * step_size) / longitude_length(lat_0);
            (0..num_width)
                .into_iter()
                .map(|index_lat| {
                    let current_latitude = lat_low + index_lat as f64 * step_size / latitude_length(lat_0);
                    find_elevation(current_latitude, current_longitude, data).expect(&format!("lat: {}, lon: {}", current_latitude, current_longitude))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    /*let lowest = elev_data
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
    */
    //let normalised_elev_data = elev_data.iter().map(|strip| strip.iter().map(|p| (p - lowest) / (highest - lowest)).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();
    let colourize = |height: f64| {
        return match height {
            500.0..10000.0 => {
                let r = 0.25 * height - 500.0 / 4.0;
                let g = 0.0005 * (height - 960.0).powi(2) + 150.0;
                let b = 0.2 * height - 50.0;
                (r as u8, g as u8, b as u8)
            }
            10.0..500.0 => (0, 255, (-height * 0.3 + 200.0) as u8),
            -1000.0..10.0 => (0, 0, 255),
            _ => (0, 0, 0),
        };
    };

    let mut image = RgbImage::new(num_height + 1, num_width + 1);

    for x in 0..num_height - 1 {
        for y in 0..num_width - 1 {
            /*  let tallness = normalised_elev_data[x as usize][y as usize];
            let mut r = (255.0 * (1.0 + (2.0 * PI * tallness).cos()) / 2.0) as u8;
            let mut g = (255.0 * (1.0 + (2.0 + 2.0 * PI * tallness).cos()) / 2.0) as u8;
            let mut b = (255.0 * (1.0 + (4.0 + 2.0 * PI * tallness).cos()) / 2.0) as u8;
            let grey = (tallness * 255.0) as u8;
            if elev_data[x as usize][y as usize] < 10.0 {
                r = 0;
                g = 0;
                b = 0;
            }*/

            let this_long = long_low + x as f64 * step_size / longitude_length(lat_0);
            let this_lat = lat_low + y as f64 * step_size / latitude_length(lat_0);

            let adjusted_x = x;
            let adjusted_y = num_width - y;

            let (r, g, b) = colourize(elev_data[x as usize][y as usize]);
            image.put_pixel(adjusted_x, adjusted_y, Rgb([r, g, b]));
            let error = 1e-3;
            let error_waypoint = 5e-3;

            if let Some(path) = &path_data {
                if path.par_iter().filter(|p| (p.0 - this_lat).abs() < error && (p.1 - this_long).abs() < error).count() > 0 {
                    // leads to squashing because lat and long are quite different. could make it a number of meters to return to square, not worth the computation for now
                    image.put_pixel(adjusted_x, adjusted_y, Rgb([255, 0, 0]));
                }
            }

            if let Some(points) = &waypoints {
                if points.par_iter().filter(|p| (p.0 - this_lat).abs() < error_waypoint && (p.1 - this_long).abs() < error_waypoint).count() > 0 {
                    // leads to squashing because lat and long are quite different. could make it a number of meters to return to square, not worth the computation for now
                    image.put_pixel(adjusted_x, adjusted_y, Rgb([255, 0, 255]));
                }
            }
        }
        //println!("{:?}", 100.0 * x as f64 / num_height as f64)
    }
    if let Some(name) = filename {
        image.save(name).unwrap();
    } else {
        image.save("./elevation.png").unwrap();
    }
}

/// length of one degree of longitude (east-west) at a given longitude (meters)
fn longitude_length(lat_deg: f64) -> f64 {
    let lat_rad = lat_deg * PI / 180.0;
    let numerator = PI * 6378137.0 * lat_rad.cos();
    let denominator = 180.0 * (1.0 - -0.00669437999014 * lat_rad.sin().sin()).powf(0.5);
    return numerator / denominator;
}

/// length of one degree of latitude (north/south) at a given longitude (meters)
fn latitude_length(lat_deg: f64) -> f64 {
    let lat_rad = lat_deg * PI / 180.0;
    return 111132.954 - 559.822 * (2.0 * lat_rad).cos() * (4.0 * lat_rad).cos();
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
