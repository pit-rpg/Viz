
struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};

struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};

struct PointLight {
	vec3 position;
	vec3 color;
	float distance;
	float decay;
};

struct DirectionalLight {
	vec3 direction;
	vec3 color;

	// int shadow;
	// float shadowBias;
	// float shadowRadius;
	// vec2 shadowMapSize;
};


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


vec3 getAmbientLightIrradiance( const in vec3 ambient_light ) {

	vec3 irradiance = ambient_light;

	// #ifndef PHYSICALLY_CORRECT_LIGHTS
	// 	irradiance *= PI;
	// #endif

	return irradiance;

}

void getDirectionalDirectLightIrradiance( const in DirectionalLight directionalLight, const in GeometricContext geometry, out IncidentLight directLight ) {
	directLight.color = directionalLight.color;
	directLight.direction = directionalLight.direction;
	directLight.visible = true;
}