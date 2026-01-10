#include "../include/Camera.h"

Camera::Camera()
	: moveSpeed(2.5f)
	, lookSpeedRadians(math::radians(90.0f))
	, position{0.0f, 0.0f, 3.0f}
	, front{0.0f, 0.0f, -1.0f}
	, worldUp{0.0f, 1.0f, 0.0f}
	, right{1.0f, 0.0f, 0.0f}
	, up{0.0f, 1.0f, 0.0f}
	, yawRadians(math::radians(-90.0f))
	, pitchRadians(0.0f)
	, fovRadians(math::radians(80.0f))
	, aspect(1.0f)
	, zNear(0.1f)
	, zFar(100.0f)
{
	updateAxesFromAngles();
}

void Camera::updateAxesFromAngles()
{
	// Spherical coordinates: yaw around Y, pitch around X
	front.x = std::cos(yawRadians) * std::cos(pitchRadians);
	front.y = std::sin(pitchRadians);
	front.z = std::sin(yawRadians) * std::cos(pitchRadians);
	front = math::normalize(front);

	right = math::normalize(math::cross(front, worldUp));
	up = math::normalize(math::cross(right, front));
}

void Camera::rotateYawPitch(float yawDeltaRadians, float pitchDeltaRadians)
{
	yawRadians += yawDeltaRadians;
	pitchRadians += pitchDeltaRadians;
	// Prevent gimbal-ish flip near vertical
	const float limit = math::radians(89.0f);
	pitchRadians = math::clamp(pitchRadians, -limit, limit);
	updateAxesFromAngles();
}

void Camera::setAspect(float a)
{
	aspect = math::clamp(a, 0.0001f, 10000.0f);
}

float Camera::getAspect() const { return aspect; }

void Camera::setPosition(const math::Vec3& p) { position = p; }

math::Vec3 Camera::getPosition() const { return position; }

math::Mat4 Camera::getViewMatrix() const
{
	return math::lookAt(position, math::add(position, front), up);
}

math::Mat4 Camera::getProjectionMatrix() const
{
	return math::perspective(fovRadians, aspect, zNear, zFar);
}

void Camera::moveForward(float amount)
{
	position = math::add(position, math::mul(front, amount));
}

void Camera::moveRight(float amount)
{
	position = math::add(position, math::mul(right, amount));
}

void Camera::moveUp(float amount)
{
	position = math::add(position, math::mul(worldUp, amount));
}
