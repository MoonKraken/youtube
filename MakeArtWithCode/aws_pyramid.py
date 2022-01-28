import bpy
import math
import os
import random
import bmesh

random.seed(6)
PYRAMID_BASE_WIDTH = 13
# change this to the dir you'd like to pull AWS Icons from
icon_dir_name='/Users/kenk/Documents/Repositories/art/AWSPyramid/AWSServiceIcons'

bpy.ops.object.mode_set(mode='OBJECT')

def make_mats(directory):
    mats = []
    for filename in os.listdir(icon_dir_name):
        if filename == '.DS_Store':
            continue
        
        mat = bpy.data.materials.new(f'{filename}')
        # set up nodes
        mat.use_nodes = True
            
        nodes = mat.node_tree.nodes
        
        for node in nodes:
            nodes.remove(node)
        
        output = nodes.new(type='ShaderNodeOutputMaterial')
        
        bsdf = nodes.new(type='ShaderNodeBsdfPrincipled')
        bsdf.inputs['Specular'].default_value = 0.5
        bsdf.inputs['Roughness'].default_value = 0.5
        bsdf.inputs['IOR'].default_value = 1.450
        bsdf.inputs['Emission Strength'].default_value = 3.8
        
        image_node = nodes.new(type='ShaderNodeTexImage')
        filepath = f'{directory}/{filename}'
        bpy.ops.image.open(filepath=filepath)
        
        image_node.image = bpy.data.images[f'{filename}']
        
        uv = nodes.new(type='ShaderNodeUVMap')
        
        node_tree = mat.node_tree
        
        # UV Map -> Image
        node_tree.links.new(uv.outputs[0], image_node.inputs[0])
        
        # Image -> Base color
        node_tree.links.new(image_node.outputs[0], bsdf.inputs['Base Color'])
        
        # Image -> Emission color
        node_tree.links.new(image_node.outputs[0], bsdf.inputs['Emission'])
        
        # BSDF -> Output
        node_tree.links.new(bsdf.outputs[0], output.inputs[0])
        
        mats.append(mat)
        
    return mats

def make_floor_mat():
    floor_mat = bpy.data.materials.new('floor')
    floor_mat.use_nodes = True
    nodes = floor_mat.node_tree.nodes
    for node in nodes:
        nodes.remove(node)
        
    output = nodes.new(type='ShaderNodeOutputMaterial')
        
    bsdf = nodes.new(type='ShaderNodeBsdfPrincipled')
    bsdf.inputs['Transmission'].default_value = 0.8
    bsdf.inputs['Transmission Roughness'].default_value = 0.123
    bsdf.inputs['IOR'].default_value = 1.45
    bsdf.inputs['Base Color'].default_value = [0,0,0,1]
    
    # BSDF -> Output
    floor_mat.node_tree.links.new(bsdf.outputs[0], output.inputs[0])
    
    return floor_mat

# Pyramid
def build_pyramid(center_x, center_y, center_z, center_blocks):
    if center_blocks >= 1:
        build_level(center_x, center_y, center_z, center_blocks)
        build_pyramid(center_x, center_y, center_z + 1, center_blocks - 2)

def build_level(center_x, center_y, center_z, center_blocks):
    build_row(center_x, center_y, center_z, center_blocks)
    
    for i in range(1, int(math.ceil(center_blocks / 2.0))):
        row_size = center_blocks - (2*i)
        build_row(center_x, center_y + i, center_z, row_size)
        build_row(center_x, center_y - i, center_z, row_size)

def build_row(center_x, center_y, center_z, row_size):
    if row_size == 1:
        make_cube_with_material(1, center_x, center_y, center_z)
    elif row_size > 1:
        offset = int(math.ceil(row_size / 2.0)) - 1.0
        for x_val in [center_x+offset, center_x-offset]:
            make_cube_with_material(1, x_val, center_y, center_z)

def make_cube_with_material(size, x, y, z):
    bpy.ops.mesh.primitive_cube_add(size=size, enter_editmode=False, align='WORLD', location=(x, y, z))
    obj = bpy.context.active_object
    obj.data.materials.append(random.choice(mats))
    mesh = obj.data

    bpy.ops.object.mode_set(mode='EDIT')
    bpy.ops.uv.reset()

    bm = bmesh.from_edit_mesh(mesh)

    bm.edges.ensure_lookup_table()
    for edge in bm.edges:
        edge.seam = True

    bm.faces.ensure_lookup_table()
    for face in bm.faces:
        face.select_set(False)

    for face in bm.faces:
        face.material_index = 0
        face.select_set(True)
        bmesh.update_edit_mesh(mesh)
        bpy.ops.uv.smart_project(angle_limit=90.0, island_margin=0.001)
        #bpy.ops.uv.unwrap()
        face.select_set(False)

    bpy.ops.object.mode_set(mode='OBJECT')

# remove existing materials
for material in bpy.data.materials:
    material.user_clear()
    bpy.data.materials.remove(material)

# remove objects
removeThese = bpy.context.copy()
removeThese['selected_objects'] = list(bpy.context.scene.objects,)
bpy.ops.object.delete(removeThese)

# background
world = bpy.data.worlds['World']
world.use_nodes = True
bg = world.node_tree.nodes['Background']
bg.inputs[0].default_value[:3] = (0, 0, 0)
bg.inputs[1].default_value = 1.0

mats = make_mats(icon_dir_name)
build_pyramid(0,0,0.5,PYRAMID_BASE_WIDTH)

bpy.ops.object.camera_add(
    enter_editmode=False, 
    align='VIEW', 
    location=(9.8, 21.02, 5.12), 
    rotation=(math.radians(84), 0, math.radians(157)), 
    scale=(1, 1, 1)
)

bpy.context.object.data.lens = 35

# ground
bpy.ops.mesh.primitive_plane_add(size=100, location=(0,0,0))
bpy.context.active_object.data.materials.append(make_floor_mat())

# lights....

light_z = 15
light_distance = 20
light_power = 300
for x in [-1, 1]:
    for y in [-1, 1]:
        this_z_rotation = 135 * x * y if y > 0 else -45 * x * y
        bpy.ops.object.light_add(
            type='SPOT', 
            radius = 5, 
            align = 'WORLD', 
            location=(x * light_distance, y * light_distance, light_z),
            rotation=(math.radians(60), 0, math.radians(this_z_rotation))
        )
        bpy.context.object.data.energy = light_power