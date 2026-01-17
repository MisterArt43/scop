#include "../include/OBJParser.h"

#include <fstream>
#include <sstream>
#include <unordered_map>
#include <iostream>

static bool readPpmToken(std::istream& in, std::string& outTok)
{
    while (in >> outTok)
    {
        if (!outTok.empty() && outTok[0] == '#')
        {
            std::string rest;
            std::getline(in, rest);
            continue;
        }
        return true;
    }
    return false;
}

static std::string ltrim(std::string s)
{
    s.erase(0, s.find_first_not_of(" \t\r\n"));
    return s;
}

// opérateur d'égalité pour ObjIndex
bool OBJParser::ObjIndex::operator==(const ObjIndex& other) const {
    return v == other.v && vt == other.vt && vn == other.vn;
}

// fonction de hachage pour ObjIndex pour utilisation dans unordered_map
size_t OBJParser::ObjIndexHash::operator()(const ObjIndex& k) const {
    return ((k.v * 73856093) ^
            (k.vt * 19349663) ^
            (k.vn * 83492791));
}

// Vide toutes les données chargées
void OBJParser::clear() {
    m_positions.clear();
    m_normals.clear();
    m_uvs.clear();
    m_vertices.clear();
    m_indices.clear();
    m_materials.clear();
    m_activeMaterial.clear();
    m_firstUsedMaterial.clear();
    m_boundsMin = math::Vec3{0.0f, 0.0f, 0.0f};
    m_boundsMax = math::Vec3{0.0f, 0.0f, 0.0f};
    m_hasUVs = false;
}

std::string OBJParser::directoryOf(const std::string& filepath)
{
    const std::string::size_type slash = filepath.find_last_of("/\\");
    if (slash == std::string::npos)
        return std::string(".");
    if (slash == 0)
        return std::string("/");
    return filepath.substr(0, slash);
}

bool loadPPM(const std::string& filepath, std::vector<Pixel>& image, int& width, int& height) {
    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << "Impossible d'ouvrir le fichier PPM : " << filepath << std::endl;
        return false;
    }

    std::string tok;
    if (!readPpmToken(file, tok) || tok != "P3")
    {
        std::cerr << "Format PPM invalide. Attendu P3." << std::endl;
        return false;
    }

    width = 0;
    height = 0;
    int maxColorValue = 0;
    if (!readPpmToken(file, tok)) return false;
    width = std::stoi(tok);
    if (!readPpmToken(file, tok)) return false;
    height = std::stoi(tok);
    if (!readPpmToken(file, tok)) return false;
    maxColorValue = std::stoi(tok);
    if (width <= 0 || height <= 0 || maxColorValue <= 0)
    {
        std::cerr << "PPM header invalide: " << filepath << std::endl;
        return false;
    }

    image.clear();
    image.resize(static_cast<size_t>(width) * static_cast<size_t>(height));
    for (int i = 0; i < width * height; ++i)
    {
        int r = 0, g = 0, b = 0;
        if (!readPpmToken(file, tok)) return false;
        r = std::stoi(tok);
        if (!readPpmToken(file, tok)) return false;
        g = std::stoi(tok);
        if (!readPpmToken(file, tok)) return false;
        b = std::stoi(tok);

        // Scale to 0..255 if maxColorValue differs
        if (maxColorValue != 255)
        {
            r = (r * 255) / maxColorValue;
            g = (g * 255) / maxColorValue;
            b = (b * 255) / maxColorValue;
        }
        image[i].r = r;
        image[i].g = g;
        image[i].b = b;
    }

    return true;
}


bool OBJParser::loadMtlFromFile(const std::string& filepath)
{
    std::cout << "DEBUG 1 /////////////////////////////" << std::endl;
    std::ifstream file(filepath);
    if (!file.is_open())
    {
        std::cerr << "OBJParser: impossible d'ouvrir MTL " << filepath << "\n";
        return false;
    }

    const std::string baseDir = directoryOf(filepath);

    MTLMaterial current;
    bool hasCurrent = false;
    std::string line;
    while (std::getline(file, line))
    {
        line = ltrim(line);
        if (line.empty() || line[0] == '#')
            continue;

        std::istringstream iss(line);
        std::string key;
        iss >> key;

        if (key == "newmtl")
        {
            if (hasCurrent && !current.name.empty())
                m_materials[current.name] = current;
            current = MTLMaterial{};
            hasCurrent = true;
            iss >> current.name;
        }
        else if (!hasCurrent)
        {
            continue;
        }
        else if (key == "Ka")
        {
            iss >> current.Ka.x >> current.Ka.y >> current.Ka.z;
        }
        else if (key == "Kd")
        {
            iss >> current.Kd.x >> current.Kd.y >> current.Kd.z;
        }
        else if (key == "Ks")
        {
            iss >> current.Ks.x >> current.Ks.y >> current.Ks.z;
        }
        else if (key == "Ns")
        {
            iss >> current.Ns;
        }
        else if (key == "d")
        {
            iss >> current.d;
        }
        else if (key == "Tr")
        {
            float tr = 0.0f;
            iss >> tr;
            current.d = 1.0f - tr;
        }
        else if (key == "illum")
        {
            iss >> current.illum;
        }
        else if (key == "map_Kd")
        {
            std::cout << "Loading texture map_Kd for material " << current.name << "\n";
            std::cout << "DEBUG /////////////////////////////" << std::endl;
            
            std::string textureFile;
            std::getline(iss, textureFile);
            textureFile = ltrim(textureFile);
            current.map_Kd = textureFile;

            std::string texturePath = textureFile;
            if (!texturePath.empty() && texturePath[0] != '/')
                texturePath = baseDir + "/" + texturePath;

            // Charger la texture PPM
            std::vector<Pixel> image;
            int width, height;
            if (loadPPM(texturePath, image, width, height))
            {
                // Sauvegarder l'image dans le matériau
                current.textureWidth = width;
                current.textureHeight = height;
                current.textureData = image;
            }
            else
            {
                std::cerr << "Échec du chargement de la texture PPM : " << texturePath << std::endl;
            }
        }
    }

    if (hasCurrent && !current.name.empty())
        m_materials[current.name] = current;

    return true;
}


bool OBJParser::tryGetActiveDiffuse(math::Vec3& outKd) const
{
    const std::string& name = !m_activeMaterial.empty() ? m_activeMaterial : m_firstUsedMaterial;
    if (name.empty())
        return false;

    std::unordered_map<std::string, MTLMaterial>::const_iterator it = m_materials.find(name);
    if (it == m_materials.end())
        return false;
    outKd = it->second.Kd;
    return true;
}

const MTLMaterial* OBJParser::getResolvedActiveMaterial() const
{
    const std::string& name = !m_activeMaterial.empty() ? m_activeMaterial : m_firstUsedMaterial;
    if (name.empty())
        return NULL;
    std::unordered_map<std::string, MTLMaterial>::const_iterator it = m_materials.find(name);
    if (it == m_materials.end())
        return NULL;
    return &it->second;
}

// Convertit un index OBJ (1-based, négatif pour relatif) en index C++ (0-based)
int OBJParser::fixIndex(int idx, int size) const {
    if (idx > 0) //si positif
        return idx - 1;
    if (idx < 0) //si négatif alors par rapport à la fin (a l'envers)
        return size + idx;
    return -1;
}

// Analyse un token de face (ex: "1/2/3") et retourne un ObjIndex
OBJParser::ObjIndex OBJParser::parseFaceToken(const std::string& token) const {
    ObjIndex idx{ -1, -1, -1 };

    // Trouver les positions des '/'
    size_t p1 = token.find('/');
    size_t p2 = token.find('/', p1 + 1); // chercher le second '/'

    //si pas de '/', alors c'est juste la position
    if (p1 == std::string::npos) {
        idx.v = std::stoi(token);
        return idx;
    }

    // Extraire les indices
    idx.v = std::stoi(token.substr(0, p1));

    //Rappel : 
    //  vt = texture coord, 
    //  vn = normal, 
    //  v  = vertex position

    // Si pas de second '/', alors c'est position/uv
    if (p2 == std::string::npos) {
        idx.vt = std::stoi(token.substr(p1 + 1));
    } else {
        // position/uv/normal
        if (p2 > p1 + 1)
            idx.vt = std::stoi(token.substr(p1 + 1, p2 - p1 - 1));
        if (p2 + 1 < token.size())
            idx.vn = std::stoi(token.substr(p2 + 1));
    }

    return idx;
}

// Récupère l'index du vertex correspondant à ObjIndex, ou le crée s'il n'existe pas
uint32_t OBJParser::getOrCreateVertex(const ObjIndex& idx) {
    static std::unordered_map<ObjIndex, uint32_t, ObjIndexHash> indexMap;

    auto it = indexMap.find(idx);
    if (it != indexMap.end())
        return it->second;

    Vertex v{};
    v.position = m_positions[idx.v];
    v.normal   = (idx.vn >= 0) ? m_normals[idx.vn] : math::Vec3{0, 0, 0};
    v.uv       = (idx.vt >= 0) ? m_uvs[idx.vt]     : math::Vec2{0, 0};

    uint32_t newIndex = static_cast<uint32_t>(m_vertices.size());
    m_vertices.push_back(v);
    indexMap[idx] = newIndex;

    return newIndex;
}

// Charge un fichier .obj et remplit les données de vertices et indices
bool OBJParser::loadFromFile(const std::string& filepath) {
    clear();

    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << "OBJParser: impossible d'ouvrir " << filepath << "\n";
        return false;
    }

    std::unordered_map<ObjIndex, uint32_t, ObjIndexHash> indexMap;
    std::string line;
	const std::string baseDir = directoryOf(filepath);

    while (std::getline(file, line)) {
        std::istringstream iss(line);
        std::string type;
        iss >> type;

        // TODO : 
        // 's' pour smooth shading groups
        // 'o' pour object name
        // 'g' pour group name
        // 'l' pour line (non supporté ici)
        // 'p' pour point (non supporté ici)
        // '#' pour commentaire

        // 'vt' pour texture coords
        // 'vn' pour vertex normals
        // 'v' pour vertex positions
        // 'f' pour faces
        // 'mtllib' pour fichier de matériaux
        // 'usemtl' pour utiliser un matériau
        if (type == "v") {
            math::Vec3 v;
            iss >> v.x >> v.y >> v.z;
            m_positions.push_back(v);
            if (m_positions.size() == 1)
            {
                m_boundsMin = v;
                m_boundsMax = v;
            }
            else
            {
                if (v.x < m_boundsMin.x) m_boundsMin.x = v.x;
                if (v.y < m_boundsMin.y) m_boundsMin.y = v.y;
                if (v.z < m_boundsMin.z) m_boundsMin.z = v.z;
                if (v.x > m_boundsMax.x) m_boundsMax.x = v.x;
                if (v.y > m_boundsMax.y) m_boundsMax.y = v.y;
                if (v.z > m_boundsMax.z) m_boundsMax.z = v.z;
            }
        }
        else if (type == "vn") {
            math::Vec3 n;
            iss >> n.x >> n.y >> n.z;
            m_normals.push_back(n);
        }
        else if (type == "vt") {
            math::Vec2 uv;
            iss >> uv.x >> uv.y;
            m_uvs.push_back(uv);
			m_hasUVs = true;
        }
        else if (type == "mtllib") {
            std::string mtlFile;
            getline(iss, mtlFile);
            mtlFile = ltrim(mtlFile);
            if (!mtlFile.empty())
                loadMtlFromFile(baseDir + "/" + mtlFile);
        }
        else if (type == "usemtl") {
            iss >> m_activeMaterial;
            if (m_firstUsedMaterial.empty())
                m_firstUsedMaterial = m_activeMaterial;
        }
        else if (type == "f") {
            std::vector<uint32_t> faceIndices;
            std::string token;

            while (iss >> token) {
                ObjIndex idx = parseFaceToken(token);

                idx.v  = fixIndex(idx.v,  (int)m_positions.size());
                idx.vt = fixIndex(idx.vt, (int)m_uvs.size());
                idx.vn = fixIndex(idx.vn, (int)m_normals.size());

                auto it = indexMap.find(idx);
                uint32_t vertIndex;

                if (it != indexMap.end()) {
                    vertIndex = it->second;
                } else {
                    Vertex v{};
                    v.position = m_positions[idx.v];
                    v.normal   = (idx.vn >= 0) ? m_normals[idx.vn] : math::Vec3{0,0,0};
                    v.uv       = (idx.vt >= 0) ? m_uvs[idx.vt]     : math::Vec2{0,0};

                    vertIndex = (uint32_t)m_vertices.size();
                    m_vertices.push_back(v);
                    indexMap[idx] = vertIndex;
                }

                faceIndices.push_back(vertIndex);
            }

            // Triangulation fan
            for (size_t i = 1; i + 1 < faceIndices.size(); ++i) {
                m_indices.push_back(faceIndices[0]);
                m_indices.push_back(faceIndices[i]);
                m_indices.push_back(faceIndices[i + 1]);
            }
        }
    }

    //debug
    std::cout << "Loaded OBJ: " << filepath << "\n";
    std::cout << "Positions: " << m_positions.size() << ", Normals: " << m_normals.size() << ", UVs: " << m_uvs.size() << "\n";
    std::cout << "Final Vertices: " << m_vertices.size() << ", Indices: " << m_indices.size() << "\n";
	if (!m_materials.empty())
		std::cout << "Materials: " << m_materials.size() << ", active: " << (!m_activeMaterial.empty() ? m_activeMaterial : m_firstUsedMaterial) << "\n";

    return true;
}
