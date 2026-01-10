#ifndef MATH3D_H
# define MATH3D_H

# include <array>
# include <cmath>

namespace math
{
	inline float clamp(float v, float lo, float hi)
	{
		if (v < lo)
			return lo;
		if (v > hi)
			return hi;
		return v;
	}

	inline float radians(float deg)
	{
		return deg * (3.14159265358979323846f / 180.0f);
	}

	struct Vec2
	{
		float x;
		float y;
	};

	struct Vec3
	{
		float x;
		float y;
		float z;
	};

	struct Vec4
	{
		float x;
		float y;
		float z;
		float w;
	};

	inline Vec2 add(const Vec2& a, const Vec2& b) { return Vec2{a.x + b.x, a.y + b.y}; }
	inline Vec2 sub(const Vec2& a, const Vec2& b) { return Vec2{a.x - b.x, a.y - b.y}; }
	inline Vec2 mul(const Vec2& v, float s) { return Vec2{v.x * s, v.y * s}; }
	inline Vec2 div(const Vec2& v, float s) { return Vec2{v.x / s, v.y / s}; }

	inline Vec3 add(const Vec3& a, const Vec3& b) { return Vec3{a.x + b.x, a.y + b.y, a.z + b.z}; }
	inline Vec3 sub(const Vec3& a, const Vec3& b) { return Vec3{a.x - b.x, a.y - b.y, a.z - b.z}; }
	inline Vec3 mul(const Vec3& v, float s) { return Vec3{v.x * s, v.y * s, v.z * s}; }
	inline Vec3 div(const Vec3& v, float s) { return Vec3{v.x / s, v.y / s, v.z / s}; }

	inline float dot(const Vec3& a, const Vec3& b) { return a.x * b.x + a.y * b.y + a.z * b.z; }

	inline Vec3 cross(const Vec3& a, const Vec3& b)
	{
		return Vec3{
			a.y * b.z - a.z * b.y,
			a.z * b.x - a.x * b.z,
			a.x * b.y - a.y * b.x,
		};
	}

	inline float length(const Vec3& v) { return std::sqrt(dot(v, v)); }

	inline Vec3 normalize(const Vec3& v)
	{
		const float len = length(v);
		if (len <= 0.0f)
			return Vec3{0.0f, 0.0f, 0.0f};
		return mul(v, 1.0f / len);
	}

	struct Mat4
	{
		// Column-major 4x4 matrix (OpenGL-friendly)
		std::array<float, 16> m;
		const float* data() const { return m.data(); }
		float* data() { return m.data(); }
	};

	inline Mat4 identity()
	{
		Mat4 r{};
		r.m = {1.0f, 0.0f, 0.0f, 0.0f,
		       0.0f, 1.0f, 0.0f, 0.0f,
		       0.0f, 0.0f, 1.0f, 0.0f,
		       0.0f, 0.0f, 0.0f, 1.0f};
		return r;
	}

	inline Mat4 mul(const Mat4& a, const Mat4& b)
	{
		Mat4 r{};
		r.m.fill(0.0f);
		// Column-major multiplication: r = a * b
		for (int col = 0; col < 4; ++col)
		{
			for (int row = 0; row < 4; ++row)
			{
				float sum = 0.0f;
				for (int k = 0; k < 4; ++k)
					sum += a.m[k * 4 + row] * b.m[col * 4 + k];
				r.m[col * 4 + row] = sum;
			}
		}
		return r;
	}

	inline Mat4 translate(const Vec3& t)
	{
		Mat4 r = identity();
		r.m[12] = t.x;
		r.m[13] = t.y;
		r.m[14] = t.z;
		return r;
	}

	inline Mat4 scale(const Vec3& s)
	{
		Mat4 r{};
		r.m = {s.x, 0.0f, 0.0f, 0.0f,
		       0.0f, s.y, 0.0f, 0.0f,
		       0.0f, 0.0f, s.z, 0.0f,
		       0.0f, 0.0f, 0.0f, 1.0f};
		return r;
	}

	inline Mat4 rotateAxisAngle(const Vec3& axisIn, float angleRadians)
	{
		const Vec3 axis = normalize(axisIn);
		const float c = std::cos(angleRadians);
		const float s = std::sin(angleRadians);
		const float t = 1.0f - c;

		// Rodrigues' rotation formula (3x3), embedded into 4x4
		Mat4 r = identity();
		r.m[0] = c + axis.x * axis.x * t;
		r.m[1] = axis.y * axis.x * t + axis.z * s;
		r.m[2] = axis.z * axis.x * t - axis.y * s;

		r.m[4] = axis.x * axis.y * t - axis.z * s;
		r.m[5] = c + axis.y * axis.y * t;
		r.m[6] = axis.z * axis.y * t + axis.x * s;

		r.m[8] = axis.x * axis.z * t + axis.y * s;
		r.m[9] = axis.y * axis.z * t - axis.x * s;
		r.m[10] = c + axis.z * axis.z * t;
		return r;
	}

	inline Mat4 perspective(float fovRadians, float aspect, float zNear, float zFar)
	{
		Mat4 r{};
		r.m.fill(0.0f);

		const float safeAspect = (aspect <= 0.0f ? 1.0f : aspect);
		const float tanHalfFovy = std::tan(fovRadians / 2.0f);

		r.m[0] = 1.0f / (safeAspect * tanHalfFovy);
		r.m[5] = 1.0f / tanHalfFovy;
		r.m[10] = -(zFar + zNear) / (zFar - zNear);
		r.m[11] = -1.0f;
		r.m[14] = -(2.0f * zFar * zNear) / (zFar - zNear);
		return r;
	}

	inline Mat4 lookAt(const Vec3& eye, const Vec3& center, const Vec3& upIn)
	{
		const Vec3 f = normalize(sub(center, eye));
		const Vec3 s = normalize(cross(f, upIn));
		const Vec3 u = cross(s, f);

		Mat4 r = identity();
		// Column-major
		r.m[0] = s.x;
		r.m[4] = s.y;
		r.m[8] = s.z;

		r.m[1] = u.x;
		r.m[5] = u.y;
		r.m[9] = u.z;

		r.m[2] = -f.x;
		r.m[6] = -f.y;
		r.m[10] = -f.z;

		r.m[12] = -dot(s, eye);
		r.m[13] = -dot(u, eye);
		r.m[14] = dot(f, eye);
		return r;
	}
}

#endif
