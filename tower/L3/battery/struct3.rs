struct Vec3 { x: i32, y: i32, z: i32 }
fn main() {
    let v = Vec3 { x: 1, y: 2, z: 3 };
    let w = Vec3 { x: 10, y: 20, z: 30 };
    std::process::exit(v.x * w.x + v.y * w.y + v.z * w.z);   // 10+40+90 = 140
}
