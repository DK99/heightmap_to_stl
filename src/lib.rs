use stl_io::{Normal, Triangle, Vertex};

pub fn heightmap_to_stl(heightmap: Vec<Vec<f32>>, base_height: f32) -> Vec<Triangle> {
    let width = heightmap.len();
    let width_float = width as f32;
    let height = heightmap[0].len();
    let height_float = height as f32;    

    let default_vertex = Vertex::new([0.0, 0.0, 0.0]);
    let default_normal = Normal::new([0.0, 0.0, 0.0]);
    let default_triangle = Triangle {
        normal: default_normal,
        vertices: [default_vertex, default_vertex, default_vertex]
    };
    let mut mesh = vec![default_triangle; calculate_triangle_count(width as u32, height as u32)];
    let mut mesh_index = 0;

    let mut min_height = f32::MAX;

    // top surface
    for i in 0..(width - 1) {
        for j in 0..(height - 1) {
            let i_float = i as f32;
            let j_float = j as f32;
            let i_usize = i as usize;
            let j_usize = j as usize;

            let vector_a = [i_float,     j_float,        heightmap[i_usize][j_usize]];
            let vector_b = [i_float+1.0, j_float,  heightmap[i_usize+1][j_usize]];
            let vector_c = [i_float,     j_float + 1.0,  heightmap[i_usize][j_usize+1]];

            mesh[mesh_index] = create_triangle(vector_a, vector_b, vector_c, false);
            mesh_index += 1;

            let vector_d = [i_float+1.0, j_float,        heightmap[i_usize+1][j_usize]];
            let vector_e = [i_float+1.0, j_float + 1.0,  heightmap[i_usize+1][j_usize+1]];
            let vector_f = [i_float,     j_float + 1.0,  heightmap[i_usize][j_usize+1]];

            mesh[mesh_index] = create_triangle(vector_d, vector_e, vector_f, false);
            mesh_index += 1;

            if heightmap[i][j] < min_height {
                min_height = heightmap[i][j];
            }
        }
    }

    let model_min_height = min_height - base_height;

    //top base
    for i in 0..(width - 1) {
        let i_float = i as f32;
        let i_usize = i as usize;

        let vector_a = [i_float,     0.0, model_min_height];
        let vector_b = [i_float+1.0, 0.0, model_min_height];
        let vector_c = [i_float,     0.0, heightmap[i_usize][0]];

        mesh[mesh_index] = create_triangle(vector_a, vector_b, vector_c, false);
        mesh_index += 1;

        let vector_d = [i_float+1.0, 0.0, heightmap[i_usize+1][0]];
        let vector_e = [i_float,     0.0, heightmap[i_usize][0]];
        let vector_f = [i_float+1.0, 0.0, model_min_height];

        mesh[mesh_index] = create_triangle(vector_d, vector_e, vector_f, false);
        mesh_index += 1;
    }

    //bottom base
    for i in 0..(width - 1) {
        let i_float = i as f32;
        let i_usize = i as usize;
        
        let vector_a = [i_float,     height_float-1.0, heightmap[i_usize][height-1]];
        let vector_b = [i_float+1.0, height_float-1.0, model_min_height];
        let vector_c = [i_float,     height_float-1.0, model_min_height];

        mesh[mesh_index] = create_triangle(vector_a, vector_b, vector_c, false);
        mesh_index += 1;

        let vector_d = [i_float+1.0, height_float-1.0, model_min_height];
        let vector_e = [i_float,     height_float-1.0, heightmap[i_usize][height-1]];
        let vector_f = [i_float+1.0, height_float-1.0, heightmap[i_usize+1][height-1]];

        mesh[mesh_index] = create_triangle(vector_d, vector_e, vector_f, false);
        mesh_index += 1;
    }

    //right base
    for i in 0..(height - 1) {
        let i_float = i as f32;
        let i_usize = i as usize;

        let vector_a = [width_float-1.0, i_float,     model_min_height];
        let vector_b = [width_float-1.0, i_float+1.0, model_min_height];
        let vector_c = [width_float-1.0, i_float,     heightmap[width-1][i_usize]];

        mesh[mesh_index] = create_triangle(vector_a, vector_b, vector_c, false);
        mesh_index += 1;

        let vector_d = [width_float-1.0, i_float+1.0, heightmap[width-1][i_usize+1]];
        let vector_e = [width_float-1.0, i_float,     heightmap[width-1][i_usize+1]];
        let vector_f = [width_float-1.0, i_float+1.0, model_min_height];

        mesh[mesh_index] = create_triangle(vector_d, vector_e, vector_f, false);
        mesh_index += 1;
    }

    //left base
    for i in 0..(height - 1) {
        let i_float = i as f32;
        let i_usize = i as usize;

        let vector_a = [0.0, i_float,     heightmap[0][i_usize]];
        let vector_b = [0.0, i_float+1.0, model_min_height];
        let vector_c = [0.0, i_float,     model_min_height];

        mesh[mesh_index] = create_triangle(vector_a, vector_b, vector_c, false);
        mesh_index += 1;

        let vector_d = [0.0, i_float+1.0, model_min_height];
        let vector_e = [0.0, i_float,     heightmap[0][i_usize+1]];
        let vector_f = [0.0, i_float+1.0, heightmap[0][i_usize+1]];

        mesh[mesh_index] = create_triangle(vector_d, vector_e, vector_f, false);
        mesh_index += 1;
    }

    //bottom
    let vector_a = [0.0,             height_float-1.0, model_min_height];
    let vector_b = [width_float-1.0, 0.0,              model_min_height];
    let vector_c = [0.0,             0.0,              model_min_height];

    mesh[mesh_index] = create_triangle(vector_a, vector_b, vector_c, false);
    mesh_index += 1;

    let vector_d = [0.0,             height_float-1.0, model_min_height];
    let vector_e = [width_float-1.0, height_float-1.0, model_min_height];
    let vector_f = [width_float-1.0, 0.0,              model_min_height];

    mesh[mesh_index] = create_triangle(vector_d, vector_e, vector_f, false);

    mesh
}

fn create_triangle(vector_a: [f32; 3], vector_b: [f32; 3], vector_c: [f32; 3], flip_normal: bool) -> Triangle {
    let vertex_a = Vertex::new(vector_a);
    let vertex_b = Vertex::new(vector_b);
    let vertex_c = Vertex::new(vector_c);

    let vector_ab = subtract(vector_b, vector_a);
    let vector_ac = subtract(vector_c, vector_a);
    let mut vector_normal = normalize(cartesian_product(vector_ab, vector_ac));

    if flip_normal {
        vector_normal = multiply(vector_normal, -1.0)
    }
    let vertex_normal = Vertex::new(vector_normal);

    Triangle {
        normal: vertex_normal,
        vertices: [vertex_a, vertex_b, vertex_c]
    }
}

fn normalize(a: [f32; 3]) -> [f32; 3] {
    divide(a, length(a))
}

fn length(a: [f32; 3]) -> f32 {
    f32::sqrt(a[0].powf(2.0) + a[1].powf(2.0) + a[2].powf(2.0))
}

fn cartesian_product(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    let mut result = [0.0; 3];
    for i in 0..3 {
        result[i] = (a[(i+1) % 3] * b[(i+2) % 3]) - (a[(i+2) % 3] * b[(i+1) % 3])
    }
    result
}

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    let mut result = [0.0; 3];
    for i in 0..3 {
        result[i] = a[i] - b[i]
    }
    result
}

fn divide(a: [f32; 3], b: f32) -> [f32; 3] {
    multiply(a, 1.0/b)
}

fn multiply(a: [f32; 3], b: f32) -> [f32; 3] {
    let mut result = [0.0; 3];
    for i in 0..3 {
        result[i] = a[i] * b
    }
    result
}

fn calculate_triangle_count(width: u32, height: u32) -> usize {
    // Surfacearea * 2: each pixel needs 2 triangles
    let mut count = (width - 1) * (height - 1) * 2; 

    // Top base has (width - 1) columns, each column needs 2 triangles. Bottom base needs the same count
    count += (width - 1) * 2 * 2;

    // Left base has (height - 1) rows, each row needs 2 triangles. Right base needs the same count
    count += (height - 1) * 2 * 2;

    // bottom of the model needs 2 trianges
    count += 2;

    count as usize
}