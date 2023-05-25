uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;

void main() {
    vec4 color = u_color;
    gl_FragColor = color;
}
#endif
