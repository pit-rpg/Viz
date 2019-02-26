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
layout (location = 0) out vec4 FragColor;

struct Material {
	vec3 diffuse;
	vec3 specular;
	float shininess;
};

struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};

struct PointLight {
	vec3 position;
	vec3 color;
	float distance;
	float decay;
};

struct GeometricContext {
	vec3 position;
	vec3 normal;
	vec3 viewDir;
};

struct BlinnPhongMaterial {

	vec3	diffuseColor;
	vec3	specularColor;
	float	specularShininess;
	float	specularStrength;

};

struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};



#define saturate(a) clamp( a, 0.0, 1.0 )
#define RECIPROCAL_PI 0.31830988618


float punctualLightIntensityToIrradianceFactor( const in float lightDistance, const in float cutoffDistance, const in float decayExponent ) {
	if( cutoffDistance > 0.0 ) {
		return pow( saturate( -lightDistance / cutoffDistance + 1.0 ), decayExponent );
	}
	return 1.0;
}


// directLight is an out parameter as having it as a return value caused compiler errors on some devices
void getPointDirectLightIrradiance( const in PointLight pointLight, const in GeometricContext geometry, out IncidentLight directLight ) {

	vec3 lVector = pointLight.position - geometry.position;
	directLight.direction = normalize( lVector );

	float lightDistance = length( lVector );

	directLight.color = pointLight.color;
	directLight.color *= punctualLightIntensityToIrradianceFactor( lightDistance, pointLight.distance, pointLight.decay );
	directLight.visible = ( directLight.color != vec3( 0.0 ) );

}


vec3 BRDF_Diffuse_Lambert( const in vec3 diffuseColor ) {
	return RECIPROCAL_PI * diffuseColor;
} // validated


vec3 F_Schlick( const in vec3 specularColor, const in float dotLH ) {
	// Original approximation by Christophe Schlick '94
	// float fresnel = pow( 1.0 - dotLH, 5.0 );
	// Optimized variant (presented by Epic at SIGGRAPH '13)
	// https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf
	float fresnel = exp2( ( -5.55473 * dotLH - 6.98316 ) * dotLH );
	return ( 1.0 - specularColor ) * fresnel + specularColor;
}

float G_BlinnPhong_Implicit( /* const in float dotNL, const in float dotNV */ ) {
	// geometry term is (n dot l)(n dot v) / 4(n dot l)(n dot v)
	return 0.25;
} // validated

float D_BlinnPhong( const in float shininess, const in float dotNH ) {
	return RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dotNH, shininess );
}


vec3 BRDF_Specular_BlinnPhong( const in IncidentLight incidentLight, const in GeometricContext geometry, const in vec3 specularColor, const in float shininess ) {
	vec3 halfDir = normalize( incidentLight.direction + geometry.viewDir );
	//float dotNL = saturate( dot( geometry.normal, incidentLight.direction ) );
	//float dotNV = saturate( dot( geometry.normal, geometry.viewDir ) );
	float dotNH = saturate( dot( geometry.normal, halfDir ) );
	float dotLH = saturate( dot( incidentLight.direction, halfDir ) );
	vec3 F = F_Schlick( specularColor, dotLH );
	float G = G_BlinnPhong_Implicit( /* dotNL, dotNV */ );
	float D = D_BlinnPhong( shininess, dotNH );
	return F * ( G * D );
} // validated


void RE_Direct_BlinnPhong( const in IncidentLight directLight, const in GeometricContext geometry, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {

	// #ifdef TOON
	// 	vec3 irradiance = getGradientIrradiance( geometry.normal, directLight.direction ) * directLight.color;
	// #else
		float dotNL = saturate( dot( geometry.normal, directLight.direction ) );
		vec3 irradiance = dotNL * directLight.color;
	// #endif

	// #ifndef PHYSICALLY_CORRECT_LIGHTS
	// 	irradiance *= PI; // punctual light
	// #endif

	reflectedLight.directDiffuse += irradiance * BRDF_Diffuse_Lambert( material.diffuseColor );
	reflectedLight.directSpecular += irradiance * BRDF_Specular_BlinnPhong( directLight, geometry, material.specularColor, material.specularShininess ) * material.specularStrength;
}


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
	geometry.normal = v_normal;
	geometry.viewDir = normalize( v_pos );

	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );


	BlinnPhongMaterial material;
	material.diffuseColor = diffuse;
	material.specularColor = specular;
	material.specularShininess = shininess;
	material.specularStrength = 1.0;

	// BlinnPhongMaterial material;
	// material.diffuseColor = diffuseColor.rgb;
	// material.specularColor = specular;
	// material.specularShininess = shininess;
	// material.specularStrength = specularStrength;



	#pragma unroll_loop
	for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {

		pointLight = pointLights[ i ];

		getPointDirectLightIrradiance( pointLight, geometry, directLight );



		RE_Direct_BlinnPhong( directLight, geometry, material, reflectedLight );
	}

	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular ;
	// vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular + totalEmissiveRadiance;

	// float d = punctualLightIntensityToIrradianceFactor(3, 10, 1.0);
	// FragColor = vec4(d, d, d, 1.0);

	FragColor = vec4(outgoingLight, 1.0);


	// // ambient
	// vec3 ambient = light.ambient * texture(material.diffuse, TexCoords).rgb;

	// // diffuse
	// vec3 norm = normalize(Normal);
	// vec3 lightDir = normalize(light.position - FragPos);
	// float diff = max(dot(norm, lightDir), 0.0);
	// vec3 diffuse = light.diffuse * diff * texture(material.diffuse, TexCoords).rgb;

	// // specular
	// vec3 viewDir = normalize(viewPos - FragPos);
	// vec3 reflectDir = reflect(-lightDir, norm);
	// float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
	// vec3 specular = light.specular * spec * texture(material.specular, TexCoords).rgb;

	// // attenuation
	// float distance	= length(light.position - FragPos);
	// float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

	// ambient  *= attenuation;
	// diffuse   *= attenuation;
	// specular *= attenuation;

	// vec3 result = ambient + diffuse + specular;
	// FragColor = vec4(result, 1.0);
}