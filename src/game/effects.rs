use bevy::prelude::*;
use bevy_hanabi::{
    AccelModifier, Attribute, ColorBlendMask, ColorBlendMode, ColorOverLifetimeModifier,
    EffectAsset, Gradient, Module, SetAttributeModifier, SetPositionCircleModifier,
    SetVelocityCircleModifier, ShapeDimension, SpawnerSettings,
};

/// Holds pre-built handles for all particle effects in the game.
#[derive(Resource)]
pub struct ParticleEffects {
    pub collision_burst: Handle<EffectAsset>,
}

/// Startup system — builds all EffectAsset graphs and stores handles.
pub fn setup_effects(mut effects: ResMut<Assets<EffectAsset>>, mut commands: Commands) {
    // ── COLLISION BURST ──────────────────────────────────────────────────────
    // Radial explosion of orange/red "feather" particles on death.
    let collision_burst = {
        let mut module = Module::default();

        let init_pos = SetPositionCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::Z),
            radius: module.lit(2.0_f32),
            dimension: ShapeDimension::Surface,
        };
        let init_vel = SetVelocityCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::Z),
            speed: module.lit(120.0_f32),
        };
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(0.7_f32));
        let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(5.0_f32));
        let gravity = AccelModifier::new(module.lit(Vec3::new(0.0, -200.0, 0.0)));

        let mut gradient = Gradient::<Vec4>::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.5, 0.1, 1.0)); // orange
        gradient.add_key(0.5, Vec4::new(0.9, 0.2, 0.1, 0.8)); // red-orange
        gradient.add_key(1.0, Vec4::new(0.5, 0.1, 0.1, 0.0)); // fade

        let asset = EffectAsset::new(128, SpawnerSettings::once(40.0.into()), module)
            .with_name("collision_burst")
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .init(init_size)
            .update(gravity)
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::RGBA,
            });

        effects.add(asset)
    };

    commands.insert_resource(ParticleEffects { collision_burst });
}
