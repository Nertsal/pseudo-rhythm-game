uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;

varying vec2 v_pos;

float atan2(vec2 pos) {
    return atan(pos.y, pos.x);
}

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform float u_radius_inner;
uniform float u_angle_min;
uniform float u_angle_max;

void main() {
    // Check radius
    float radius = length(v_pos);
    if (radius > 1.0 || radius < u_radius_inner) {
        discard;
    }

    // Check angle
    float angle = atan2(v_pos);
    if (angle < u_angle_min || angle > u_angle_max) {
        discard;
    }

    gl_FragColor = u_color;
}
#endif
