

#define MAXIMUM_SPECULAR_COEFFICIENT 0.16
#define DEFAULT_SPECULAR_COEFFICIENT 0.04


struct PhysicalMaterial {

	vec3	diffuseColor;
	float	specularRoughness;
	vec3	specularColor;

	// #ifndef STANDARD
	// 	float clearCoat;
	// 	float clearCoatRoughness;
	// #endif
};


vec3 F_Schlick( const in vec3 specularColor, const in float dotLH ) {

	// Original approximation by Christophe Schlick '94
	// float fresnel = pow( 1.0 - dotLH, 5.0 );

	// Optimized variant (presented by Epic at SIGGRAPH '13)
	// https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf
	float fresnel = exp2( ( -5.55473 * dotLH - 6.98316 ) * dotLH );

	return ( 1.0 - specularColor ) * fresnel + specularColor;

} // validated


// Moving Frostbite to Physically Based Rendering 3.0 - page 12, listing 2
// https://seblagarde.files.wordpress.com/2015/07/course_notes_moving_frostbite_to_pbr_v32.pdf
float G_GGX_SmithCorrelated( const in float alpha, const in float dotNL, const in float dotNV ) {

	float a2 = pow2( alpha );

	// dotNL and dotNV are explicitly swapped. This is not a mistake.
	float gv = dotNL * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNV ) );
	float gl = dotNV * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNL ) );

	return 0.5 / max( gv + gl, EPSILON );

}



// Microfacet Models for Refraction through Rough Surfaces - equation (33)
// http://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
// alpha is "roughness squared" in Disney’s reparameterization
float D_GGX( const in float alpha, const in float dotNH ) {

	float a2 = pow2( alpha );

	float denom = pow2( dotNH ) * ( a2 - 1.0 ) + 1.0; // avoid alpha = 0 with dotNH = 1

	return RECIPROCAL_PI * a2 / pow2( denom );

}


// GGX Distribution, Schlick Fresnel, GGX-Smith Visibility
vec3 BRDF_Specular_GGX( const in IncidentLight incidentLight, const in GeometricContext geometry, const in vec3 specularColor, const in float roughness ) {

	float alpha = pow2( roughness ); // UE4's roughness

	vec3 halfDir = normalize( incidentLight.direction + geometry.viewDir );

	float dotNL = saturate( dot( geometry.normal, incidentLight.direction ) );
	float dotNV = saturate( dot( geometry.normal, geometry.viewDir ) );
	float dotNH = saturate( dot( geometry.normal, halfDir ) );
	float dotLH = saturate( dot( incidentLight.direction, halfDir ) );

	vec3 F = F_Schlick( specularColor, dotLH );

	float G = G_GGX_SmithCorrelated( alpha, dotNL, dotNV );

	float D = D_GGX( alpha, dotNH );

	return F * ( G * D );

} // validated


vec3 BRDF_Diffuse_Lambert( const in vec3 diffuseColor ) {
	return RECIPROCAL_PI * diffuseColor;
} // validated


void RE_Direct_Physical( const in IncidentLight directLight, const in GeometricContext geometry, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {

	float dotNL = saturate( dot( geometry.normal, directLight.direction ) );

	vec3 irradiance = dotNL * directLight.color;

	// #ifndef PHYSICALLY_CORRECT_LIGHTS
		// irradiance *= PI; // punctual light
	// #endif

	// #ifndef STANDARD
		// float clearCoatDHR = material.clearCoat * clearCoatDHRApprox( material.clearCoatRoughness, dotNL );
	// #else
		float clearCoatDHR = 0.0;
	// #endif

	reflectedLight.directSpecular += ( 1.0 - clearCoatDHR ) * irradiance * BRDF_Specular_GGX( directLight, geometry, material.specularColor, material.specularRoughness );

	reflectedLight.directDiffuse += ( 1.0 - clearCoatDHR ) * irradiance * BRDF_Diffuse_Lambert( material.diffuseColor );

	// #ifndef STANDARD

	// 	reflectedLight.directSpecular += irradiance * material.clearCoat * BRDF_Specular_GGX( directLight, geometry, vec3( DEFAULT_SPECULAR_COEFFICIENT ), material.clearCoatRoughness );

	// #endif

}


void RE_IndirectDiffuse_Physical( const in vec3 irradiance, const in GeometricContext geometry, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Diffuse_Lambert( material.diffuseColor );
}


// ref: https://www.unrealengine.com/blog/physically-based-shading-on-mobile - environmentBRDF for GGX on mobile
vec3 BRDF_Specular_GGX_Environment( const in GeometricContext geometry, const in vec3 specularColor, const in float roughness ) {

	float dotNV = saturate( dot( geometry.normal, geometry.viewDir ) );

	const vec4 c0 = vec4( - 1, - 0.0275, - 0.572, 0.022 );

	const vec4 c1 = vec4( 1, 0.0425, 1.04, - 0.04 );

	vec4 r = roughness * c0 + c1;

	float a004 = min( r.x * r.x, exp2( - 9.28 * dotNV ) ) * r.x + r.y;

	vec2 AB = vec2( -1.04, 1.04 ) * a004 + r.zw;

	return specularColor * AB.x + AB.y;

} // validated




void RE_IndirectSpecular_Physical( const in vec3 radiance, const in vec3 clearCoatRadiance, const in GeometricContext geometry, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {

	// #ifndef STANDARD
	// 	float dotNV = saturate( dot( geometry.normal, geometry.viewDir ) );
	// 	float dotNL = dotNV;
	// 	float clearCoatDHR = material.clearCoat * clearCoatDHRApprox( material.clearCoatRoughness, dotNL );
	// #else
		float clearCoatDHR = 0.0;
	// #endif

	reflectedLight.indirectSpecular += ( 1.0 - clearCoatDHR ) * radiance * BRDF_Specular_GGX_Environment( geometry, material.specularColor, material.specularRoughness );

	// #ifndef STANDARD

	// 	reflectedLight.indirectSpecular += clearCoatRadiance * material.clearCoat * BRDF_Specular_GGX_Environment( geometry, vec3( DEFAULT_SPECULAR_COEFFICIENT ), material.clearCoatRoughness );

	// #endif

}


// float punctualLightIntensityToIrradianceFactor( const in float lightDistance, const in float cutoffDistance, const in float decayExponent ) {
// 	if( cutoffDistance > 0.0 ) {
// 		return pow( saturate( -lightDistance / cutoffDistance + 1.0 ), decayExponent );
// 	}
// 	return 1.0;
// }


// // directLight is an out parameter as having it as a return value caused compiler errors on some devices
// void getPointDirectLightIrradiance( const in PointLight pointLight, const in GeometricContext geometry, out IncidentLight directLight ) {

// 	vec3 lVector = pointLight.position - geometry.position;
// 	directLight.direction = normalize( lVector );

// 	float lightDistance = length( lVector );

// 	directLight.color = pointLight.color;
// 	directLight.color *= punctualLightIntensityToIrradianceFactor( lightDistance, pointLight.distance, pointLight.decay );
// 	directLight.visible = ( directLight.color != vec3( 0.0 ) );

// }


// vec3 BRDF_Diffuse_Lambert( const in vec3 diffuseColor ) {
// 	return RECIPROCAL_PI * diffuseColor;
// } // validated


// vec3 F_Schlick( const in vec3 specularColor, const in float dotLH ) {
// 	// Original approximation by Christophe Schlick '94
// 	// float fresnel = pow( 1.0 - dotLH, 5.0 );
// 	// Optimized variant (presented by Epic at SIGGRAPH '13)
// 	// https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf
// 	float fresnel = exp2( ( -5.55473 * dotLH - 6.98316 ) * dotLH );
// 	return ( 1.0 - specularColor ) * fresnel + specularColor;
// }

// float G_BlinnPhong_Implicit( /* const in float dotNL, const in float dotNV */ ) {
// 	// geometry term is (n dot l)(n dot v) / 4(n dot l)(n dot v)
// 	return 0.25;
// } // validated

// float D_BlinnPhong( const in float shininess, const in float dotNH ) {
// 	return RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dotNH, shininess );
// }


// vec3 BRDF_Specular_BlinnPhong( const in IncidentLight incidentLight, const in GeometricContext geometry, const in vec3 specularColor, const in float shininess ) {
// 	vec3 halfDir = normalize( incidentLight.direction + geometry.viewDir );
// 	//float dotNL = saturate( dot( geometry.normal, incidentLight.direction ) );
// 	//float dotNV = saturate( dot( geometry.normal, geometry.viewDir ) );
// 	float dotNH = saturate( dot( geometry.normal, halfDir ) );
// 	float dotLH = saturate( dot( incidentLight.direction, halfDir ) );
// 	vec3 F = F_Schlick( specularColor, dotLH );
// 	float G = G_BlinnPhong_Implicit( /* dotNL, dotNV */ );
// 	float D = D_BlinnPhong( shininess, dotNH );
// 	return F * ( G * D );
// } // validated


// void RE_Direct_BlinnPhong( const in IncidentLight directLight, const in GeometricContext geometry, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {

// 	// #ifdef TOON
// 	// 	vec3 irradiance = getGradientIrradiance( geometry.normal, directLight.direction ) * directLight.color;
// 	// #else
// 		float dotNL = saturate( dot( geometry.normal, directLight.direction ) );
// 		vec3 irradiance = dotNL * directLight.color;
// 	// #endif

// 	// #ifndef PHYSICALLY_CORRECT_LIGHTS
// 	// 	irradiance *= PI; // punctual light
// 	// #endif

// 	reflectedLight.directDiffuse += irradiance * BRDF_Diffuse_Lambert( material.diffuseColor );
// 	reflectedLight.directSpecular += irradiance * BRDF_Specular_BlinnPhong( directLight, geometry, material.specularColor, material.specularShininess ) * material.specular_strength;
// }
