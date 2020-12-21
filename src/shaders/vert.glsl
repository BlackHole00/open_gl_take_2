#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform float position_x;
uniform float position_y;

void main() {
   gl_Position = vec4(aPos.x + position_x, aPos.y + position_y, 0.0, 1.0);
   TexCoord = aTexCoord;
}