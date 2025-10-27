use raylib::prelude::{CameraMode, *};
use std::io::{self, Write};

fn input(inp: &str) -> String {
    print!("{}", &inp);
    io::stdout().flush().unwrap();
    let mut inpbuffer = String::new();
    io::stdin().read_line(&mut inpbuffer).unwrap();
    return inpbuffer.trim().to_string();
}

fn main() {
    //init section
    let mut voxels: Vec<Vec<Vec<bool>>>;

    voxels = vec![
        vec![
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
        ],
        vec![
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
        ],
        vec![
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
        ],
        vec![
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, true],
        ],
    ];

    println!("Enter a coordinate to make it true");
    let x: usize = input("x : ").parse::<i32>().unwrap() as usize;
    let y: usize = input("y : ").parse::<i32>().unwrap() as usize;
    let z: usize = input("z : ").parse::<i32>().unwrap() as usize;

    voxels[x][y][z] = true;

    let (mut rl, thread) = raylib::init()
        .size(1000, 700)
        .title("Voxel Engine")
        .resizable()
        .build();
    rl.disable_cursor();

    let mut camera = Camera3D::perspective(
        Vector3 {
            x: (8.0),
            y: (8.0),
            z: (8.0),
        },
        Vector3 {
            x: (2.0),
            y: (2.0),
            z: (2.0),
        },
        Vector3 {
            x: (0.0),
            y: (1.0),
            z: (0.0),
        },
        80.0,
    );

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        let mut drawer = rl.begin_drawing(&thread);
        drawer.clear_background(Color::RAYWHITE);

        let mut d3 = drawer.begin_mode3D(&camera);
        for i in 0..voxels.len() {
            for j in 0..voxels[i].len() {
                for k in 0..voxels[i][j].len() {
                    if voxels[i][j][k] {
                        d3.draw_cube(
                            Vector3::new(i as f32 * 1.5, j as f32 * 1.5, k as f32 * 1.5),
                            1.5,
                            1.5,
                            1.5,
                            Color::RED,
                        );
                        d3.draw_cube_wires(
                            Vector3::new(i as f32 * 1.5, j as f32 * 1.5, k as f32 * 1.5),
                            1.5,
                            1.5,
                            1.5,
                            Color::BLACK,
                        );
                    }
                }
            }
        }
    }
}
