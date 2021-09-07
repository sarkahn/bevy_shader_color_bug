use bevy::{prelude::*, reflect::TypeUuid, render::{pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, base}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};

const MAT_COLOR_VERT: &str = include_str!("mat_color.vert"); 
const MAT_COLOR_FRAG: &str = include_str!("mat_color.frag"); 


const VERT_COLOR_VERT: &str = include_str!("vert_color.vert"); 
const VERT_COLOR_FRAG: &str = include_str!("vert_color.frag"); 

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08161c-0b5a-417e-1bce-35233b25127e"]
struct CustomMaterial {
    color: Color,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let size = 5.0;

    // Create a new shader pipeline
    let mat_color_pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, MAT_COLOR_VERT)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, MAT_COLOR_FRAG))),
    }));
    
    render_graph.add_system_node("custom_material", AssetRenderResourcesNode::<CustomMaterial>::new(true));
    render_graph.add_node_edge("custom_material", base::node::MAIN_PASS).unwrap();

    let vert_color_pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERT_COLOR_VERT)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, VERT_COLOR_FRAG))),
    }));
    
    let col = Color::rgb(1.0, 0.25, 0.0);
    let col_vec = vec![[col.r(), col.g(), col.b()]; 4];

    let color_mat = materials.add(ColorMaterial::color(col));

    let custom_color_material = custom_materials.add(CustomMaterial {
        color: col
    });

    let mut mesh = Mesh::from(shape::Quad { size: Vec2::ONE * size, flip: false});
    // Set vertex colors in base mesh - they only get used in vert_color shaders
    mesh.set_attribute("Vertex_Color", col_vec);
    let quad_mesh = meshes.add(mesh);

    // Sprite
    commands.spawn_bundle(SpriteBundle {
        material: color_mat,
        sprite: Sprite {
            size: Vec2::ONE * size,
            resize_mode: SpriteResizeMode::Manual,
            ..Default::default()
        },
        ..Default::default()
    });

    // Custom color material
    commands.spawn_bundle(MeshBundle {
        mesh: quad_mesh.clone(),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(mat_color_pipeline)]),
        transform: Transform::from_xyz(size + 1.0, 0.0, 0.0),
        ..Default::default()
    }).insert(custom_color_material);

    // Vertex colors
    commands.spawn_bundle(MeshBundle {
        mesh: quad_mesh.clone(),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(vert_color_pipeline)]),
        transform: Transform::from_xyz(size * 2.0 + 2.0, 0.0, 0.0),
        ..Default::default()
    });

    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scale = 1.0 / 15.0;
    commands.spawn_bundle(cam);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_asset::<CustomMaterial>()
        .add_startup_system(setup.system())
        .run();
}
