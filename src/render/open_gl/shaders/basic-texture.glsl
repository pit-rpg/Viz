
#<vertex>
#version 330 core
#<uniforms>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aUv;


out vec4 v_color;
out vec2 v_uv;

void main() {
	v_color = vec4(aColor.xyz, 1.0);
	v_uv = aUv;
	gl_Position = matrix_view * matrix_model * vec4(aPos.xyz, 1.0);
}




#<fragment>
#version 330 core
#<uniforms>
#<textures>
in vec4 v_color;
in vec2 v_uv;

layout (location = 0) out vec4 FragColor;

void main() {
	FragColor = texture(texture_color, v_uv);
    // FragColor.x *=v_uv.x;
	// if (texture_color) {
	// } else {
	// 	FragColor = v_color;
	// }
}