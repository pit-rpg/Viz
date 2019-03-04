#<vertex>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;

out vec3 v_pos;
out vec3 v_normal;
out vec2 v_uv;

void main() {
	v_pos = vec3(matrix_model * vec4(aPos, 1.0));
	v_normal = matrix_normal * aNormal;
	v_uv = aTexCoords;

	gl_Position = matrix_view * vec4(v_pos, 1.0);
}


//////////////////////////////////////////////////////////////////



#<fragment>

#include <snippet-common>
#include <snippet-common-lighting>
#include <snippet-phong>


layout (location = 0) out vec4 FragColor;


in vec3 v_pos;
in vec3 v_normal;
in vec2 v_uv;

uniform vec3 diffuse;
uniform vec3 emissive;
uniform vec3 specular;
uniform float shininess;
uniform float opacity;


// uniform vec3 viewPos;
// uniform Material material;
uniform PointLight pointLights[ NUM_POINT_LIGHTS ];

void main()
{

	PointLight pointLight;
	IncidentLight directLight;
	GeometricContext geometry;

	geometry.position = v_pos;
	geometry.normal = normalize( v_normal );
	geometry.viewDir = normalize( v_pos );

	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );

	BlinnPhongMaterial material;
	material.diffuseColor = diffuse;
	material.specularColor = specular;
	material.specularShininess = shininess;
	material.specularStrength = 1.0;

	#pragma unroll_loop
	for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {

		pointLight = pointLights[ i ];

		getPointDirectLightIrradiance( pointLight, geometry, directLight );

		RE_Direct_BlinnPhong( directLight, geometry, material, reflectedLight );
	}

	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular ;
	FragColor = vec4(outgoingLight, 1.0);

}