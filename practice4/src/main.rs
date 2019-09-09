extern crate kiss3d;
extern crate nalgebra as na;
extern crate image;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    //let ref mut fout = File::create(&Path::new("./test.png")).unwrap();
    //let _ = image::ImageRgb8(imgbuf).save(fout);
    let _ = window.render();
    let mut imgbuf = window.snap_image();
    //let _ = image::ImageRgb8(imgbuf).save("./test.png");
    let _ = image::ImageRgb8(imgbuf).save(&Path::new("./test.png"));

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}
