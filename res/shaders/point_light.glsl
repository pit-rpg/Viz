struct PointLight {
	vec3 position;
	vec3 color;
	float power;

	float constant;
	float linear;
	float quadratic;

	// vec3 ambient;
	// vec3 diffuse;
	// vec3 specular;
};

#define NR_POINT_LIGHTS 4
uniform PointLight pointLights[NR_POINT_LIGHTS];

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
	vec3 lightDir = normalize(light.position - fragPos);
	// диффузное освещение
	float diff = max(dot(normal, lightDir), 0.0);
	// освещение зеркальных бликов
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
	// затухание
	float distance	= length(light.position - fragPos);
	float attenuation = 1.0 / (light.constant + light.linear * distance +
  				 light.quadratic * (distance * distance));
	// комбинируем результаты
	vec3 ambient  = light.ambient  * vec3(texture(material.diffuse, TexCoords));
	vec3 diffuse  = light.diffuse  * diff * vec3(texture(material.diffuse, TexCoords));
	vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCoords));
	ambient  *= attenuation;
	diffuse  *= attenuation;
	specular *= attenuation;
	return (ambient + diffuse + specular);
}