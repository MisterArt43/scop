#ifndef EBO_CLASS_H
#define EBO_CLASS_H

#include <glad/glad.h>

// Element Buffer Object
class EBO
{
    public:
        GLuint ID;
        EBO(GLuint* indices, GLsizeiptr size);

        void Bind();
        void Unbind();
        void Delete();
};

#endif