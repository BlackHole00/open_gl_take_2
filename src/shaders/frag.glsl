#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform sampler2D texture3;

void main() {
    FragColor = mix(mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.4), texture(texture3, TexCoord), 0.5) * vec4(ourColor, 1.0);
}