#ifndef OBJ_PARSER_H
#define OBJ_PARSER_H

#include <string>
#include <vector>
#include <cstdint>
#include <unordered_map>

struct Vec2 {
    float x, y;
};

struct Vec3 {
    float x, y, z;
};

struct Pixel {
    int r, g, b;
};

struct Vertex {
    Vec3 position;
    Vec3 normal;
    Vec2 uv;
};

struct MTLMaterial {
    std::string name;
    Vec3 Ka{0.0f, 0.0f, 0.0f};
    Vec3 Kd{0.8f, 0.8f, 0.8f};
    Vec3 Ks{0.0f, 0.0f, 0.0f};
    float Ns{0.0f};
    float d{1.0f};
    int illum{0};
    std::string map_Kd;
    int textureWidth{0};
    int textureHeight{0};
    std::vector<Pixel> textureData;

};

class OBJParser {
public:
    // Charge un fichier .obj et remplit vertices + indices
    bool loadFromFile(const std::string& filepath);

    const std::vector<Vertex>& getVertices() const { return m_vertices; }
    const std::vector<uint32_t>& getIndices() const { return m_indices; }

    const std::unordered_map<std::string, MTLMaterial>& getMaterials() const { return m_materials; }
    const std::string& getActiveMaterialName() const { return m_activeMaterial; }
    const MTLMaterial* getResolvedActiveMaterial() const;
    bool tryGetActiveDiffuse(Vec3& outKd) const;
    const Vec3& getBoundsMin() const { return m_boundsMin; }
    const Vec3& getBoundsMax() const { return m_boundsMax; }
    bool hasUVs() const { return m_hasUVs; }

    void clear();

private:
    // Données brutes OBJ
    std::vector<Vec3> m_positions;
    std::vector<Vec3> m_normals;
    std::vector<Vec2> m_uvs;

	// Matériaux (.mtl)
	std::unordered_map<std::string, MTLMaterial> m_materials;
	std::string m_activeMaterial;
	std::string m_firstUsedMaterial;
    Vec3 m_boundsMin{0.0f, 0.0f, 0.0f};
    Vec3 m_boundsMax{0.0f, 0.0f, 0.0f};
    bool m_hasUVs{false};

    //PPM parser pour les textures
    static std::string directoryOf(const std::string& filepath);

    // Données finales OpenGL
    std::vector<Vertex> m_vertices;
    std::vector<uint32_t> m_indices;

private:
    struct ObjIndex {
        int v;
        int vt;
        int vn;

        bool operator==(const ObjIndex& other) const;
    };

    struct ObjIndexHash {
        size_t operator()(const ObjIndex& k) const;
    };

private:
    ObjIndex parseFaceToken(const std::string& token) const;
    uint32_t getOrCreateVertex(const ObjIndex& idx);

	bool loadMtlFromFile(const std::string& filepath);

    int fixIndex(int idx, int size) const;
};

#endif