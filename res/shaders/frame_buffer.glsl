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


uniform sampler2D map_color;

const float offset = 1.0 / 1000.0;

void main()
{
	// vec3 diffuseColor = vec3(1.0,0.0,1.0);
	vec2 TexCoords = ( v_pos.xy * 0.5 + 0.5);

	// vec3 diffuseColor = vec3(texture(map_color, ( v_pos.xy * 0.5 + 0.5)));
	// #if defined TRANSPARENT
	// 	fragmentAalpha *= texture(map_color, v_uv).a;
	// #endif

	  vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right
    );

    float kernel[9] = float[](
        -1, -1, -1,
        -1,  9, -1,
        -1, -1, -1
    );

// float kernel[9] = float[](
//     1.0 / 16, 2.0 / 16, 1.0 / 16,
//     2.0 / 16, 4.0 / 16, 2.0 / 16,
//     1.0 / 16, 2.0 / 16, 1.0 / 16
// );

    vec3 sampleTex[9];

    for(int i = 0; i < 9; i++)
    {
        sampleTex[i] = vec3(texture(map_color, TexCoords.st + offsets[i]));
    }

    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++) {
        col = col + (sampleTex[i] * kernel[i]);
    }

	// vec3 col = vec3(texture(map_color, TexCoords.st));

	// gamma corection
	// col = pow(col, vec3(1.0/3.2));
	// /gamma corection



    FragColor = vec4(col, 1.0);



	// float lum = (diffuseColor.x+diffuseColor.y+diffuseColor.z)/3.0;
	// float lum = 0.2126 * diffuseColor.r + 0.7152 * diffuseColor.g + 0.0722 * diffuseColor.b;

	// lum = dFdx(lum);
	// lum = fwidth(lum);

	// FragColor = vec4(diffuseColor.x, 0.0, 0.0, 1.0);
	// FragColor = vec4(diffuseColor.zxy, 1.0);
	// FragColor = vec4(lum,lum,lum, 1.0);
	// FragColor = vec4(1.0-diffuseColor, 1.0);
	// FragColor = vec4(( v_pos.xy * 0.5 + 0.5), 0.0, 1.0);
}