pub const RAW: &str = include_str!("map.txt");
pub const MAP_HEIGHT: usize = 16;
pub const MAP_WIDTH: usize = 16;

pub fn data() -> Vec<char> {
    RAW.replace('\n', "").chars().collect()
}
