
#<vertex>
#version 330 core
#<uniforms>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec4 v_color;

void main() {
	v_color = vec4(aColor.xyz, 1.0);
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}




#<fragment>
#version 330 core
#<uniforms>
#<textures>
in vec4 v_color;

layout (location = 0) out vec4 FragColor;

void main() {
	FragColor = v_color;
}