use bevy::prelude::*;
use bevy_hanabi::{
    AccelModifier, Attribute, ColorBlendMask, ColorBlendMode, ColorOverLifetimeModifier,
    EffectAsset, Gradient, LinearDragModifier, Module, SetAttributeModifier,
    SetPositionCircleModifier, SetVelocityCircleModifier, ShapeDimension, SpawnerSettings,
};

/// Holds pre-built handles for all particle effects in the game.
/// Populated at startup; clone a handle to spawn an effect.
#[derive(Resource)]
pub struct ParticleEffects {
    pub flap_trail: Handle<EffectAsset>,
    pub collision_burst: Handle<EffectAsset>,
    pub pipe_sparkle: Handle<EffectAsset>,
}

/// Startup system — builds all EffectAsset graphs and stores handles.
pub fn setup_effects(mut effects: ResMut<Assets<EffectAsset>>, mut commands: Commands) {
    // ── 1. FLAP TRAIL ────────────────────────────────────────────────────────
    // Small white puffs emitted on every jump; fall away quickly.
    let flap_trail = {
        let mut module = Module::default();

        let init_pos = SetPositionCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::Z),
            radius: module.lit(4.0_f32),
            dimension: ShapeDimension::Surface,
        };
        let init_vel = SetVelocityCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::Z),
            speed: module.lit(25.0_f32),
        };
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(0.35_f32));
        let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(3.0_f32));
        let drag = LinearDragModifier::new(module.lit(6.0_f32));

        let mut gradient: Gradient<Vec4> = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.8));
        gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

        let asset = EffectAsset::new(64, SpawnerSettings::burst(12.0.into(), 0.0.into()), module)
            .with_name("flap_trail")
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .init(init_size)
            .update(drag)
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::RGBA,
            });

        effects.add(asset)
    };

    // ── 2. COLLISION BURST ───────────────────────────────────────────────────
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

        let mut gradient: Gradient<Vec4> = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.5, 0.1, 1.0)); // orange
        gradient.add_key(0.5, Vec4::new(0.9, 0.2, 0.1, 0.8)); // red-orange
        gradient.add_key(1.0, Vec4::new(0.5, 0.1, 0.1, 0.0)); // fade

        let asset = EffectAsset::new(128, SpawnerSettings::burst(40.0.into(), 0.0.into()), module)
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

    // ── 3. PIPE SPARKLE ──────────────────────────────────────────────────────
    // Brief yellow sparkle confirming a successful pipe pass.
    let pipe_sparkle = {
        let mut module = Module::default();

        let init_pos = SetPositionCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::Z),
            radius: module.lit(8.0_f32),
            dimension: ShapeDimension::Surface,
        };
        let init_vel = SetVelocityCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::Z),
            speed: module.lit(50.0_f32),
        };
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(0.4_f32));
        let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(4.0_f32));
        let drag = LinearDragModifier::new(module.lit(8.0_f32));

        let mut gradient: Gradient<Vec4> = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.3, 1.0)); // bright yellow
        gradient.add_key(0.5, Vec4::new(1.0, 0.8, 0.1, 0.7));
        gradient.add_key(1.0, Vec4::new(1.0, 0.6, 0.0, 0.0)); // fade to orange

        let asset = EffectAsset::new(64, SpawnerSettings::burst(16.0.into(), 0.0.into()), module)
            .with_name("pipe_sparkle")
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .init(init_size)
            .update(drag)
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::RGBA,
            });

        effects.add(asset)
    };

    commands.insert_resource(ParticleEffects { flap_trail, collision_burst, pipe_sparkle });
}
