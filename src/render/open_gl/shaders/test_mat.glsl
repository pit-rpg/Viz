#<vertex>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUv;
uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;
uniform float time;

out vec3 n;
out vec3 p;

void main() {
	n = matrix_normal * aNormal;
	p = (matrix_model * vec4(aPos.xyz, 1.0)).xyz;

	float x = cos(-time*3 + p.y);
	x = x * cos( p.y * 20 - time * 8 );
	x = x * cos( p.x * 20 - time * 8 );

	vec4 xvec = vec4(1 + 1*x, 1,1,1); 

	vec4 p = matrix_view * matrix_model * vec4(aPos.xyz, 1.0);

	xvec = normalize(p * xvec) *0.2; 

	gl_Position = p + xvec;
}



#<fragment>
uniform sampler2D texture_color;
uniform float time;

in vec3 n;
in vec3 p;

layout (location = 0) out vec4 FragColor;

#include <lololo>

void main() {
	vec3 x = normalize(n) * 0.5 + 0.5;
	// vec3 base = texture2D( texture_color, x ).rgb;

	vec3 color = vec3( x.xy, 0.0 ); 

	vec3 rim = (1.0 - x.z) * (1.0 - x.z) * 4 * vec3(1,1,1);

	vec3 c = min(color - rim, vec3(1,1,1));

	// c.z = sin(time*4);
	c.z = cos(-time*3 + p.y);
	c.z = c.z * cos( p.y * 20 - time * 8 );

	c.z = c.z * cos( p.x * 20 - time * 8 );



	FragColor = vec4( c, 1.0 );
	// FragColor = vec4( c, 1.0 );
	// FragColor = vec4( base, 1. );
}