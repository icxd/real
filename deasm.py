from typing import List, Tuple
from rich import print
import io
from sys import argv

CONSTANT_Class              = 7
CONSTANT_Fieldref           = 9
CONSTANT_Methodref          = 10
CONSTANT_InterfaceMethodref = 11
CONSTANT_String             = 8
CONSTANT_Integer            = 3
CONSTANT_Float              = 4
CONSTANT_Long               = 5
CONSTANT_Double             = 6
CONSTANT_NameAndType        = 12
CONSTANT_Utf8               = 1
CONSTANT_MethodHandle       = 15
CONSTANT_MethodType         = 16
CONSTANT_InvokeDynamic      = 18

class_access_flags = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_FINAL", 0x0010),
    ("ACC_SUPER", 0x0020),
    ("ACC_INTERFACE", 0x0200),
    ("ACC_ABSTRACT", 0x0400),
    ("ACC_SYNTHETIC", 0x1000),
    ("ACC_ANNOTATION", 0x2000),
    ("ACC_ENUM", 0x4000),
]

method_access_flags = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_PRIVATE", 0x0002),
    ("ACC_PROTECTED", 0x0004),
    ("ACC_STATIC", 0x0008),
    ("ACC_FINAL", 0x0010),
    ("ACC_SYNCHRONIZED", 0x0020),
    ("ACC_BRIDGE", 0x0040),
    ("ACC_VARARGS", 0x0080),
    ("ACC_NATIVE", 0x0100),
    ("ACC_ABSTRACT", 0x0400),
    ("ACC_STRICT", 0x0800),
    ("ACC_SYNTHETIC", 0x1000),
]

field_access_flags = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_PRIVATE", 0x0002),
    ("ACC_PROTECTED", 0x0004),
    ("ACC_STATIC", 0x0008),
    ("ACC_FINAL", 0x0010),
    ("ACC_VOLATILE", 0x0040),
    ("ACC_TRANSIENT", 0x0080),
    ("ACC_SYNTHETIC", 0x1000),
    ("ACC_ENUM", 0x4000),
]

def parse_flags(value: int, flags: List[Tuple[str, int]]) -> List[str]:
    return [name for (name, mask) in flags if (value&mask) != 0]

def parse_attributes(f, attributes_count: int) -> list:
    attributes = []
    for i in range(attributes_count):
        attribute = {}
        attribute['attribute_name_index'] = parse_u2(f)
        attribute_length = parse_u4(f)
        attribute['info'] = f.read(attribute_length)
        attributes.append(attribute)
    return attributes

def parse_u1(f) -> int: return int.from_bytes(f.read(1), 'big')
def parse_u2(f) -> int: return int.from_bytes(f.read(2), 'big')
def parse_u4(f) -> int: return int.from_bytes(f.read(4), 'big')

def parse_class_file(file_path: str):
    with open(file_path, 'rb') as f:
        clazz = {}
        clazz['magic'] = hex(parse_u4(f))
        clazz['minor'] = parse_u2(f)
        clazz['major'] = parse_u2(f)
        constant_pool_count = parse_u2(f)
        constant_pool = []
        for i in range(constant_pool_count - 1):
            cp_info = {}
            tag = parse_u1(f)
            if tag == CONSTANT_Methodref:
                cp_info['tag'] = 'CONSTANT_Methodref'
                cp_info['class_index'] = parse_u2(f)
                cp_info['name_and_type_index'] = parse_u2(f)
            elif tag == CONSTANT_Class:
                cp_info['tag'] = 'CONSTANT_Class'
                cp_info['name_index'] = parse_u2(f)
            elif tag == CONSTANT_NameAndType:
                cp_info['tag'] = 'CONSTANT_NameAndType'
                cp_info['name_index'] = parse_u2(f)
                cp_info['descriptor_index'] = parse_u2(f)
            elif tag == CONSTANT_Utf8:
                cp_info['tag'] = 'CONSTANT_Utf8'
                length = parse_u2(f)
                cp_info['bytes'] = f.read(length)
            elif tag == CONSTANT_Fieldref:
                cp_info['tag'] = 'CONSTANT_Fieldref'
                cp_info['class_index'] = parse_u2(f)
                cp_info['name_and_type_index'] = parse_u2(f)
            elif tag == CONSTANT_String:
                cp_info['tag'] = 'CONSTANT_String'
                cp_info['string_index'] = parse_u2(f)
            elif tag == CONSTANT_Integer:
                cp_info['tag'] = 'CONSTANT_Integer'
                cp_info['bytes'] = parse_u4(f)
            else:
                assert False, f"Unexpected tag {tag}"
            constant_pool.append(cp_info)

        clazz['constant_pool'] = constant_pool
        clazz['access_flags'] = parse_flags(parse_u2(f), class_access_flags)
        clazz['this_class'] = parse_u2(f)
        clazz['super_class'] = parse_u2(f)
        interfaces_count = parse_u2(f)
        interfaces = []
        for i in range(interfaces_count):
            assert False, "Parsing interfaces is not implemented"
        clazz['interfaces'] = interfaces
        fields_count = parse_u2(f)
        fields = []
        for i in range(fields_count):
            field = {}
            field['access_flags'] = parse_flags(parse_u2(f), field_access_flags)
            field['name_index'] = parse_u2(f)
            field['descriptor'] = parse_u2(f)
            attributes_count = parse_u2(f)
            field['attributes'] = parse_attributes(f, attributes_count)
            fields.append(field)
        clazz['fields'] = fields
        methods_count = parse_u2(f)
        methods = []
        for i in range(methods_count):
            method = {}
            method['access_flags'] = parse_flags(parse_u2(f), method_access_flags)
            method['name_index'] = parse_u2(f)
            method['descriptor'] = parse_u2(f)
            attributes_count = parse_u2(f)
            method['attributes'] = parse_attributes(f, attributes_count)
            methods.append(method)
        clazz['methods'] = methods
        attributes_count = parse_u2(f)
        clazz['attributes'] = parse_attributes(f, attributes_count)

        return clazz
    
def find_methods_by_name(clazz, name: bytes):
    return [method for method in clazz['methods'] if clazz['constant_pool'][method['name_index'] - 1]['bytes'] == name]

def find_attribute_by_name(clazz, attributes, name: bytes):
    return [attr for attr in attributes if clazz['constant_pool'][attr['attribute_name_index'] - 1]['bytes'] == name]

def parse_code_info(info: bytes):
    code = {}
    with io.BytesIO(info) as f:
        code['max_stack'] = parse_u2(f)
        code['max_locals'] = parse_u2(f)
        code_length = parse_u4(f)
        code['code'] = f.read(code_length)
        exception_table_length = parse_u2(f)
        for i in range(exception_table_length):
            assert False, "Parsing expection table is not implemented"
        attributes_count = parse_u2(f)
        code['attributes'] = parse_attributes(f, attributes_count)
    return code

getstatic_opcode = 0xb2
idc_opcode = 0x12
invokevirtual_opcode = 0xb6
return_opcode = 0xb1
bipush_opcode = 0x10

def name_of_class(clazz, index):
    return clazz['constant_pool'][clazz['constant_pool'][index - 1]['name_index'] - 1]['bytes']

def execute_code(clazz, code: bytes):
    stack = []
    with io.BytesIO(code) as f:
        while f.tell() < len(code):
            opcode = parse_u1(f)
            if opcode == getstatic_opcode:
                index = parse_u2(f)
                fieldref = clazz['constant_pool'][index - 1]
                name = name_of_class(clazz, fieldref['class_index'])
                member = name_of_class(clazz, fieldref['name_and_type_index'])
                print(f"getstatic {index}")
            elif opcode == idc_opcode:
                index = parse_u1(f)
                print(f"ldc {index}")
            elif opcode == invokevirtual_opcode:
                index = parse_u2(f)
                methodref = clazz['constant_pool'][index - 1]
                name = name_of_class(clazz, methodref['class_index'])
                member = name_of_class(clazz, methodref['name_and_type_index'])
                print(f"invokevirtual {index}")
            elif opcode == return_opcode:
                return
            elif opcode == bipush_opcode:
                byte = parse_u1(f)
                print(f"bipush {byte}")
            else:
                assert False, f"unknown opcode {hex(opcode)}"
            print(f" => {stack}")

clazz = parse_class_file(argv[1])
print(clazz)

for field in clazz['fields']:
    f: str = ""
    for flag in field['access_flags']:
        f += flag.replace("ACC_", "").lower() + " "
    t = clazz['constant_pool'][field['descriptor'] - 1]['bytes'].decode('utf-8').replace("/", ".")
    if t.endswith(";"): t = t[:-1]
    if t.startswith("["): t = t[1:] + "[]"
    if t.startswith("L"): t = t[1:]
    if t.endswith(";"): t = t[:-1]
    if t == "Z": t = "boolean"
    elif t == "C": t = "char"
    elif t == "B": t = "byte"
    elif t == "S": t = "short"
    elif t == "I": t = "int"
    elif t == "J": t = "long"
    elif t == "F": t = "float"
    elif t == "D": t = "double"
    f += t
    f += " "
    f += clazz['constant_pool'][field['name_index'] - 1]['bytes'].decode('utf-8')
    
    print(f"{f}")