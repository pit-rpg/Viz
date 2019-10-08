#<vertex>
in vec3 B_Pos;


uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;

out vec3 v_pos;

void main() {
	v_pos = B_Pos;
	gl_Position = vec4(v_pos, 1.0);
}


//////////////////////////////////////////////////////////////////



#<fragment>

#include <snippet-common>
#include <snippet-common-lighting>
#include <snippet-standart>


out vec4 FragColor;

in vec3 v_pos;


#ifdef MAP_COLOR
uniform sampler2D map_color;
#endif

#ifdef TRANSPARENT
uniform float alpha;
#endif


void main()
{
	vec3 diffuseColor = vec3(1.0,0.0,1.0);


#if defined MAP_COLOR
	diffuseColor = vec3(texture(map_color, ( v_pos.xy * 0.5 + 0.5)));
	// #if defined TRANSPARENT
	// 	fragmentAalpha *= texture(map_color, v_uv).a;
	// #endif
#endif

	float lum = (diffuseColor.x+diffuseColor.y+diffuseColor.z)/3.0;

	// FragColor = vec4(diffuseColor.x, 0.0, 0.0, 1.0);
	// FragColor = vec4(diffuseColor.zxy, 1.0);
	FragColor = vec4(lum,lum,lum, 1.0);
	// FragColor = vec4(( v_pos.xy * 0.5 + 0.5), 0.0, 1.0);
}