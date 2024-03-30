#version 330 core

uniform vec4 u_Color;

in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;


void main() {
    float ambientStrength = 0.9;
    vec3 ambient = 1.5 * u_Color.rgb;

    vec3 result = ambient;
    FragColor = vec4(result, 1.0);
}
