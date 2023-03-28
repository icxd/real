#![feature(extend_one)]



#[derive(Debug, Clone)] pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Constant>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}
impl ClassFile {
    pub fn add_method_ref(&mut self, class: u16, name_and_type: u16) -> u16 {
        let index = self.constant_pool.len() as u16 + 1;

        self.constant_pool.push(Constant {
            tag: 10,
            info: ConstantInfo::MethodRef(class, name_and_type),
        });

        index
    }
    pub fn add_class(&mut self, name: u16) -> u16 {
        let index = self.constant_pool.len() as u16 + 1;

        self.constant_pool.push(Constant {
            tag: 7,
            info: ConstantInfo::Class(name),
        });

        index
    }
    pub fn add_name_and_type(&mut self, name: u16, descriptor: u16) -> u16 {
        let index = self.constant_pool.len() as u16 + 1;

        self.constant_pool.push(Constant {
            tag: 12,
            info: ConstantInfo::NameAndType(name, descriptor),
        });

        index
    }
    pub fn add_utf8(&mut self, value: &str) -> u16 {
        let index = self.constant_pool.len() as u16 + 1;

        self.constant_pool.push(Constant {
            tag: 1,
            info: ConstantInfo::Utf8(value.to_string()),
        });

        index
    }

    pub fn add_field_ref(&mut self, class: u16, name_and_type: u16) -> u16 {
        let index = self.constant_pool.len() as u16 + 1;

        self.constant_pool.push(Constant {
            tag: 9,
            info: ConstantInfo::FieldRef(class, name_and_type),
        });

        index
    }

    pub fn add_string(&mut self, string: u16) -> u16 {
        let index = self.constant_pool.len() as u16 + 1;

        self.constant_pool.push(Constant {
            tag: 8,
            info: ConstantInfo::String(string),
        });

        index
    }

    pub fn add_access_flags(&mut self, flags: u16) {
        self.access_flags |= flags;
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend_one(&((self.magic >> 24) as u8));
        bytes.extend_one(&((self.magic >> 16) as u8));
        bytes.extend_one(&((self.magic >> 8) as u8));
        bytes.extend_one(&((self.magic & 0xff) as u8));

        bytes.extend_one(&((self.minor_version >> 8) as u8));
        bytes.extend_one(&((self.minor_version & 0xff) as u8));

        bytes.extend_one(&((self.major_version >> 8) as u8));
        bytes.extend_one(&((self.major_version & 0xff) as u8));

        bytes.extend_one(&((self.constant_pool.len() + 1 >> 8) as u8));
        bytes.extend_one(&((self.constant_pool.len() + 1 & 0xff) as u8));

        for constant in self.constant_pool.clone() {
            // bytes.extend_one(&((constant.tag & 0xff) as u8));
            bytes.extend(constant.to_bytes());
        }

        bytes.extend_one(&((self.access_flags >> 8) as u8));
        bytes.extend_one(&((self.access_flags & 0xff) as u8));

        bytes.extend_one(&((self.this_class >> 8) as u8));
        bytes.extend_one(&((self.this_class & 0xff) as u8));

        bytes.extend_one(&((self.super_class >> 8) as u8));
        bytes.extend_one(&((self.super_class & 0xff) as u8));

        bytes.extend_one(&((self.interfaces.len() >> 8) as u8));
        bytes.extend_one(&((self.interfaces.len() & 0xff) as u8));

        for interface in self.interfaces.clone() {
            bytes.extend_one(&((interface >> 8) as u8));
            bytes.extend_one(&((interface & 0xff) as u8));
        }

        bytes.extend_one(&((self.fields.len() >> 8) as u8));
        bytes.extend_one(&((self.fields.len() & 0xff) as u8));

        for field in self.fields.clone() {
            bytes.extend(field.to_bytes());
        }

        bytes.extend_one(&((self.methods.len() >> 8) as u8));
        bytes.extend_one(&((self.methods.len() & 0xff) as u8));

        for method in self.methods.clone() {
            bytes.extend(method.to_bytes());
        }

        bytes.extend_one(&((self.attributes.len() >> 8) as u8));
        bytes.extend_one(&((self.attributes.len() & 0xff) as u8));

        for attribute in self.attributes.clone() {
            bytes.extend(attribute.to_bytes());
        }

        bytes
    }
}
#[derive(Debug, Clone)] pub struct Constant {
    pub tag: u8,
    pub info: ConstantInfo,
}
impl Constant {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.tag];
        
        match self.info {
            ConstantInfo::MethodRef(ref class, ref name_and_type) => {
                bytes.extend_one(&((class >> 8) as u8));
                bytes.extend_one(&((class & 0xff) as u8));
                bytes.extend_one(&((name_and_type >> 8) as u8));
                bytes.extend_one(&((name_and_type & 0xff) as u8));
            },
            ConstantInfo::Class(ref name) => {
                bytes.extend_one(&((name >> 8) as u8));
                bytes.extend_one(&((name & 0xff) as u8));
            },
            ConstantInfo::NameAndType(ref name, ref descriptor) => {
                bytes.extend_one(&((name >> 8) as u8));
                bytes.extend_one(&((name & 0xff) as u8));
                bytes.extend_one(&((descriptor >> 8) as u8));
                bytes.extend_one(&((descriptor & 0xff) as u8));
            },
            ConstantInfo::Utf8(ref value) => {
                bytes.extend_one(&((value.len() >> 8) as u8));
                bytes.extend_one(&((value.len() & 0xff) as u8));
                bytes.extend(value.as_bytes());
            },
            ConstantInfo::FieldRef(ref class, ref name_and_type) => {
                bytes.extend_one(&((class >> 8) as u8));
                bytes.extend_one(&((class & 0xff) as u8));
                bytes.extend_one(&((name_and_type >> 8) as u8));
                bytes.extend_one(&((name_and_type & 0xff) as u8));
            },
            ConstantInfo::String(ref string) => {
                bytes.extend_one(&((string >> 8) as u8));
                bytes.extend_one(&((string & 0xff) as u8));
            },
            _ => unimplemented!("ConstantInfo::to_bytes({:?})", self.info)
        }

        bytes
    }
}
#[derive(Debug, Clone)] pub enum ConstantInfo {
    Utf8(String),
    Class(u16),
    NameAndType(u16, u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    String(u16),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    MethodHandle(u8, u16),
    MethodType(u16),
    InvokeDynamic(u16, u16),
}
#[derive(Debug, Clone)] pub struct Field {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}
impl Field {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_one(&((self.access_flags >> 8) as u8));
        bytes.extend_one(&((self.access_flags & 0xff) as u8));
        bytes.extend_one(&((self.name_index >> 8) as u8));
        bytes.extend_one(&((self.name_index & 0xff) as u8));
        bytes.extend_one(&((self.descriptor_index >> 8) as u8));
        bytes.extend_one(&((self.descriptor_index & 0xff) as u8));
        bytes.extend_one(&((self.attributes.len() >> 8) as u8));
        bytes.extend_one(&((self.attributes.len() & 0xff) as u8));
        for attribute in &self.attributes {
            bytes.extend(attribute.to_bytes());
        }
        
        bytes
    }
}
#[derive(Debug, Clone)] pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}
impl Method {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_one(&((self.access_flags >> 8) as u8));
        bytes.extend_one(&((self.access_flags & 0xff) as u8));
        bytes.extend_one(&((self.name_index >> 8) as u8));
        bytes.extend_one(&((self.name_index & 0xff) as u8));
        bytes.extend_one(&((self.descriptor_index >> 8) as u8));
        bytes.extend_one(&((self.descriptor_index & 0xff) as u8));
        bytes.extend_one(&((self.attributes.len() >> 8) as u8));
        bytes.extend_one(&((self.attributes.len() & 0xff) as u8));
        for attribute in &self.attributes {
            bytes.extend(attribute.to_bytes());
        }
        
        bytes
    }
}
#[derive(Debug, Clone)] pub struct Attribute {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
}
impl Attribute {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_one(&((self.attribute_name_index >> 8) as u8));
        bytes.extend_one(&((self.attribute_name_index & 0xff) as u8));
        bytes.extend_one(&((self.info.len() >> 24) as u8));
        bytes.extend_one(&((self.info.len() >> 16) as u8));
        bytes.extend_one(&((self.info.len() >> 8) as u8));
        bytes.extend_one(&((self.info.len() & 0xff) as u8));
        bytes.extend(self.info.clone());
        
        bytes
    }
}