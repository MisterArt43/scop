#ifndef CAMERA_H
# define CAMERA_H

# include "Math3D.h"

class Camera
{
	public:
		Camera();

		void setAspect(float aspect);
		float getAspect() const;

		void setPosition(const math::Vec3& position);
		math::Vec3 getPosition() const;

		math::Mat4 getViewMatrix() const;
		math::Mat4 getProjectionMatrix() const;

		void moveForward(float amount);
		void moveRight(float amount);
		void moveUp(float amount);
		void rotateYawPitch(float yawDeltaRadians, float pitchDeltaRadians);

		float moveSpeed;
		float lookSpeedRadians;

	private:
		math::Vec3 position;
		math::Vec3 front;
		math::Vec3 worldUp;
		math::Vec3 right;
		math::Vec3 up;

		float yawRadians;
		float pitchRadians;

		float fovRadians;
		float aspect;
		float zNear;
		float zFar;

		void updateAxesFromAngles();
};

#endif
