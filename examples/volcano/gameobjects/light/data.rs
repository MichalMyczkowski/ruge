pub struct LightColor {
    pub ambient: glm::Vec4,
    pub diffuse: glm::Vec4,
    pub specular: glm::Vec4,
}

impl LightColor {
    pub fn new(ambient: glm::Vec3, diffuse: glm::Vec3, specular: glm::Vec3) -> Self {
        Self {
            ambient: glm::vec3_to_vec4(&ambient),
            diffuse: glm::vec3_to_vec4(&diffuse),
            specular: glm::vec3_to_vec4(&specular),
        }
    }
    pub fn as_vec(&self) -> Vec<f32> {
        self.ambient
            .iter()
            .chain(self.diffuse.iter())
            .chain(self.specular.iter())
            .map(|x| *x)
            .collect::<Vec<f32>>()
    }
}

pub struct DirectionalLight {
    pub direction: glm::Vec4,
    pub color: LightColor,
}

impl DirectionalLight {
    pub fn new(direction: glm::Vec3, color: LightColor) -> Self {
       Self {
           direction: glm::vec3_to_vec4(&direction),
           color,
       } 
    }
    pub fn as_vec(&self) -> Vec<f32> {
       self.direction.iter().map(|x| *x).chain(self.color.as_vec()).collect::<Vec<f32>>()
    }
}

pub struct PointLight {
    pub position: glm::Vec4,
    pub color: LightColor,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    _padding: f32,
}

impl PointLight {
    pub fn new(position: glm::Vec3, color: LightColor, constant: f32, linear: f32, quadratic: f32) -> Self {
        Self {
            position: glm::vec3_to_vec4(&position),
            color,
            constant,
            linear,
            quadratic,
            _padding: 0.0,
        } 
    }
    pub fn as_vec(&self) -> Vec<f32> {
       self.position.iter().map(|x| *x)
           .chain(self.color.as_vec())
           .chain(vec![self.constant, self.linear, self.quadratic, self._padding])
           .collect::<Vec<f32>>()
    }
}

pub struct SpotLight {
    pub position: glm::Vec4,
    pub direction: glm::Vec4,
    pub color: LightColor,
    pub cutoff: f32,
    pub outer_cutoff: f32,
    _padding2: f32,
    _padding3: f32,
}

impl SpotLight {
    pub fn new(position: glm::Vec3, direction: glm::Vec3, color: LightColor, cutoff: f32, outer_cutoff: f32) -> Self {
       Self {
           position: glm::vec3_to_vec4(&position),
           direction: glm::vec3_to_vec4(&direction),
           color,
           cutoff,
           outer_cutoff,
           _padding2: 0.0,
           _padding3: 0.0,
       } 
    }
    pub fn as_vec(&self) -> Vec<f32> {
       self.position.iter().map(|x| *x)
           .chain(self.direction.iter().map(|x| *x))
           .chain(self.color.as_vec())
           .chain(vec![self.cutoff, self.outer_cutoff, self._padding2, self._padding3])
           .collect::<Vec<f32>>()
    }
}
