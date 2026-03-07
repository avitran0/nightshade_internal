#pragma once

struct Vector {
    float x, y, z;

    explicit Vector(float scalar = 0.0f);
    Vector(const Vector &other);
    Vector(float x, float y, float z);

    Vector &operator=(const Vector &other) = default;

    Vector &operator+=(const Vector &other);
    Vector &operator-=(const Vector &other);
    Vector &operator*=(const Vector &other);
    Vector &operator/=(const Vector &other);

    Vector &operator+=(float other);
    Vector &operator-=(float other);
    Vector &operator*=(float other);
    Vector &operator/=(float other);

    bool operator==(const Vector &other) const;
    bool operator!=(const Vector &other) const;

    Vector operator-() const;

    Vector operator+(const Vector &other) const;
    Vector operator-(const Vector &other) const;
    Vector operator*(const Vector &other) const;
    Vector operator/(const Vector &other) const;

    Vector operator+(float other) const;
    Vector operator-(float other) const;
    Vector operator*(float other) const;
    Vector operator/(float other) const;

    [[nodiscard]] float HorizontalLengthSquared() const;
    [[nodiscard]] float HorizontalLength() const;
    [[nodiscard]] float LengthSquared() const;
    [[nodiscard]] float Length() const;
    [[nodiscard]] float Dot(const Vector &other) const;
    [[nodiscard]] Vector Normalized() const;
    [[nodiscard]] bool IsZero() const;

    Vector Wrap();
};

using QAngle = Vector;

struct Matrix4x4 {
    float matrix[4][4];

    const float *operator[](int i) const;
    [[nodiscard]] const float *Base() const;
};

struct Matrix3x4 {
    float matrix[3][4];

    float *operator[](int i);
    float *Base();
    [[nodiscard]] Vector Origin() const;
};
