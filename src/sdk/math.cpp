#include "sdk/math.hpp"

#include <algorithm>
#include <cmath>

#include "interfaces/interfaces.hpp"

// fsqrt polyfill if it isn't defined
#ifndef fsqrt
#define fsqrt sqrtf
#endif

Vector::Vector(float scalar) { x = y = z = scalar; }

Vector::Vector(const Vector &other) {
    x = other.x;
    y = other.y;
    z = other.z;
}

Vector::Vector(float x, float y, float z) {
    this->x = x;
    this->y = y;
    this->z = z;
}

Vector &Vector::operator+=(const Vector &other) {
    x += other.x;
    y += other.y;
    z += other.z;
    return *this;
}

Vector &Vector::operator-=(const Vector &other) {
    x -= other.x;
    y -= other.y;
    z -= other.z;
    return *this;
}

Vector &Vector::operator*=(const Vector &other) {
    x *= other.x;
    y *= other.y;
    z *= other.z;
    return *this;
}

Vector &Vector::operator/=(const Vector &other) {
    x /= other.x;
    y /= other.y;
    z /= other.z;
    return *this;
}

Vector &Vector::operator+=(float other) {
    x += other;
    y += other;
    z += other;
    return *this;
}

Vector &Vector::operator-=(float other) {
    x -= other;
    y -= other;
    z -= other;
    return *this;
}

Vector &Vector::operator*=(float other) {
    x *= other;
    y *= other;
    z *= other;
    return *this;
}

Vector &Vector::operator/=(float other) {
    x /= other;
    y /= other;
    z /= other;
    return *this;
}

bool Vector::operator==(const Vector &other) const {
    return x == other.x && y == other.y && z == other.z;
}

bool Vector::operator!=(const Vector &other) const {
    return x != other.x || y != other.y || z != other.z;
}

Vector Vector::operator-() const { return {-x, -y, -z}; }

Vector Vector::operator+(const Vector &other) const {
    return {x + other.x, y + other.y, z + other.z};
}

Vector Vector::operator-(const Vector &other) const {
    return {x - other.x, y - other.y, z - other.z};
}

Vector Vector::operator*(const Vector &other) const {
    return {x * other.x, y * other.y, z * other.z};
}

Vector Vector::operator/(const Vector &other) const {
    return {x / other.x, y / other.y, z / other.z};
}

Vector Vector::operator+(float other) const { return {x + other, y + other, z + other}; }

Vector Vector::operator-(float other) const { return {x - other, y - other, z - other}; }

Vector Vector::operator*(float other) const { return {x * other, y * other, z * other}; }

Vector Vector::operator/(float other) const { return {x / other, y / other, z / other}; }

float Vector::HorizontalLengthSquared() const { return x * x + y * y; }

float Vector::HorizontalLength() const { return fsqrt(HorizontalLengthSquared()); }

float Vector::LengthSquared() const { return x * x + y * y + z * z; }

float Vector::Length() const { return fsqrt(LengthSquared()); }

float Vector::Dot(const Vector &other) const {
    return this->x * other.x + this->y * other.y + this->z * other.z;
}

Vector Vector::Normalized() const {
    const float len = Length();
    return {this->x / len, this->y / len, this->z / len};
}

bool Vector::IsZero() const { return x == 0 && y == 0 && z == 0; }

Vector Vector::Wrap() {
    this->x = std::remainderf(this->x, 360.0f);
    this->x = std::clamp(
        this->x, -Interfaces::convar->Get("cl_pitchup")->GetFloat(),
        Interfaces::convar->Get("cl_pitchdown")->GetFloat());

    this->y = std::remainderf(this->y, 360.0f);

    this->z = 0.0f;

    return *this;
}

const float *Matrix4x4::operator[](int i) const { return matrix[i]; }

const float *Matrix4x4::Base() const { return &matrix[0][0]; }

float *Matrix3x4::operator[](int i) { return matrix[i]; }

float *Matrix3x4::Base() { return &matrix[0][0]; }

Vector Matrix3x4::Origin() const { return {matrix[0][3], matrix[1][3], matrix[2][3]}; }
