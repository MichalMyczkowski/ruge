pub mod textures;
pub mod srtm3;

pub struct HeightMap {
    height: Vec<f32>,
    world_position: Vec<glm::Vec3>,
}

pub fn latlong_to_world(lat: f32, long: f32) -> glm::Vec3 {
    let lat = lat.to_radians();
    let long = long.to_radians();
    glm::normalize(&glm::Vec3::new(
        lat.cos() * long.sin(),
        lat.sin(),
        lat.cos() * long.cos(),
    ))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srtm3_data_loading() {
        //let hgt = srtm3::SRTM3::load("./examples/earth/assets/srtm_data").unwrap();
        //hgt.for_each( |hm| {
        //    println!("height_map len: {}", hm.height.len());
        //    println!("world_pos len: {}", hm.world_position.len());
        //});
        println!("lat 0 long 90 = = {}", latlong_to_world(0.0, 90.0));
        println!("lat 0 long -90 = = {}", latlong_to_world(0.0, -90.0));
        println!("lat 0 long 180 = = {}", latlong_to_world(0.0, 180.0));
        println!("lat 90 long 0 = = {}", latlong_to_world(90.0, 0.0));
        println!("lat 0 long 0 = = {}", latlong_to_world(0.0, 0.0));
        println!("lat -90 long 90 = = {}", latlong_to_world(-90.0, 90.0));
        assert!(false);
    }
}
