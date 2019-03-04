#define saturate(a) clamp( a, 0.0, 1.0 )
#define RECIPROCAL_PI 0.31830988618

struct Material {
	vec3 diffuse;
	vec3 specular;
	float shininess;
};

struct GeometricContext {
	vec3 position;
	vec3 normal;
	vec3 viewDir;
};
