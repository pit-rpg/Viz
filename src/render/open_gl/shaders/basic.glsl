
#<vertex>
#version 330 core

uniform mat4 matrix_model;
uniform mat4 matrix_view;

layout (location = 0) in vec3 aPos;

out vec4 v_color;

void main() {
	gl_Position = matrix_view * matrix_model * vec4(aPos.xyz, 1.0);
}


#<fragment>
#version 330 core

uniform vec4 color;

layout (location = 0) out vec4 FragColor;

void main() {
	FragColor = color;
}