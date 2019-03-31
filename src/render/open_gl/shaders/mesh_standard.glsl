#<vertex>
in vec3 B_Pos;
in vec3 B_Normal;
#ifdef B_UV
in vec2 B_uv;
#endif
#ifdef B_COLOR_4
in vec4 B_Color;
#endif
#ifdef B_COLOR_3
in vec3 B_Color;
#endif


uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;

out vec3 v_pos;
out vec3 v_normal;
#ifdef B_UV
out vec2 v_uv;
#endif
#ifdef B_COLOR_4
out vec4 v_color;
#endif
#ifdef B_COLOR_3
out vec3 v_color;
#endif


void main() {
	v_pos = vec3(matrix_model * vec4(B_Pos, 1.0));
	v_normal = matrix_normal * B_Normal;

	#ifdef B_UV
	v_uv = B_uv;
	#endif
	#if defined B_COLOR_4 || defined B_COLOR_3
	v_color = B_Color;
	#endif

	gl_Position = matrix_view * vec4(v_pos, 1.0);
}


//////////////////////////////////////////////////////////////////



#<fragment>

#include <snippet-common>
#include <snippet-common-lighting>
#include <snippet-standart>


out vec4 FragColor;

in vec3 v_pos;
in vec3 v_normal;
#ifdef B_UV
in vec2 v_uv;
#endif
#ifdef B_COLOR_4
in vec4 v_color;
#endif
#ifdef B_COLOR_3
in vec3 v_color;
#endif

uniform vec3 diffuse;
uniform vec3 specular;
uniform float roughness;
uniform float metalness;

uniform vec3 ambientLightColor;
// uniform vec3 emissive;
// uniform float opacity;


// uniform vec3 viewPos;
// uniform Material material;
uniform PointLight pointLights[ NUM_POINT_LIGHTS ];

uniform sampler2D map_color;


void main()
{

	IncidentLight directLight;
	GeometricContext geometry;

	geometry.position = v_pos;
	geometry.normal = normalize( v_normal );
	geometry.viewDir = normalize( v_pos );

#if defined B_UV && defined MAP_NORMAL
geometry.normal = normalize(geometry.normal + vec3(texture(map_color, v_uv)));
// geometry.normal *= normalize(vec3(texture(map_color, v_uv)));
#endif


	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );

	// PhysicalMaterial material;
	// material.diffuseColor = diffuse;
	// material.specularColor = specular;
	// material.specularRoughness = roughness;

	float metalnessFactor = metalness;
	float roughnessFactor = roughness;
	// float metalnessFactor = metalness;
	// float roughnessFactor = roughness;
	vec3 diffuseColor = diffuse;

#if defined B_UV && defined MAP_COLOR
diffuseColor = vec3(texture(map_color, v_uv));
#endif

PhysicalMaterial material;
material.diffuseColor = diffuseColor.rgb * ( 1.0 - metalnessFactor );
material.specularRoughness = clamp( roughnessFactor, 0.04, 1.0 );
// #ifdef STANDARD
	material.specularColor = mix( vec3( DEFAULT_SPECULAR_COEFFICIENT ), diffuseColor.rgb, metalnessFactor );
// #else
// 	material.specularColor = mix( vec3( MAXIMUM_SPECULAR_COEFFICIENT * pow2( reflectivity ) ), diffuseColor.rgb, metalnessFactor );
// 	material.clearCoat = saturate( clearCoat ); // Burley clearcoat model
// 	material.clearCoatRoughness = clamp( clearCoatRoughness, 0.04, 1.0 );
// #endif

	#if defined B_COLOR_4 || defined B_COLOR_3
	material.diffuseColor *= v_color.xyz;
	#endif


	#if ( NUM_POINT_LIGHTS > 0 )
		PointLight pointLight;

		#pragma unroll_loop
		for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {

			pointLight = pointLights[ i ];

			getPointDirectLightIrradiance( pointLight, geometry, directLight );

			// #ifdef USE_SHADOWMAP
			// directLight.color *= all( bvec2( pointLight.shadow, directLight.visible ) ) ? getPointShadow( pointShadowMap[ i ], pointLight.shadowMapSize, pointLight.shadowBias, pointLight.shadowRadius, vPointShadowCoord[ i ], pointLight.shadowCameraNear, pointLight.shadowCameraFar ) : 1.0;
			// #endif

			RE_Direct_Physical( directLight, geometry, material, reflectedLight );

		}
	#endif






// #if defined( RE_IndirectDiffuse )
	vec3 irradiance = getAmbientLightIrradiance( ambientLightColor );
	// #if ( NUM_HEMI_LIGHTS > 0 )
	// 	#pragma unroll_loop
	// 	for ( int i = 0; i < NUM_HEMI_LIGHTS; i ++ ) {

	// 		irradiance += getHemisphereLightIrradiance( hemisphereLights[ i ], geometry );

	// 	}
	// #endif
// #endif



	// #if defined( RE_IndirectDiffuse )
	RE_IndirectDiffuse_Physical( irradiance, geometry, material, reflectedLight );
	// #endif


	vec3 radiance = vec3( 0.0 );
	vec3 clearCoatRadiance = vec3( 0.0 );

	// #if defined( RE_IndirectSpecular )
	RE_IndirectSpecular_Physical( radiance, clearCoatRadiance, geometry, material, reflectedLight );
	// #endif





	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular ;
	FragColor = vec4(outgoingLight, 1.0);

}