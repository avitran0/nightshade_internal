#pragma once

#include <cmath>

#include "interfaces/interfaces.hpp"

struct Matrix {
    float matrix[4][4];

    inline const float *operator[](int i) const { return matrix[i]; }

    [[nodiscard]] inline const float *Base() const { return &matrix[0][0]; }
};

struct Vector {
    float x, y, z;

    inline explicit Vector(float scalar = 0.0f) { x = y = z = scalar; }

    inline Vector(const Vector &other) {
        x = other.x;
        y = other.y;
        z = other.z;
    }

    inline Vector(float x, float y, float z) {
        this->x = x;
        this->y = y;
        this->z = z;
    }

    inline Vector &operator=(const Vector &other) = default;

    inline Vector &operator+=(const Vector &other) {
        x += other.x;
        y += other.y;
        z += other.z;
        return *this;
    }

    inline Vector &operator-=(const Vector &other) {
        x -= other.x;
        y -= other.y;
        z -= other.z;
        return *this;
    }

    inline Vector &operator*=(const Vector &other) {
        x *= other.x;
        y *= other.y;
        z *= other.z;
        return *this;
    }

    inline Vector &operator/=(const Vector &other) {
        x /= other.x;
        y /= other.y;
        z /= other.z;
        return *this;
    }

    inline Vector &operator+=(float other) {
        x += other;
        y += other;
        z += other;
        return *this;
    }

    inline Vector &operator-=(float other) {
        x -= other;
        y -= other;
        z -= other;
        return *this;
    }

    inline Vector &operator*=(float other) {
        x *= other;
        y *= other;
        z *= other;
        return *this;
    }

    inline Vector &operator/=(float other) {
        x /= other;
        y /= other;
        z /= other;
        return *this;
    }

    inline bool operator==(const Vector &other) const {
        return x == other.x && y == other.y && z == other.z;
    }

    inline bool operator!=(const Vector &other) const {
        return x != other.x || y != other.y || z != other.z;
    }

    inline Vector operator-() const { return {-x, -y, -z}; }

    inline Vector operator+(const Vector &other) const {
        return {x + other.x, y + other.y, z + other.z};
    }

    inline Vector operator-(const Vector &other) const {
        return {x - other.x, y - other.y, z - other.z};
    }

    inline Vector operator*(const Vector &other) const {
        return {x * other.x, y * other.y, z * other.z};
    }

    inline Vector operator/(const Vector &other) const {
        return {x / other.x, y / other.y, z / other.z};
    }

    inline Vector operator+(float other) const { return {x + other, y + other, z + other}; }

    inline Vector operator-(float other) const { return {x - other, y - other, z - other}; }

    inline Vector operator*(float other) const { return {x * other, y * other, z * other}; }

    inline Vector operator/(float other) const { return {x / other, y / other, z / other}; }

    [[nodiscard]] inline float HorizontalLengthSquared() const { return x * x + y * y; }

    [[nodiscard]] inline float HorizontalLength() const { return fsqrt(HorizontalLengthSquared()); }

    [[nodiscard]] inline float LengthSquared() const { return x * x + y * y + z * z; }

    [[nodiscard]] inline float Length() const { return fsqrt(LengthSquared()); }

    [[nodiscard]] inline float Dot(const Vector &other) const {
        return this->x * other.x + this->y * other.y + this->z * other.z;
    }

    [[nodiscard]] inline Vector Normalized() const {
        const float len = Length();
        return {this->x / len, this->y / len, this->z / len};
    }

    [[nodiscard]] inline bool IsZero() const { return x == 0 && y == 0 && z == 0; }

    inline Vector Wrap() {
        this->x = std::remainderf(this->x, 360.0f);
        this->x = std::clamp(
            this->x, -Interfaces::convar->Get("cl_pitchup")->GetFloat(),
            Interfaces::convar->Get("cl_pitchdown")->GetFloat());

        this->y = std::remainderf(this->y, 360.0f);

        this->z = 0.0f;

        return *this;
    }
};
