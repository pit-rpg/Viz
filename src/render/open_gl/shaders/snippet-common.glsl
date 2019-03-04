
#define PI 3.14159265359
#define PI2 6.28318530718
#define PI_HALF 1.5707963267949
#define RECIPROCAL_PI 0.31830988618
#define RECIPROCAL_PI2 0.15915494
#define LOG2 1.442695
#define EPSILON 1e-6


#define saturate(a) clamp( a, 0.0, 1.0 )
float pow2( const in float x ) { return x*x; }
float pow3( const in float x ) { return x*x*x; }
float pow4( const in float x ) { float x2 = x*x; return x2*x2; }

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
