#version 330 core
layout (location = 0) in vec2 aPos;

uniform float xpos;
uniform float ypos;
uniform float zpos;

void main() {
   gl_Position = vec4(aPos.x + xpos, aPos.y + ypos, 0.0 + zpos, 1.0);
}