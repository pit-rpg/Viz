
#<vertex>
#version 330 core
#<uniforms>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUv;

out vec4 v_color;

void main() {
    vec3 n = aNormal + 1;
    n = normalize(n);
	// v_color = vec4(aNormal+0.5, 1.0);
	// v_color = vec4(n, 1.0);
	v_color =  vec4(n, 1.0) * inverse(transform);
	v_color.z = 1.0;
	gl_Position = transform * vec4(aPos.xyz, 1.0);
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