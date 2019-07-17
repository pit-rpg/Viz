#<vertex>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUv;
uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;

out vec3 n;

void main() {
	gl_Position = matrix_view * matrix_model * vec4(aPos.xyz, 1.0);
	n = matrix_normal * aNormal;
}




#<fragment>
uniform sampler2D texture_color;

in vec3 n;

layout (location = 0) out vec4 FragColor;

void main() {
	vec2 x = (normalize(n).xy * 0.5) + 0.5;
	vec3 base = texture2D( texture_color, x ).rgb;

	FragColor = vec4( base, 1. );
}