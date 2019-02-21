#<vertex>
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;
uniform vec3 position_light;

out vec3 v_pos;
out vec3 v_norm;

void main() {
	gl_Position = matrix_view * matrix_model * vec4(aPos.xyz, 1.0);
	v_pos = vec3(matrix_model * vec4(aPos, 1.0f));
	v_norm = matrix_normal * aNormal;
}


#<fragment>
#version 330 core

layout (location = 0) out vec4 FragColor;
in vec3 v_pos;
in vec3 v_norm;

uniform vec3 color_light;
uniform vec4 color;
uniform vec3 position_light;


float specularStrength = 0.534;

void main() {
	vec3 norm = normalize(v_norm);

	// ambient
	vec3 amb = color_light * 0.1;

	// diffuse
	vec3 light_dir = normalize(position_light - v_pos);
	float diff = max(dot(norm, light_dir), 0.0);
	vec3 diffuse = diff* color_light;

	// specular
	vec3 viewDir = normalize(-v_pos);
	vec3 reflectDir = reflect(-light_dir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
	vec3 specular = specularStrength * spec * color_light;

	vec3 f_color = (diffuse+amb+specular)*color.xyz;

	FragColor = vec4(f_color.xyz, 1.0) ;
}