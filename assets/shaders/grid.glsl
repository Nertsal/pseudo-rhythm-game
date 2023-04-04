uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_grid_matrix;

varying vec2 v_pos;

vec2 into_2d(vec3 pos) {
    return pos.xy / pos.z;
}

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;

    vec3 pos = vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    vec4 even_color = vec4(0.1, 0.1, 0.1, 1.0);
    vec4 odd_color = vec4(0.2, 0.2, 0.2, 1.0);

    mat3 screen_to_grid = inverse(u_projection_matrix * u_view_matrix * u_grid_matrix);
    vec2 grid_pos = into_2d(vec3(v_pos, 1.0) * screen_to_grid);

    ivec2 grid_ipos = ivec2(floor(grid_pos - 0.5));

    // let mut cell_pos = vec2(offset.x.trunc() as _, offset.y.trunc() as _);
    // offset = vec2(offset.x.fract(), offset.y.fract());
    // if offset.x < 0.0 {
    //     offset.x += 1.0;
    //     cell_pos.x -= 1;
    // }
    // if offset.y < 0.0 {
    //     offset.y += 1.0;
    //     cell_pos.y -= 1;
    // }

    vec4 color = vec4(0.0);
    if (mod(float(grid_ipos.x + grid_ipos.y), 2.0) == 0.0) {
        color = even_color;
    } else {
        color = odd_color;
    }

    gl_FragColor = color;
}
#endif
